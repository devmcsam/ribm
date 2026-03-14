//! Subnormal floating-point normalization helpers.
//!
//! These shift a subnormal fraction field left until the hidden bit is in
//! position, returning the normalized fraction and an exponent adjustment.
//! Both functions are branchless and rely on a single leading_zeros()
//! intrinsic (compiles to lzcnt / clz on x86 / ARM).

use super::constants::{FLT_BITS_FRAC, FLT_MASK_FRAC, DBL_BITS_FRAC, DBL_MASK_FRAC};

/// Normalize a subnormal f32 fraction field.
///
/// Takes the raw 23-bit fraction from a subnormal f32 (biased exponent == 0)
/// and shifts it left until bit 23 (the hidden bit position) is set.
///
/// Returns (normalized_fraction, exponent_adjustment) where:
/// - normalized_fraction has the hidden bit stripped (only the 23 fraction bits).
/// - exponent_adjustment is a negative i32 to add to the subnormal's effective
///   exponent (1 - bias) to obtain the true unbiased exponent.
///
/// The caller must ensure frac is non-zero; passing zero is undefined and will
/// produce a garbage shift count.
#[must_use]
pub const fn f32_normalize_subnormal(frac: u32) -> (u32, i32) {
    // Number of leading zeros beyond what a properly positioned bit 23 would have.
    // A u32 with bit 23 set has 8 leading zeros, so:
    //   shift = leading_zeros(frac) - 8
    // This equals: leading_zeros(frac) - (32 - 24) = leading_zeros(frac) - (32 - FLT_BITS_FRAC - 1)
    let shift = frac.leading_zeros() - (u32::BITS - FLT_BITS_FRAC - 1);
    let normalized = (frac << shift) & FLT_MASK_FRAC;
    let exponent_adjustment = -(shift as i32) + 1;
    (normalized, exponent_adjustment)
}

/// Normalize a subnormal f64 fraction field.
///
/// Takes the raw 52-bit fraction from a subnormal f64 (biased exponent == 0)
/// and shifts it left until bit 52 (the hidden bit position) is set.
///
/// Returns (normalized_fraction, exponent_adjustment) where:
/// - normalized_fraction has the hidden bit stripped (only the 52 fraction bits).
/// - exponent_adjustment is a negative i32 to add to the subnormal's effective
///   exponent (1 - bias) to obtain the true unbiased exponent.
///
/// The caller must ensure frac is non-zero; passing zero is undefined and will
/// produce a garbage shift count.
#[must_use]
pub const fn f64_normalize_subnormal(frac: u64) -> (u64, i32) {
    let shift = frac.leading_zeros() - (u64::BITS - DBL_BITS_FRAC - 1);
    let normalized = (frac << shift) & DBL_MASK_FRAC;
    let exponent_adjustment = -(shift as i32) + 1;
    (normalized, exponent_adjustment)
}