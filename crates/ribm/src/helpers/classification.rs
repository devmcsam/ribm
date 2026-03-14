use crate::helpers::{
    f32_extract_biased_exponent_from_bits,
    f32_extract_fraction_from_bits,
    f32_extract_sign_from_bits,
    f64_extract_biased_exponent_from_bits,
    f64_extract_fraction_from_bits,
    f64_extract_sign_from_bits,
};
use super::constants::{FLT_EXP_INF_NAN, FLT_SHIFT_EXP, FLT_MASK_QUIET_NAN, FLT_MASK_ABS, FLT_INF,
                       FLT_POS_INF, FLT_NEG_INF, FLT_EXP_ZERO_SUBNORMAL, FLT_EXP_BIASED_MIN_NORMAL,
                       FLT_EXP_BIASED_MAX_NORMAL, FLT_MASK_EXP, DBL_EXP_INF_NAN, DBL_SHIFT_EXP,
                       DBL_MASK_QUIET_NAN, DBL_MASK_ABS, DBL_INF, DBL_POS_INF, DBL_NEG_INF,
                       DBL_EXP_ZERO_SUBNORMAL, DBL_EXP_BIASED_MIN_NORMAL, DBL_EXP_BIASED_MAX_NORMAL,
                       DBL_MASK_EXP};

/// Returns true if the f32 bit pattern represents a NaN.
pub const fn f32_is_nan_from_bits(bits: u32) -> bool {
    // NaN: exponent field is all-ones AND fraction is non-zero.
    f32_extract_biased_exponent_from_bits(bits) == (FLT_EXP_INF_NAN >> FLT_SHIFT_EXP)
        && f32_extract_fraction_from_bits(bits) != 0
}

/// Returns true if num is NaN.
pub const fn f32_is_nan(num: f32) -> bool {
    f32_is_nan_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a quiet NaN.
pub const fn f32_is_qnan_from_bits(bits: u32) -> bool {
    // Quiet NaN: exponent all-ones, quiet bit set, and fraction non-zero.
    f32_is_nan_from_bits(bits) && (bits & FLT_MASK_QUIET_NAN) != 0
}

/// Returns true if num is a quiet NaN.
pub const fn f32_is_qnan(num: f32) -> bool {
    f32_is_qnan_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a positive quiet NaN.
pub const fn f32_is_pos_qnan_from_bits(bits: u32) -> bool {
    f32_is_qnan_from_bits(bits) && f32_extract_sign_from_bits(bits) == 0
}

/// Returns true if num is a positive quiet NaN.
pub const fn f32_is_pos_qnan(num: f32) -> bool {
    f32_is_pos_qnan_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a negative quiet NaN.
pub const fn f32_is_neg_qnan_from_bits(bits: u32) -> bool {
    f32_is_qnan_from_bits(bits) && f32_extract_sign_from_bits(bits) != 0
}

/// Returns true if num is a negative quiet NaN.
pub const fn f32_is_neg_qnan(num: f32) -> bool {
    f32_is_neg_qnan_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a signaling NaN.
pub const fn f32_is_snan_from_bits(bits: u32) -> bool {
    // Signaling NaN: exponent all-ones, quiet bit clear, fraction non-zero.
    f32_is_nan_from_bits(bits) && (bits & FLT_MASK_QUIET_NAN) == 0
}

/// Returns true if num is a signaling NaN.
pub const fn f32_is_snan(num: f32) -> bool {
    f32_is_snan_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a positive signaling NaN.
pub const fn f32_is_pos_snan_from_bits(bits: u32) -> bool {
    f32_is_snan_from_bits(bits) && f32_extract_sign_from_bits(bits) == 0
}

/// Returns true if num is a positive signaling NaN.
pub const fn f32_is_pos_snan(num: f32) -> bool {
    f32_is_pos_snan_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a negative signaling NaN.
pub const fn f32_is_neg_snan_from_bits(bits: u32) -> bool {
    f32_is_snan_from_bits(bits) && f32_extract_sign_from_bits(bits) != 0
}

/// Returns true if num is a negative signaling NaN.
pub const fn f32_is_neg_snan(num: f32) -> bool {
    f32_is_neg_snan_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents plus or minus infinity.
pub const fn f32_is_inf_from_bits(bits: u32) -> bool {
    (bits & FLT_MASK_ABS) == FLT_INF
}

/// Returns true if num is plus or minus infinity.
pub const fn f32_is_inf(num: f32) -> bool {
    f32_is_inf_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents positive infinity.
pub const fn f32_is_pos_inf_from_bits(bits: u32) -> bool {
    bits == FLT_POS_INF
}

/// Returns true if num is positive infinity.
pub const fn f32_is_pos_inf(num: f32) -> bool {
    f32_is_pos_inf_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents negative infinity.
pub const fn f32_is_neg_inf_from_bits(bits: u32) -> bool {
    bits == FLT_NEG_INF
}

/// Returns true if num is negative infinity.
pub const fn f32_is_neg_inf(num: f32) -> bool {
    f32_is_neg_inf_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents plus or minus zero.
pub const fn f32_is_zero_from_bits(bits: u32) -> bool {
    (bits & FLT_MASK_ABS) == 0
}

/// Returns true if num is plus or minus zero.
pub const fn f32_is_zero(num: f32) -> bool {
    f32_is_zero_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a subnormal (denormal).
pub const fn f32_is_subnormal_from_bits(bits: u32) -> bool {
    let exp = f32_extract_biased_exponent_from_bits(bits);
    let frac = f32_extract_fraction_from_bits(bits);
    exp == FLT_EXP_ZERO_SUBNORMAL && frac != 0
}

/// Returns true if num is subnormal (denormal).
pub const fn f32_is_subnormal(num: f32) -> bool {
    f32_is_subnormal_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a normal number.
pub const fn f32_is_normal_from_bits(bits: u32) -> bool {
    let exp = f32_extract_biased_exponent_from_bits(bits);
    exp >= FLT_EXP_BIASED_MIN_NORMAL && exp <= FLT_EXP_BIASED_MAX_NORMAL
}

/// Returns true if num is a normal number.
pub const fn f32_is_normal(num: f32) -> bool {
    f32_is_normal_from_bits(num.to_bits())
}

/// Returns true if the f32 bit pattern represents a finite number
/// (zero, subnormal, or normal -- i.e. not NaN or infinity).
pub const fn f32_is_finite_from_bits(bits: u32) -> bool {
    (bits & FLT_MASK_EXP) != FLT_EXP_INF_NAN
}

/// Returns true if num is finite.
pub const fn f32_is_finite(num: f32) -> bool {
    f32_is_finite_from_bits(num.to_bits())
}

/// Returns true if the sign bit is set (negative).
pub const fn f32_is_sign_negative_from_bits(bits: u32) -> bool {
    f32_extract_sign_from_bits(bits) != 0
}

/// Returns true if num has its sign bit set (negative).
pub const fn f32_is_sign_negative(num: f32) -> bool {
    f32_is_sign_negative_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a NaN.
pub const fn f64_is_nan_from_bits(bits: u64) -> bool {
    f64_extract_biased_exponent_from_bits(bits) == (DBL_EXP_INF_NAN >> DBL_SHIFT_EXP)
        && f64_extract_fraction_from_bits(bits) != 0
}

/// Returns true if num is NaN.
pub const fn f64_is_nan(num: f64) -> bool {
    f64_is_nan_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a quiet NaN.
pub const fn f64_is_qnan_from_bits(bits: u64) -> bool {
    f64_is_nan_from_bits(bits) && (bits & DBL_MASK_QUIET_NAN) != 0
}

/// Returns true if num is a quiet NaN.
pub const fn f64_is_qnan(num: f64) -> bool {
    f64_is_qnan_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a positive quiet NaN.
pub const fn f64_is_pos_qnan_from_bits(bits: u64) -> bool {
    f64_is_qnan_from_bits(bits) && f64_extract_sign_from_bits(bits) == 0
}

/// Returns true if num is a positive quiet NaN.
pub const fn f64_is_pos_qnan(num: f64) -> bool {
    f64_is_pos_qnan_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a negative quiet NaN.
pub const fn f64_is_neg_qnan_from_bits(bits: u64) -> bool {
    f64_is_qnan_from_bits(bits) && f64_extract_sign_from_bits(bits) != 0
}

/// Returns true if num is a negative quiet NaN.
pub const fn f64_is_neg_qnan(num: f64) -> bool {
    f64_is_neg_qnan_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a signaling NaN.
pub const fn f64_is_snan_from_bits(bits: u64) -> bool {
    f64_is_nan_from_bits(bits) && (bits & DBL_MASK_QUIET_NAN) == 0
}

/// Returns true if num is a signaling NaN.
pub const fn f64_is_snan(num: f64) -> bool {
    f64_is_snan_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a positive signaling NaN.
pub const fn f64_is_pos_snan_from_bits(bits: u64) -> bool {
    f64_is_snan_from_bits(bits) && f64_extract_sign_from_bits(bits) == 0
}

/// Returns true if num is a positive signaling NaN.
pub const fn f64_is_pos_snan(num: f64) -> bool {
    f64_is_pos_snan_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a negative signaling NaN.
pub const fn f64_is_neg_snan_from_bits(bits: u64) -> bool {
    f64_is_snan_from_bits(bits) && f64_extract_sign_from_bits(bits) != 0
}

/// Returns true if num is a negative signaling NaN.
pub const fn f64_is_neg_snan(num: f64) -> bool {
    f64_is_neg_snan_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents plus or minus infinity.
pub const fn f64_is_inf_from_bits(bits: u64) -> bool {
    (bits & DBL_MASK_ABS) == DBL_INF
}

/// Returns true if num is plus or minus infinity.
pub const fn f64_is_inf(num: f64) -> bool {
    f64_is_inf_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents positive infinity.
pub const fn f64_is_pos_inf_from_bits(bits: u64) -> bool {
    bits == DBL_POS_INF
}

/// Returns true if num is positive infinity.
pub const fn f64_is_pos_inf(num: f64) -> bool {
    f64_is_pos_inf_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents negative infinity.
pub const fn f64_is_neg_inf_from_bits(bits: u64) -> bool {
    bits == DBL_NEG_INF
}

/// Returns true if num is negative infinity.
pub const fn f64_is_neg_inf(num: f64) -> bool {
    f64_is_neg_inf_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents plus or minus zero.
pub const fn f64_is_zero_from_bits(bits: u64) -> bool {
    (bits & DBL_MASK_ABS) == 0
}

/// Returns true if num is plus or minus zero.
pub const fn f64_is_zero(num: f64) -> bool {
    f64_is_zero_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a subnormal (denormal).
pub const fn f64_is_subnormal_from_bits(bits: u64) -> bool {
    let exp = f64_extract_biased_exponent_from_bits(bits);
    let frac = f64_extract_fraction_from_bits(bits);
    exp == DBL_EXP_ZERO_SUBNORMAL && frac != 0
}

/// Returns true if num is subnormal (denormal).
pub const fn f64_is_subnormal(num: f64) -> bool {
    f64_is_subnormal_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a normal number.
pub const fn f64_is_normal_from_bits(bits: u64) -> bool {
    let exp = f64_extract_biased_exponent_from_bits(bits);
    exp >= DBL_EXP_BIASED_MIN_NORMAL && exp <= DBL_EXP_BIASED_MAX_NORMAL
}

/// Returns true if num is a normal number.
pub const fn f64_is_normal(num: f64) -> bool {
    f64_is_normal_from_bits(num.to_bits())
}

/// Returns true if the f64 bit pattern represents a finite number
/// (zero, subnormal, or normal -- i.e. not NaN or infinity).
pub const fn f64_is_finite_from_bits(bits: u64) -> bool {
    (bits & DBL_MASK_EXP) != DBL_EXP_INF_NAN
}

/// Returns true if num is finite.
pub const fn f64_is_finite(num: f64) -> bool {
    f64_is_finite_from_bits(num.to_bits())
}

/// Returns true if the sign bit is set (negative).
pub const fn f64_is_sign_negative_from_bits(bits: u64) -> bool {
    f64_extract_sign_from_bits(bits) != 0
}

/// Returns true if num has its sign bit set (negative).
pub const fn f64_is_sign_negative(num: f64) -> bool {
    f64_is_sign_negative_from_bits(num.to_bits())
}