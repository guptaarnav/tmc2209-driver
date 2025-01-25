//! TMC2209 Driver: Three Separate Structs for the Three Modes or Operation
//!
//! 1. `Tmc2209StandaloneLegacy` – Option 1 (Legacy STEP/DIR driver, no UART)
//! 2. `Tmc2209StandaloneOtpPreconfig` – Option 2 (Standalone + OTP, same pins as Legacy)
//! 3. `Tmc2209FullUartDiagnosticsAndControl` – Option 3 (Full UART Diagnostics & Control)

use embedded_hal::digital::{InputPin, OutputPin};
use embedded_io::{ErrorType, Read, Write};

use crate::errors::TmcError; // e.g. PinError, SerialError, etc.
use crate::packet::{
    // for building / parsing TMC2209 frames
    build_read_packet,
    build_write_packet,
    calc_crc8,
};
use crate::registers::*; // TMC2209 register addresses & bit flags

// ---------------------------------------------------------------------------
// 1) Standalone Legacy (Option 1)
// ---------------------------------------------------------------------------

/// TMC2209 in "Standalone Legacy" mode.
/// No UART usage, pure step/dir. The driver is configured via pins (MS1, MS2, VREF).
/// Optional DIAG and INDEX pins can be read if provided.
pub struct Tmc2209StandaloneLegacy<EN, STEP, DIR, DIAG, INDEX>
where
    EN: OutputPin,
    STEP: OutputPin,
    DIR: OutputPin,
    DIAG: InputPin,
    INDEX: InputPin,
{
    en: EN,
    step: STEP,
    dir: DIR,
    diag: Option<DIAG>,
    index: Option<INDEX>,
}

impl<EN, STEP, DIR, DIAG, INDEX> Tmc2209StandaloneLegacy<EN, STEP, DIR, DIAG, INDEX>
where
    EN: OutputPin,
    STEP: OutputPin,
    DIR: OutputPin,
    DIAG: InputPin,
    INDEX: InputPin,
{
    /// Create a new Legacy mode driver with *only* EN, STEP, and DIR pins.
    pub fn new_basic(en: EN, step: STEP, dir: DIR) -> Self {
        Self {
            en,
            step,
            dir,
            diag: None,
            index: None,
        }
    }

    /// Create a new Legacy mode driver with optional DIAG and INDEX pins.
    pub fn new_with_options(
        en: EN,
        step: STEP,
        dir: DIR,
        diag: Option<DIAG>,
        index: Option<INDEX>,
    ) -> Self {
        Self {
            en,
            step,
            dir,
            diag,
            index,
        }
    }

    /// Enable the motor driver (active-low => EN pin LOW).
    pub fn enable(&mut self) -> Result<(), TmcError> {
        self.en.set_low().map_err(|_| TmcError::PinError)
    }

    /// Disable the motor driver (active-low => EN pin HIGH).
    pub fn disable(&mut self) -> Result<(), TmcError> {
        self.en.set_high().map_err(|_| TmcError::PinError)
    }

    /// Set direction. `true` => DIR pin HIGH.
    pub fn set_direction(&mut self, clockwise: bool) -> Result<(), TmcError> {
        if clockwise {
            self.dir.set_high().map_err(|_| TmcError::PinError)
        } else {
            self.dir.set_low().map_err(|_| TmcError::PinError)
        }
    }

    /// Step once by toggling STEP pin. (Blocking approach)
    pub fn step_pulse(&mut self) -> Result<(), TmcError> {
        self.step.set_high().map_err(|_| TmcError::PinError)?;
        // Possibly wait a few microseconds...
        self.step.set_low().map_err(|_| TmcError::PinError)
    }

    /// If DIAG pin is provided, read it. Returns `Ok(Some(true/false))` or `Ok(None)`.
    pub fn read_diag(&mut self) -> Result<Option<bool>, TmcError> {
        match &mut self.diag {
            Some(pin) => {
                let val = pin.is_high().map_err(|_| TmcError::PinError)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    /// If INDEX pin is provided, read it. Returns `Ok(Some(true/false))` or `Ok(None)`.
    pub fn read_index(&mut self) -> Result<Option<bool>, TmcError> {
        match &mut self.index {
            Some(pin) => {
                let val = pin.is_high().map_err(|_| TmcError::PinError)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }
}

// ---------------------------------------------------------------------------
// 2) Standalone OTP Preconfig (Option 2)
// ---------------------------------------------------------------------------

/// TMC2209 in "Standalone OTP Preconfig" mode.
/// Same pin usage as Legacy mode, but we assume the TMC2209 has been
/// pre-configured via OTP or CPU-based writes bit-banged to TMC2209 UART input (handled outside of this driver). No normal UART usage.
pub struct Tmc2209StandaloneOtpPreconfig<EN, STEP, DIR, DIAG, INDEX>
where
    EN: OutputPin,
    STEP: OutputPin,
    DIR: OutputPin,
    DIAG: InputPin,
    INDEX: InputPin,
{
    en: EN,
    step: STEP,
    dir: DIR,
    diag: Option<DIAG>,
    index: Option<INDEX>,
}

impl<EN, STEP, DIR, DIAG, INDEX> Tmc2209StandaloneOtpPreconfig<EN, STEP, DIR, DIAG, INDEX>
where
    EN: OutputPin,
    STEP: OutputPin,
    DIR: OutputPin,
    DIAG: InputPin,
    INDEX: InputPin,
{
    /// Create an OTP Preconfig driver with *only* EN, STEP, and DIR pins.
    pub fn new_basic(en: EN, step: STEP, dir: DIR) -> Self {
        Self {
            en,
            step,
            dir,
            diag: None,
            index: None,
        }
    }

    /// Create an OTP Preconfig driver with optional DIAG and INDEX pins.
    pub fn new_with_options(
        en: EN,
        step: STEP,
        dir: DIR,
        diag: Option<DIAG>,
        index: Option<INDEX>,
    ) -> Self {
        Self {
            en,
            step,
            dir,
            diag,
            index,
        }
    }

    /// Enable the motor driver.
    pub fn enable(&mut self) -> Result<(), TmcError> {
        self.en.set_low().map_err(|_| TmcError::PinError)
    }

    /// Disable the motor driver.
    pub fn disable(&mut self) -> Result<(), TmcError> {
        self.en.set_high().map_err(|_| TmcError::PinError)
    }

    /// Set direction.
    pub fn set_direction(&mut self, clockwise: bool) -> Result<(), TmcError> {
        if clockwise {
            self.dir.set_high().map_err(|_| TmcError::PinError)
        } else {
            self.dir.set_low().map_err(|_| TmcError::PinError)
        }
    }

    /// Step once by toggling STEP pin. (Blocking)
    pub fn step_pulse(&mut self) -> Result<(), TmcError> {
        self.step.set_high().map_err(|_| TmcError::PinError)?;
        // Possibly wait a few microseconds...
        self.step.set_low().map_err(|_| TmcError::PinError)
    }

    /// If DIAG pin is provided, read it.
    pub fn read_diag(&mut self) -> Result<Option<bool>, TmcError> {
        match &mut self.diag {
            Some(pin) => {
                let val = pin.is_high().map_err(|_| TmcError::PinError)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    /// If INDEX pin is provided, read it.
    pub fn read_index(&mut self) -> Result<Option<bool>, TmcError> {
        match &mut self.index {
            Some(pin) => {
                let val = pin.is_high().map_err(|_| TmcError::PinError)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }
}

// ---------------------------------------------------------------------------
// 3) Full UART Diagnostics & Control (Option 3)
// ---------------------------------------------------------------------------

/// TMC2209 in "Full UART Diagnostics and Control" mode.
///
/// - Requires EN, STEP, DIR, plus a UART interface
/// - No use of DIAG or INDEX pins here (user can wire them externally if desired).
pub struct Tmc2209FullUartDiagnosticsAndControl<EN, STEP, DIR, SERIAL, E>
where
    EN: OutputPin,
    STEP: OutputPin,
    DIR: OutputPin,
    SERIAL: Write + Read + ErrorType<Error = nb::Error<E>>,
{
    en: EN,
    step: STEP,
    dir: DIR,
    slave_address: u8,
    serial: SERIAL,
}

impl<EN, STEP, DIR, SERIAL, E> Tmc2209FullUartDiagnosticsAndControl<EN, STEP, DIR, SERIAL, E>
where
    EN: OutputPin,
    STEP: OutputPin,
    DIR: OutputPin,
    SERIAL: Write + Read + ErrorType<Error = nb::Error<E>>,
{
    /// Create a new driver in Full UART mode.
    pub fn new(en: EN, step: STEP, dir: DIR, serial: SERIAL, slave_address: u8) -> Self {
        Self {
            en,
            step,
            dir,
            slave_address,
            serial,
        }
    }

    /// Enable the driver (active-low => EN = LOW).
    pub fn enable(&mut self) -> Result<(), TmcError> {
        self.en.set_low().map_err(|_| TmcError::PinError)
    }

    /// Disable the driver (active-low => EN = HIGH).
    pub fn disable(&mut self) -> Result<(), TmcError> {
        self.en.set_high().map_err(|_| TmcError::PinError)
    }

    /// Set the direction pin.
    pub fn set_direction(&mut self, clockwise: bool) -> Result<(), TmcError> {
        if clockwise {
            self.dir.set_high().map_err(|_| TmcError::PinError)
        } else {
            self.dir.set_low().map_err(|_| TmcError::PinError)
        }
    }

    /// Issue a single step pulse (blocking).
    pub fn step_pulse(&mut self) -> Result<(), TmcError> {
        self.step.set_high().map_err(|_| TmcError::PinError)?;
        // short delay if needed
        self.step.set_low().map_err(|_| TmcError::PinError)
    }

    /// check IFCNT, set PDN_DISABLE, etc.
    pub fn init_uart(&mut self) -> Result<(), TmcError> {
        let ifcnt_before = self.read_register(REG_IFCNT)?;

        // Set PDN_DISABLE => use UART-based config
        let gconf = self.read_register(REG_GCONF)?;
        let new_gconf = gconf | GCONF_PDN_DISABLE;
        self.write_register(REG_GCONF, new_gconf)?;

        let ifcnt_after = self.read_register(REG_IFCNT)?;
        if ifcnt_after == ifcnt_before {
            return Err(TmcError::SerialError);
        }
        Ok(())
    }

    /// set run/hold current in IHOLD_IRUN via UART.
    pub fn set_current(&mut self, irun: u8, ihold: u8, ihold_delay: u8) -> Result<(), TmcError> {
        if irun > 31 || ihold > 31 || ihold_delay > 15 {
            return Err(TmcError::VerificationError);
        }
        let mut val = 0u32;
        val |= (ihold as u32) & 0x1F;
        val |= ((irun as u32) & 0x1F) << 8;
        val |= ((ihold_delay as u32) & 0x0F) << 16;
        self.write_register(REG_IHOLD_IRUN, val)?;
        Ok(())
    }

    /// Low-level 32-bit register write via UART (blocking).
    fn write_register(&mut self, reg: u8, value: u32) -> Result<(), TmcError> {
        let packet = build_write_packet(self.slave_address, reg, value);
        for &b in &packet {
            nb::block!(self.serial.write(&[b])).map_err(|_| TmcError::SerialError)?;
        }
        Ok(())
    }

    /// Low-level 32-bit register read via UART (blocking).
    fn read_register(&mut self, reg: u8) -> Result<u32, TmcError> {
        let packet = build_read_packet(self.slave_address, reg);
        for &b in &packet {
            nb::block!(self.serial.write(&[b])).map_err(|_| TmcError::SerialError)?;
        }

        let mut resp = [0u8; 7];
        for i in 0..7 {
            let val = 0u8;
            let val =
                nb::block!(self.serial.read(&mut [val])).map_err(|_| TmcError::SerialError)?;
            resp[i] = val as u8;
        }

        // Validate address
        if (resp[0] & 0x0F) != (self.slave_address & 0x0F) {
            return Err(TmcError::VerificationError);
        }
        // Validate register
        if (resp[1] & 0x7F) != (reg & 0x7F) {
            return Err(TmcError::VerificationError);
        }
        // CRC
        let crc_calc = calc_crc8(&resp[..6]);
        if crc_calc != resp[6] {
            return Err(TmcError::CrcError);
        }

        let d0 = resp[2] as u32;
        let d1 = resp[3] as u32;
        let d2 = resp[4] as u32;
        let d3 = resp[5] as u32;
        let val = d0 | (d1 << 8) | (d2 << 16) | (d3 << 24);
        Ok(val)
    }
}
