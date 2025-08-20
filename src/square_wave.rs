//! Traits for Square Wave control

use crate::rtc::Rtc;

/// Square wave output frequencies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquareWaveFreq {
    /// 1 Hz
    Hz1,
    /// 1024 Hz (1.024 kHz)
    Hz1024,
    /// 4096 Hz (4.096 kHz)
    Hz4096,
    /// 8192 Hz (8.192 kHz)
    Hz8192,
    /// 32768 Hz (32.768 kHz)
    Hz32768,
    /// Custom frequency (if supported by device)
    Custom(u32),
}

impl SquareWaveFreq {
    /// Get frequency value in Hz
    pub fn to_hz(&self) -> u32 {
        match self {
            Self::Hz1 => 1,
            Self::Hz1024 => 1024,
            Self::Hz4096 => 4096,
            Self::Hz8192 => 8192,
            Self::Hz32768 => 32768,
            Self::Custom(freq) => *freq,
        }
    }

    /// Create from Hz value
    pub fn from_hz(hz: u32) -> Self {
        match hz {
            1 => Self::Hz1,
            1024 => Self::Hz1024,
            4096 => Self::Hz4096,
            8192 => Self::Hz8192,
            32768 => Self::Hz32768,
            other => Self::Custom(other),
        }
    }
}

/// Square wave functionality trait
pub trait SquareWave: Rtc {
    /// Enable square wave output with specified frequency
    fn enable_square_wave(&mut self, freq: SquareWaveFreq) -> Result<(), Self::Error>;

    /// Disable square wave output
    fn disable_square_wave(&mut self) -> Result<(), Self::Error>;

    /// Set the frequency (without enabling/disabling)
    fn set_square_wave_frequency(&mut self, freq: SquareWaveFreq) -> Result<(), Self::Error>;
}
