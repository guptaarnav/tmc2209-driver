# TMC2209 Driver (Rust, embedded-hal v1.0)

A platform-agnostic Rust driver for the Trinamic TMC2209 stepper motor driver.
Implements three modes of operation:
1.	**Standalone Legacy** (Tmc2209StandaloneLegacy) – Uses only STEP/DIR pins (no UART).
2.	**Standalone OTP Preconfig** (Tmc2209StandaloneOtpPreconfig) – Same pin usage as Legacy mode, assuming the chip’s OTP memory or external code has already done some config.
3.	**Full UART Diagnostics and Control** (Tmc2209FullUartDiagnosticsAndControl) – Enables all TMC2209 features by communicating over UART.

## Usage Examples
In a real-world scenario, you would provide actual MCU pins and a real timer or delay implementing embedded_hal traits such as [DelayMs] or [DelayUs]. The examples below assume you have some delay object for timing.
  1. Standalone Legacy Mode
```
use tmc2209_driver::Tmc2209StandaloneLegacy;
use tmc2209_driver::errors::TmcError;
use embedded_hal::delay::DelayMs;

fn main() -> Result<(), TmcError> {
    // Example hardware pins implementing 'OutputPin':
    let en_pin = /* ... */;
    let step_pin = /* ... */;
    let dir_pin = /* ... */;

    // Some embedded-hal delay object
    let mut delay = /* ... */;

    // Create the Legacy driver (no UART).
    let mut driver = Tmc2209StandaloneLegacy::new_basic(en_pin, step_pin, dir_pin);

    // Enable driver (active-low => sets EN pin LOW).
    driver.enable()?;
    // Rotate the motor clockwise.
    driver.set_direction(true)?;

    // Step 200 times, with a short delay between pulses
    for _ in 0..200 {
        driver.step_pulse()?;
        // 1 ms delay between pulses
        delay.delay_ms(1).ok();
    }

    // Optionally read DIAG/INDEX if using '.new_with_options(...)':
    // let diag_val = driver.read_diag()?;
    // let index_val = driver.read_index()?;
    Ok(())
}

```
2.  Standalone OTP Preconfig
```
use tmc2209_driver::Tmc2209StandaloneOtpPreconfig;
use tmc2209_driver::errors::TmcError;
use embedded_hal::delay::DelayMs;

fn main() -> Result<(), TmcError> {
    // Pins for EN, STEP, DIR. The TMC2209 is pre-configured via OTP.
    let en_pin = /* ... */;
    let step_pin = /* ... */;
    let dir_pin = /* ... */;

    let mut delay = /* ... */;

    let mut driver = Tmc2209StandaloneOtpPreconfig::new_basic(en_pin, step_pin, dir_pin);

    driver.enable()?;
    driver.set_direction(false)?; // Counterclockwise

    for _ in 0..200 {
        driver.step_pulse()?;
        delay.delay_ms(1).ok();
    }

    // If desired, read DIAG/INDEX pins:
    // let diag_val = driver.read_diag()?;
    // let index_val = driver.read_index()?;

    Ok(())
}
```  
3.  Full UART Diagnostics & Control
```
use tmc2209_driver::Tmc2209FullUartDiagnosticsAndControl;
use tmc2209_driver::errors::TmcError;
use embedded_io::blocking::Write as BlockingWrite; // or nb::block usage
use embedded_io::blocking::Read as BlockingRead;
use embedded_hal::delay::DelayMs;

fn main() -> Result<(), TmcError> {
    // Example pins implementing OutputPin:
    let en_pin = /* ... */;
    let step_pin = /* ... */;
    let dir_pin = /* ... */;

    // Some UART-like struct implementing `embedded_io::{Read, Write}`
    // plus `ErrorType<Error=nb::Error<E>>`.
    let serial_port = /* ... */;
    let slave_address = 0x01; // Example address

    // Some embedded-hal delay object
    let mut delay = /* ... */;

    let mut driver = Tmc2209FullUartDiagnosticsAndControl::new(
        en_pin,
        step_pin,
        dir_pin,
        serial_port,
        slave_address
    );

    // Enable the driver
    driver.enable()?;

    // Initialize UART-based config (e.g., check IFCNT, set PDN_DISABLE, etc.)
    driver.init_uart()?; 

    // Set run/hold current (IRUN=16, IHOLD=8, IHOLD_DELAY=6 as an example)
    driver.set_current(16, 8, 6)?;

    // Move the motor clockwise
    driver.set_direction(true)?;

    for _ in 0..200 {
        driver.step_pulse()?;
        delay.delay_ms(1).ok(); 
    }

    Ok(())
}
```
     
