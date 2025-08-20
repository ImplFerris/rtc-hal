//! # RTC Error Types and Classification
//!
//! This module provides a standardized error handling framework for RTC drivers,
//! allowing consistent error categorization across different RTC hardware implementations.

/// Common categories of errors for RTC drivers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    // Errors related to core traits
    /// Underlying bus error (I2C, SPI, etc.)
    Bus,
    /// Invalid date/time value provided
    InvalidDateTime,

    // Errors related to extended
    /// Invalid alarm configuration
    InvalidAlarmConfig,
    /// The specified square wave frequency is not supported by the RTC
    UnsupportedSqwFrequency,
    /// Invalid register address
    InvalidAddress,
    /// NVRAM address out of bounds
    NvramOutOfBounds,
    /// NVRAM is write protected
    NvramWriteProtected,

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
        InvalidAlarmTime,
        UnsupportedSqwFrequency,
        InvalidRegisterAddress,
        NvramAddressOutOfBounds,
        NvramWriteProtected,
        UnknownError,
    }

    impl RtcError for MockRtcError {
        fn kind(&self) -> ErrorKind {
            match self {
                MockRtcError::I2cError => ErrorKind::Bus,
                MockRtcError::InvalidDateTime => ErrorKind::InvalidDateTime,
                MockRtcError::InvalidAlarmTime => ErrorKind::InvalidAlarmConfig,
                MockRtcError::UnsupportedSqwFrequency => ErrorKind::UnsupportedSqwFrequency,
                MockRtcError::InvalidRegisterAddress => ErrorKind::InvalidAddress,
                MockRtcError::NvramAddressOutOfBounds => ErrorKind::NvramOutOfBounds,
                MockRtcError::NvramWriteProtected => ErrorKind::NvramWriteProtected,
                MockRtcError::UnknownError => ErrorKind::Other,
            }
        }
    }

    #[test]
    fn test_error_kind_mapping() {
        assert_eq!(MockRtcError::I2cError.kind(), ErrorKind::Bus);
        assert_eq!(
            MockRtcError::InvalidDateTime.kind(),
            ErrorKind::InvalidDateTime
        );
        assert_eq!(
            MockRtcError::InvalidAlarmTime.kind(),
            ErrorKind::InvalidAlarmConfig
        );
        assert_eq!(
            MockRtcError::UnsupportedSqwFrequency.kind(),
            ErrorKind::UnsupportedSqwFrequency
        );
        assert_eq!(
            MockRtcError::InvalidRegisterAddress.kind(),
            ErrorKind::InvalidAddress
        );
        assert_eq!(
            MockRtcError::NvramAddressOutOfBounds.kind(),
            ErrorKind::NvramOutOfBounds
        );
        assert_eq!(
            MockRtcError::NvramWriteProtected.kind(),
            ErrorKind::NvramWriteProtected
        );
        assert_eq!(MockRtcError::UnknownError.kind(), ErrorKind::Other);
    }

    #[test]
    fn test_error_kind_equality() {
        assert_eq!(ErrorKind::Bus, ErrorKind::Bus);
        assert_ne!(ErrorKind::Bus, ErrorKind::InvalidDateTime);
        assert_ne!(
            ErrorKind::InvalidAlarmConfig,
            ErrorKind::UnsupportedSqwFrequency
        );
        assert_ne!(ErrorKind::NvramOutOfBounds, ErrorKind::NvramWriteProtected);
    }
}
