//! TMC2209 Register Definitions
//!
//! This module contains all known TMC2209 register addresses, bitfield positions, etc.

//! TMC2209 Register Definitions

// Commonly used registers
pub const REG_GCONF: u8 = 0x00;
pub const REG_GSTAT: u8 = 0x01;
pub const REG_IFCNT: u8 = 0x02;
pub const REG_SLAVECONF: u8 = 0x03;
pub const REG_IOIN: u8 = 0x06;
pub const REG_FACTORY_CONF: u8 = 0x07;

// Current control
pub const REG_IHOLD_IRUN: u8 = 0x10;
pub const REG_TPOWERDOWN: u8 = 0x11;
pub const REG_TSTEP: u8 = 0x12;
pub const REG_TPWMTHRS: u8 = 0x13;
pub const REG_VACTUAL: u8 = 0x22;

// StallGuard / CoolStep
pub const REG_TCOOLTHRS: u8 = 0x14;
pub const REG_SGTHRS: u8 = 0x40;
pub const REG_SG_RESULT: u8 = 0x41;
pub const REG_COOLCONF: u8 = 0x42;

// Sequencer registers
pub const REG_MSCNT: u8 = 0x6A;
pub const REG_MSCURACT: u8 = 0x6B;

// Chopper & driver
pub const REG_CHOPCONF: u8 = 0x6C;
pub const REG_DRVSTATUS: u8 = 0x6F;
pub const REG_PWMCONF: u8 = 0x70;
pub const REG_PWMSTATUS: u8 = 0x71;
pub const REG_ENCM_CTRL: u8 = 0x72;

// --- GCONF bits ---
pub const GCONF_I_SCALE_ANALOG: u32 = 1 << 0; // 0 => internal reference, 1 => VREF pin
pub const GCONF_INTERNAL_RSENSE: u32 = 1 << 1;
pub const GCONF_EN_SPREADCYCLE: u32 = 1 << 2; // 0 => stealthChop, 1 => spreadCycle
pub const GCONF_SHAFT: u32 = 1 << 3;
pub const GCONF_INDEX_OTPW: u32 = 1 << 4;
pub const GCONF_INDEX_STEP: u32 = 1 << 5;
pub const GCONF_PDN_DISABLE: u32 = 1 << 6; // 1 => PDN_UART pin disabled
pub const GCONF_MSTEP_REG_SELECT: u32 = 1 << 7; // 1 => microstep resolution by MRES bits
pub const GCONF_MULTISTEP_FILT: u32 = 1 << 8;
pub const GCONF_TEST_MODE: u32 = 1 << 9; // not for normal use

// --- IHOLD_IRUN bits ---
// Bits [4..0]: IHOLD
// Bits [12..8]: IRUN
// Bits [19..16]: IHOLDDELAY
