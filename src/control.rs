//! Power control functionality for RTC devices.

use crate::rtc::Rtc;

/// This trait extends [`Rtc`] with methods to start and halt the RTC clock.
pub trait RtcPowerControl: Rtc {
    /// Start or resume the RTC oscillator so that timekeeping can continue.
    fn start_clock(&mut self) -> Result<(), Self::Error>;

    /// Halt the RTC oscillator, pausing timekeeping until restarted.
    fn halt_clock(&mut self) -> Result<(), Self::Error>;
}
