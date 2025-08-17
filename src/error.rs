//! # RTC Error Types and Classification
//!
//! This module provides a standardized error handling framework for RTC drivers,
//! allowing consistent error categorization across different RTC hardware implementations.
//!
//! ## Error Categories
//!
//! RTC operations can fail in several predictable ways:
//! - **Bus errors**: I2C/SPI communication failures
//! - **Invalid input**: Out-of-range date/time values
//! - **Hardware issues**: Clock not running, oscillator stopped
//! - **Other**: Implementation-specific errors

/// Common categories of errors for RTC drivers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Underlying bus error (I2C, SPI, etc.)
    Bus,
    /// Invalid date/time value provided
    InvalidInput,
    /// The RTC is not running (e.g. oscillator stopped)
    NotRunning,
    /// Any other error not covered above
    Other,
}

/// Trait that RTC driver error types should implement
pub trait RtcError {
    /// Map a driver-specific error into a general category
    fn kind(&self) -> ErrorKind;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock error type for testing
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum MockRtcError {
        I2cError,
        InvalidDateTime,
        ClockStopped,
        UnknownError,
    }

    impl RtcError for MockRtcError {
        fn kind(&self) -> ErrorKind {
            match self {
                MockRtcError::I2cError => ErrorKind::Bus,
                MockRtcError::InvalidDateTime => ErrorKind::InvalidInput,
                MockRtcError::ClockStopped => ErrorKind::NotRunning,
                MockRtcError::UnknownError => ErrorKind::Other,
            }
        }
    }

    #[test]
    fn test_error_kind_mapping() {
        assert_eq!(MockRtcError::I2cError.kind(), ErrorKind::Bus);
        assert_eq!(
            MockRtcError::InvalidDateTime.kind(),
            ErrorKind::InvalidInput
        );
        assert_eq!(MockRtcError::ClockStopped.kind(), ErrorKind::NotRunning);
        assert_eq!(MockRtcError::UnknownError.kind(), ErrorKind::Other);
    }

    #[test]
    fn test_error_kind_equality() {
        assert_eq!(ErrorKind::Bus, ErrorKind::Bus);
        assert_ne!(ErrorKind::Bus, ErrorKind::InvalidInput);
    }
}
