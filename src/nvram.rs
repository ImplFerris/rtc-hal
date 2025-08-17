//! Traits for RTC with non-volatile memory (NVRAM/SRAM) access

use crate::rtc::Rtc;

/// RTC with non-volatile memory (NVRAM/SRAM) access
pub trait RtcNvram: Rtc {
    /// Read data from NVRAM starting at the given offset
    ///
    /// # Parameters
    /// * `offset` - NVRAM offset (0 = first NVRAM byte, up to device-specific max)
    /// * `buffer` - Buffer to store the read data
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(Self::Error)` if offset or length is invalid, or read fails
    fn read_nvram(&mut self, offset: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;

    /// Write data to NVRAM starting at the given offset
    ///
    /// # Parameters
    /// * `offset` - NVRAM offset (0 = first NVRAM byte, up to device-specific max)
    /// * `data` - Data to write to NVRAM
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(Self::Error)` if offset or length is invalid, or write fails
    fn write_nvram(&mut self, offset: u8, data: &[u8]) -> Result<(), Self::Error>;

    /// Get the size of available NVRAM in bytes
    ///
    /// # Returns
    /// Total NVRAM size (e.g., 56 for DS1307, 0 for DS3231)
    fn nvram_size(&self) -> u16;
}
