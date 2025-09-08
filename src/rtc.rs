//! # RTC Trait Interface
//!
//! This module defines the core trait for Real-Time Clock (RTC) devices in embedded systems.
//!
//! ## Features
//! - Provides a platform-independent interface for reading and writing date/time values to hardware RTC chips.
//! - Compatible with the design patterns of `embedded-hal`, focusing on trait-based abstraction.
//! - Uses the hardware-agnostic `DateTime` struct for representing calendar date and time.
//!
//! ## Usage Notes
//! - Each RTC driver should implement its own error type conforming to the `Error` trait, allowing accurate hardware-specific error reporting.
//! - Drivers are responsible for validating that all `DateTime` values provided are within the supported range of their underlying hardware (for example, some chips only support years 2000-2099).
//! - This trait is intended for use in platform implementors and applications needing unified RTC access across hardware targets.
//!
//! ## For application and library developers
//!
//! Applications and libraries should take the `Rtc` instance as an argument to `new()`, and store it in their
//! struct. They **should not** take `&mut Rtc`, the trait has a blanket impl for all `&mut T`
//! so taking just `Rtc` ensures the user can still pass a `&mut`, but is not forced to.
//!
//! Applications and libraries **should not** try to enable sharing by taking `&mut Rtc` at every method.
//! This is much less ergonomic than owning the `Rtc`, which still allows the user to pass an
//! implementation that does sharing behind the scenes.
//!
//! ## Example
//! ```ignore
//! use crate::{datetime::DateTime, error::ErrorType, rtc::Rtc};
//!
//! let mut rtc = Ds1307::new(i2c);
//! let now = rtc.get_datetime()?;
//! rtc.set_datetime(&DateTime::new(2024, 8, 16, 12, 0, 0)?)?;
//! ```
use crate::{datetime::DateTime, error::ErrorType};

/// Core trait for Real-Time Clock (RTC) devices.
///
/// This trait provides a platform-agnostic interface for reading and
/// writing date/time values from hardware RTC chips. It is designed
/// to be similar in style to `embedded-hal` traits.
///
/// Each RTC implementation should define:
/// - An associated error type for hardware-specific errors
///
/// The `DateTime` struct used here is hardware-agnostic. Drivers must
/// validate that provided values fall within the supported range.
///
/// # Example
///
/// ```ignore
/// let mut rtc = Ds1307::new(i2c);
/// let now = rtc.get_datetime()?;
/// rtc.set_datetime(&DateTime::new(2024, 8, 16, 12, 0, 0)?)?;
pub trait Rtc: ErrorType {
    /// Get the current date and time atomically.
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` if communication with the RTC fails.
    fn get_datetime(&mut self) -> Result<DateTime, Self::Error>;

    /// Set the current date and time atomically.
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` if communication with the RTC fails or
    /// if the provided `DateTime` is out of range for this device.
    fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), Self::Error>;
}

/// blanket impl for all `&mut T`
impl<T: Rtc + ?Sized> Rtc for &mut T {
    #[inline]
    fn get_datetime(&mut self) -> Result<DateTime, Self::Error> {
        T::get_datetime(self)
    }

    #[inline]
    fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), Self::Error> {
        T::set_datetime(self, datetime)
    }
}
