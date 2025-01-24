//! Errors specific to the TMC2209 driver.

/// Error type for the TMC2209 driver.
#[derive(Debug)]
pub enum TmcError {
    /// Errors arising from pin operations (e.g., `OutputPin` setting).
    PinError,
    /// UART read/write errors.
    SerialError,
    /// CRC mismatch in read response
    CrcError,
    /// If a register readback check fails.
    VerificationError,
}
