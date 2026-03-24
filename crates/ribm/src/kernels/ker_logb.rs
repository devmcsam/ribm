use crate::helpers::extraction::*;

pub const fn ker_f32_logb_bits(num: u32) -> i32 {
    f32_extract_unbiased_exponent_from_bits(num)
}

pub const fn ker_f32_logb(num: f32) -> f32 {
    f32_extract_unbiased_exponent(num) as f32
}

pub const fn ker_f64_logb_bits(num: u64) -> i32 {
    f64_extract_unbiased_exponent_from_bits(num)
}

pub const fn ker_f64_logb(num: f64) -> f64 {
    f64_extract_unbiased_exponent(num) as f64
}