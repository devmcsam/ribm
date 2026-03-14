use super::constants::{FLT_SHIFT_SIGN, FLT_MASK_EXP, FLT_SHIFT_EXP, FLT_MASK_FRAC, DBL_SHIFT_SIGN,
                       DBL_MASK_EXP, DBL_SHIFT_EXP, DBL_MASK_FRAC,
};

/// Creates a `u32` that would represent an `f32` from the given sign, exp, and frac
pub const fn u32_create_from_bits(sign: u32, exp: u32, frac: u32) -> u32 {
    ((sign & 1) << FLT_SHIFT_SIGN)
        | ((exp & FLT_MASK_EXP) << FLT_SHIFT_EXP)
        | (frac & FLT_MASK_FRAC)
}

/// Creates a u64 that would represent an `f64` from the given sign, exp, and frac
pub const fn u64_create_from_bits(sign: u64, exp: u64, frac: u64) -> u64 {
    ((sign & 1) << DBL_SHIFT_SIGN)
        | ((exp & DBL_MASK_EXP) << DBL_SHIFT_EXP)
        | (frac & DBL_MASK_FRAC)
}

/// Creates an `f32` from the given sign, exp, and frac
pub const fn f32_create_from_bits(sign: u32, exp: u32, frac: u32) -> f32 {
    f32::from_bits(u32_create_from_bits(sign, exp, frac))
}

/// Creates an `f64` from the given sign, exp, and frac
pub const fn f64_create_from_bits(sign: u64, exp: u64, frac: u64) -> f64 {
    f64::from_bits(u64_create_from_bits(sign, exp, frac))
}