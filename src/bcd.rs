//! # BCD (Binary Coded Decimal) conversion utilities for RTC operations
//!
//! This module provides conversion functions between BCD and decimal formats
//! commonly used in Real-Time Clock (RTC) chips like the DS1307.
//!
//! ## BCD Format Overview
//!
//! Binary Coded Decimal (BCD) represents decimal digits using 4-bit binary patterns:
//! - Each decimal digit (0-9) is encoded in 4 bits
//! - One byte can hold two decimal digits (00-99)
//! - High nibble = tens digit, low nibble = ones digit
//!
//! ## Format Examples
//!
//! | Decimal | BCD Binary   | BCD Hex |
//! |---------|--------------|---------|
//! | 00      | 0000 0000    | 0x00    |
//! | 01      | 0000 0001    | 0x01    |
//! | 09      | 0000 1001    | 0x09    |
//! | 10      | 0001 0000    | 0x10    |
//! | 23      | 0010 0011    | 0x23    |
//! | 59      | 0101 1001    | 0x59    |
//! | 99      | 1001 1001    | 0x99    |
//!

/// Convert a BCD encoded byte to decimal
/// First 4 bits are tens of the number
/// Last 4 bits are ones of the number
pub fn to_decimal(bcd: u8) -> u8 {
    ((bcd >> 4) * 10) + (bcd & 0x0F)
}

/// Convert decimal byte to BCD encoding
pub fn from_decimal(decimal: u8) -> u8 {
    debug_assert!(decimal <= 99, "Decimal value must be <= 99 for BCD");
    ((decimal / 10) << 4) | (decimal % 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal(0x00), 0);
        assert_eq!(to_decimal(0x01), 1);
        assert_eq!(to_decimal(0x09), 9);
        assert_eq!(to_decimal(0x10), 10);
        assert_eq!(to_decimal(0x23), 23);
        assert_eq!(to_decimal(0x45), 45);
        assert_eq!(to_decimal(0x59), 59);
        assert_eq!(to_decimal(0x99), 99);
    }

    #[test]
    fn test_from_decimal() {
        assert_eq!(from_decimal(0), 0x00);
        assert_eq!(from_decimal(1), 0x01);
        assert_eq!(from_decimal(9), 0x09);
        assert_eq!(from_decimal(10), 0x10);
        assert_eq!(from_decimal(23), 0x23);
        assert_eq!(from_decimal(45), 0x45);
        assert_eq!(from_decimal(59), 0x59);
        assert_eq!(from_decimal(99), 0x99);
    }

    #[test]
    fn test_bcd_round_trip() {
        for i in 0..=99 {
            let bcd = from_decimal(i);
            let decimal = to_decimal(bcd);
            assert_eq!(i, decimal);
        }
    }

    #[test]
    fn test_bcd_edge_cases() {
        // Test boundary values commonly used in RTC
        let test_cases = [0, 1, 9, 10, 23, 31, 59, 99];

        for &value in &test_cases {
            let bcd = from_decimal(value);
            let back = to_decimal(bcd);
            assert_eq!(value, back, "Failed for value: {value}");
        }
    }
}
