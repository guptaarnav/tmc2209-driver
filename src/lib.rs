#![no_std]
//! TMC2209 Driver Crate
//!
//! This crate provides a platform-agnostic driver for the TMC2209 stepper motor driver,
//! relying on `embedded-hal` traits for hardware abstraction.
//!
//! # Usage
//! ```ignore
//! use tmc2209_driver::Tmc2209;
//! // (Instantiate pins/UART from your HAL, then call Tmc2209::new(...))
//! ```
//!
//! # Features
//! - Read/Write TMC2209 registers over UART
//! - Control step/dir pins
//! - Configurable microstepping, current, stealthChop, etc.
//!

mod config;
mod errors;
mod packet;
mod registers;
mod tmc2209;

pub use config::*;
pub use errors::*;
pub use tmc2209::Tmc2209FullUartDiagnosticsAndControl;
pub use tmc2209::Tmc2209StandaloneLegacy;
pub use tmc2209::Tmc2209StandaloneOtpPreconfig;

/// Crate prelude, re-exporting common items (optional).
pub mod prelude {
    pub use crate::Tmc2209FullUartDiagnosticsAndControl;
    pub use crate::Tmc2209StandaloneLegacy;
    pub use crate::Tmc2209StandaloneOtpPreconfig;
    // re-export if you want shorter import lines
}
