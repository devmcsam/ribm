//! Helper functions for extracting and classifying IEEE-754 floating-point
//! numbers at the bit level.

use crate::helpers::{DBL_EXP_SUBNORMAL, FLT_EXP_SUBNORMAL};
use super::constants::{DBL_EXP_BIAS, DBL_EXP_ZERO_SUBNORMAL, DBL_HIDDEN_BIT, DBL_MASK_ABS,
                       DBL_MASK_EXP, DBL_MASK_FRAC, DBL_MASK_SIGN, DBL_SHIFT_EXP, DBL_SHIFT_SIGN,
                       FLT_EXP_BIAS, FLT_EXP_ZERO_SUBNORMAL, FLT_HIDDEN_BIT, FLT_MASK_ABS,
                       FLT_MASK_EXP, FLT_MASK_FRAC, FLT_MASK_SIGN, FLT_SHIFT_EXP, FLT_SHIFT_SIGN,
};
use super::normalization::{f32_normalize_subnormal, f64_normalize_subnormal};
use super::classification::{f32_is_zero_from_bits, f64_is_zero_from_bits};

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
/// For normal numbers, this returns the standard (biased - 127).
/// For subnormal numbers, it performs normalization and returns the true
/// effective exponent.
pub const fn f32_extract_unbiased_exponent_from_bits(bits: u32) -> i32 {
    let exp = f32_extract_biased_exponent_from_bits(bits);
    if exp == FLT_EXP_ZERO_SUBNORMAL {
        if f32_is_zero_from_bits(bits) {
            return FLT_EXP_SUBNORMAL;
        }
        let frac = f32_extract_fraction_from_bits(bits);
        let (_, adj) = f32_normalize_subnormal(frac);
        FLT_EXP_SUBNORMAL + adj
    } else {
        exp as i32 - FLT_EXP_BIAS as i32
    }
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
/// For normal numbers, this returns the standard (biased - 1023).
/// For subnormal numbers, it performs normalization and returns the true
/// effective exponent.
pub const fn f64_extract_unbiased_exponent_from_bits(bits: u64) -> i32 {
    let exp = f64_extract_biased_exponent_from_bits(bits);
    if exp == DBL_EXP_ZERO_SUBNORMAL {
        if f64_is_zero_from_bits(bits) {
            return DBL_EXP_SUBNORMAL;
        }
        let frac = f64_extract_fraction_from_bits(bits);
        let (_, adj) = f64_normalize_subnormal(frac);
        DBL_EXP_SUBNORMAL + adj
    } else {
        exp as i32 - DBL_EXP_BIAS as i32
    }
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