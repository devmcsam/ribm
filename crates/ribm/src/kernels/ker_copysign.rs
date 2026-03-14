//! The kernel for the copysign function for f32 and f64.

use crate::helpers::extraction::{f32_extract_magnitude_from_bits, f32_extract_sign_from_bits,
                                 f64_extract_magnitude_from_bits, f64_extract_sign_from_bits};

/// Copys the sign bit from the second number to the magnitude of the first number.
pub const fn ker_f64_copysign_bits(first: u64, second: u64) -> u64 {
    let mut num = f64_extract_magnitude_from_bits(first);
    num |= f64_extract_sign_from_bits(second);
    num
}

/// Copys the sign bit from the second number to the magnitude of the first number.
pub const fn ker_f64_copysign(first: f64, second: f64) -> f64 {
    f64::from_bits(ker_f64_copysign_bits(first.to_bits(), second.to_bits()))
}

/// Copys the sign bit from the second number to the magnitude of the first number.
pub const fn ker_f32_copysign_bits(first: u32, second: u32) -> u32 {
    let mut num = f32_extract_magnitude_from_bits(first);
    num |= f32_extract_sign_from_bits(second);
    num
}

/// Copys the sign bit from the second number to the magnitude of the first number.
pub const fn ker_f32_copysign(first: f32, second: f32) -> f32 {
    f32::from_bits(ker_f32_copysign_bits(first.to_bits(), second.to_bits()))
}