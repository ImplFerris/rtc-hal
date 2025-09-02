//! # RTC Error Types and Classification
//!
//! This module provides a standardized error handling framework for RTC drivers,
//! allowing consistent error categorization across different RTC hardware implementations.

/// Common categories of errors for RTC drivers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

/// Trait that RTC driver error types should implement.
///
/// Allows converting driver-specific errors into standard categories.
/// Drivers can either define custom error types or use `ErrorKind` directly.
pub trait Error: core::fmt::Debug {
    /// Map a driver-specific error into a general category
    fn kind(&self) -> ErrorKind;
}

/// RTC error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

/// Allows `ErrorKind` to be used directly as an error type.
///
/// Simple drivers can use `type Error = ErrorKind` instead of defining custom errors.
impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

// blanket impl for all `&mut T`
impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bus => write!(f, "Underlying bus error occurred"),
            Self::InvalidDateTime => write!(f, "Invalid datetime value provided"),
            Self::InvalidAlarmConfig => write!(f, "Invalid alarm configuration"),
            Self::UnsupportedSqwFrequency => write!(
                f,
                "The specified square wave frequency is not supported by the RTC"
            ),
            Self::InvalidAddress => write!(f, "Invalid register address"),
            Self::NvramOutOfBounds => write!(f, "NVRAM address out of bounds"),
            Self::NvramWriteProtected => write!(f, "NVRAM is write protected"),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
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

    impl Error for MockRtcError {
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

    #[test]
    fn test_error_kind_returns_self() {
        let error = ErrorKind::Other;
        assert_eq!(error.kind(), ErrorKind::Other);
    }

    #[test]
    fn test_error_kind_display_messages() {
        assert_eq!(
            format!("{}", ErrorKind::Bus),
            "Underlying bus error occurred"
        );

        assert_eq!(
            format!("{}", ErrorKind::InvalidDateTime),
            "Invalid datetime value provided"
        );

        assert_eq!(
            format!("{}", ErrorKind::InvalidAlarmConfig),
            "Invalid alarm configuration"
        );

        assert_eq!(
            format!("{}", ErrorKind::UnsupportedSqwFrequency),
            "The specified square wave frequency is not supported by the RTC"
        );

        assert_eq!(
            format!("{}", ErrorKind::InvalidAddress),
            "Invalid register address"
        );

        assert_eq!(
            format!("{}", ErrorKind::NvramOutOfBounds),
            "NVRAM address out of bounds"
        );

        assert_eq!(
            format!("{}", ErrorKind::NvramWriteProtected),
            "NVRAM is write protected"
        );

        assert_eq!(
            format!("{}", ErrorKind::Other),
            "A different error occurred. The original error may contain more information"
        );
    }
}
