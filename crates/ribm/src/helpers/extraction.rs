//! Helper functions for extracting and classifying IEEE-754 floating-point
//! numbers at the bit level.

use super::constants::{DBL_EXP_BIAS, DBL_EXP_ZERO_SUBNORMAL, DBL_HIDDEN_BIT, DBL_MASK_ABS,
                       DBL_MASK_EXP, DBL_MASK_FRAC, DBL_MASK_SIGN, DBL_SHIFT_EXP, DBL_SHIFT_SIGN,
                       FLT_EXP_BIAS, FLT_EXP_ZERO_SUBNORMAL, FLT_HIDDEN_BIT, FLT_MASK_ABS,
                       FLT_MASK_EXP, FLT_MASK_FRAC, FLT_MASK_SIGN, FLT_SHIFT_EXP, FLT_SHIFT_SIGN,
};

/// Extract the sign bit (0 or 1) from raw f32 bits.
pub const fn f32_extract_sign_from_bits(bits: u32) -> u32 {
    bits >> FLT_SHIFT_SIGN
}

/// Extract the sign bit (0 or 1) from an f32.
pub const fn f32_extract_sign(num: f32) -> u32 {
    f32_extract_sign_from_bits(num.to_bits())
}

/// Extract the biased exponent field (0..=255) from raw f32 bits.
pub const fn f32_extract_biased_exponent_from_bits(bits: u32) -> u32 {
    (bits & FLT_MASK_EXP) >> FLT_SHIFT_EXP
}

/// Extract the biased exponent field (0..=255) from an f32.
pub const fn f32_extract_biased_exponent(num: f32) -> u32 {
    f32_extract_biased_exponent_from_bits(num.to_bits())
}

/// Extract the unbiased exponent (biased - 127) from raw f32 bits.
///
/// Note: For subnormals this returns -127, which is NOT the effective exponent
/// per IEEE 754 (that would be -126). Use the normalization helpers if you
/// need the true exponent for subnormals.
pub const fn f32_extract_unbiased_exponent_from_bits(bits: u32) -> i32 {
    // TODO: subnormal normalization -- adjust exponent to -126 and account
    //       for the leading-zero count in the significand.
    let exp = f32_extract_biased_exponent_from_bits(bits);
    exp as i32 - FLT_EXP_BIAS as i32
}

/// Extract the unbiased exponent (biased - 127) from an f32.
pub const fn f32_extract_unbiased_exponent(num: f32) -> i32 {
    f32_extract_unbiased_exponent_from_bits(num.to_bits())
}

/// Extract the fraction (trailing significand) field from raw f32 bits.
pub const fn f32_extract_fraction_from_bits(bits: u32) -> u32 {
    bits & FLT_MASK_FRAC
}

/// Extract the fraction (trailing significand) field from an f32.
pub const fn f32_extract_fraction(num: f32) -> u32 {
    f32_extract_fraction_from_bits(num.to_bits())
}

/// Extract the full significand (mantissa) from raw f32 bits.
///
/// For normals this prepends the implicit leading 1 (hidden bit).
/// For zeros and subnormals the hidden bit is absent.
pub const fn f32_extract_mantissa_from_bits(bits: u32) -> u32 {
    let frac = f32_extract_fraction_from_bits(bits);
    let exp = f32_extract_biased_exponent_from_bits(bits);

    if exp == FLT_EXP_ZERO_SUBNORMAL {
        frac
    } else {
        frac | FLT_HIDDEN_BIT
    }
}

/// Extract the full significand (mantissa) from an f32.
pub const fn f32_extract_mantissa(num: f32) -> u32 {
    f32_extract_mantissa_from_bits(num.to_bits())
}

/// Clear the sign bit, returning the absolute-value bit pattern.
pub const fn f32_abs_bits(bits: u32) -> u32 {
    bits & FLT_MASK_ABS
}

/// Flip the sign bit.
pub const fn f32_negate_bits(bits: u32) -> u32 {
    bits ^ FLT_MASK_SIGN
}

/// Copy the sign of sign_bits onto magnitude_bits.
pub const fn f32_copysign_bits(magnitude_bits: u32, sign_bits: u32) -> u32 {
    (magnitude_bits & FLT_MASK_ABS) | (sign_bits & FLT_MASK_SIGN)
}

/// Compose an f32 bit pattern from its sign (0 or 1), biased exponent, and
/// fraction field. No validation is performed.
pub const fn f32_compose_bits(sign: u32, biased_exp: u32, fraction: u32) -> u32 {
    (sign << FLT_SHIFT_SIGN) | (biased_exp << FLT_SHIFT_EXP) | (fraction & FLT_MASK_FRAC)
}

/// Extract the sign bit (0 or 1) from raw f64 bits.
pub const fn f64_extract_sign_from_bits(bits: u64) -> u64 {
    bits >> DBL_SHIFT_SIGN
}

/// Extract the sign bit (0 or 1) from an f64.
pub const fn f64_extract_sign(num: f64) -> u64 {
    f64_extract_sign_from_bits(num.to_bits())
}

/// Extract the biased exponent field (0..=2047) from raw f64 bits.
pub const fn f64_extract_biased_exponent_from_bits(bits: u64) -> u64 {
    (bits & DBL_MASK_EXP) >> DBL_SHIFT_EXP
}

/// Extract the biased exponent field (0..=2047) from an f64.
pub const fn f64_extract_biased_exponent(num: f64) -> u64 {
    f64_extract_biased_exponent_from_bits(num.to_bits())
}

/// Extract the unbiased exponent (biased - 1023) from raw f64 bits.
///
/// Note: For subnormals this returns -1023, which is NOT the effective exponent
/// per IEEE 754 (that would be -1022). Use the normalization helpers if you
/// need the true exponent for subnormals.
pub const fn f64_extract_unbiased_exponent_from_bits(bits: u64) -> i32 {
    // TODO: subnormal normalization -- adjust exponent to -1022 and account
    //       for the leading-zero count in the significand.
    let exp = f64_extract_biased_exponent_from_bits(bits);
    exp as i32 - DBL_EXP_BIAS as i32
}

/// Extract the unbiased exponent (biased - 1023) from an f64.
pub const fn f64_extract_unbiased_exponent(num: f64) -> i32 {
    f64_extract_unbiased_exponent_from_bits(num.to_bits())
}

/// Extract the fraction (trailing significand) field from raw f64 bits.
pub const fn f64_extract_fraction_from_bits(bits: u64) -> u64 {
    bits & DBL_MASK_FRAC
}

/// Extract the fraction (trailing significand) field from an f64.
pub const fn f64_extract_fraction(num: f64) -> u64 {
    f64_extract_fraction_from_bits(num.to_bits())
}

/// Extract the full significand (mantissa) from raw f64 bits.
///
/// For normals this prepends the implicit leading 1 (hidden bit).
/// For zeros and subnormals the hidden bit is absent.
pub const fn f64_extract_mantissa_from_bits(bits: u64) -> u64 {
    let frac = f64_extract_fraction_from_bits(bits);
    let exp = f64_extract_biased_exponent_from_bits(bits);

    if exp == DBL_EXP_ZERO_SUBNORMAL {
        frac
    } else {
        frac | DBL_HIDDEN_BIT
    }
}

/// Extract the full significand (mantissa) from an f64.
pub const fn f64_extract_mantissa(num: f64) -> u64 {
    f64_extract_mantissa_from_bits(num.to_bits())
}

/// Clear the sign bit, returning the absolute-value bit pattern.
pub const fn f64_abs_bits(bits: u64) -> u64 {
    bits & DBL_MASK_ABS
}

/// Flip the sign bit.
pub const fn f64_negate_bits(bits: u64) -> u64 {
    bits ^ DBL_MASK_SIGN
}

/// Copy the sign of sign_bits onto magnitude_bits.
pub const fn f64_copysign_bits(magnitude_bits: u64, sign_bits: u64) -> u64 {
    (magnitude_bits & DBL_MASK_ABS) | (sign_bits & DBL_MASK_SIGN)
}

/// Compose an f64 bit pattern from its sign (0 or 1), biased exponent, and
/// fraction field. No validation is performed.
pub const fn f64_compose_bits(sign: u64, biased_exp: u64, fraction: u64) -> u64 {
    (sign << DBL_SHIFT_SIGN) | (biased_exp << DBL_SHIFT_EXP) | (fraction & DBL_MASK_FRAC)
}