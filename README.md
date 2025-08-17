# rtc-hal

A platform-agnostic Hardware Abstraction Layer (HAL) for Real-Time Clock (RTC) devices in embedded Rust systems. This crate provides traits and utilities for implementing RTC drivers following the `embedded-hal` design patterns.

## Architecture

This crate is organized into several modules:

- **`datetime`** - Core `DateTime` struct with validation and utility functions
- **`rtc`** - Main `Rtc` trait defining the RTC interface
- **`nvram`** - `RtcNvram` trait for RTCs with non-volatile memory
- **`error`** - Error handling framework and `RtcError` trait
- **`bcd`** - Binary Coded Decimal conversion utilities
   
## License

This project is licensed under the MIT License.
