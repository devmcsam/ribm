use crate::helpers::extraction::*;

pub const fn ker_f32_ilogb_bits(num: u32) -> i32 {
    f32_extract_unbiased_exponent_from_bits(num)
}

pub const fn ker_f32_ilogb(num: f32) -> i32 {
    f32_extract_unbiased_exponent(num)
}

pub const fn ker_f64_ilogb_bits(num: u64) -> i32 {
    f64_extract_unbiased_exponent_from_bits(num)
}

pub const fn ker_f64_ilogb(num: f64) -> i32 {
    f64_extract_unbiased_exponent(num)
}