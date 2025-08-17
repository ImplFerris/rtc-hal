//! # RTC HAL - Platform-agnostic Real-Time Clock traits
//!
//! This crate provides traits for implementing Real-Time Clock (RTC) drivers
//! in a platform-agnostic way, following the embedded-hal design patterns.
//!
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod bcd;
pub mod datetime;
pub mod error;
pub mod nvram;
pub mod rtc;
