use crate::helpers::extraction::{f32_extract_unbiased_exponent_from_bits, f32_extract_fraction_from_bits,
                                 f32_extract_sign_from_bits, f64_extract_unbiased_exponent_from_bits,
                                 f64_extract_fraction_from_bits, f64_extract_sign_from_bits};
use crate::helpers::creation::{u32_create_from_bits, u64_create_from_bits};

use crate::helpers::constants::{FLT_EXP_BIAS, DBL_EXP_BIAS};

pub const fn ker_f32_frexp_bits(num: u32) -> (u32, i32) {
    let exp = f32_extract_unbiased_exponent_from_bits(num);
    let frac = f32_extract_fraction_from_bits(num);
    let sign = f32_extract_sign_from_bits(num);

    let out = u32_create_from_bits(sign, FLT_EXP_BIAS - 1, frac);
    (out, exp + 1)
}

pub const fn ker_f32_frexp(num: f32) -> (f32, i32) {
    let (bits, exp) = ker_f32_frexp_bits(num.to_bits());
    (f32::from_bits(bits), exp)
}

pub const fn ker_f64_frexp_bits(num: u64) -> (u64, i32) {
    let exp = f64_extract_unbiased_exponent_from_bits(num);
    let frac = f64_extract_fraction_from_bits(num);
    let sign = f64_extract_sign_from_bits(num);
    
    let out_bits = u64_create_from_bits(sign, (DBL_EXP_BIAS - 1) as u64, frac);

    (out_bits, exp + 1)
}

pub const fn ker_f64_frexp(num: f64) -> (f64, i32) {
    let (bits, exp) = ker_f64_frexp_bits(num.to_bits());
    (f64::from_bits(bits), exp)
}