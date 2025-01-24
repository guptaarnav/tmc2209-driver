//! Configuration objects or enums for TMC2209 usage
//! This is optional, but can help keep user settings typed.

/// Example of a motor config struct for run/hold current, etc.
#[derive(Debug, Clone, Copy)]
pub struct MotorConfig {
    /// Run current in [0..31], fraction of max current
    pub run_current: u8,
    /// Hold current in [0..31], fraction of max current
    pub hold_current: u8,
    /// Hold current delay in [0..15]
    pub hold_delay: u8,
}

impl Default for MotorConfig {
    fn default() -> Self {
        MotorConfig {
            run_current: 16,
            hold_current: 8,
            hold_delay: 8,
        }
    }
}
