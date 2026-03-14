use crate::helpers::classification::{f32_is_nan, f32_is_nan_from_bits, f64_is_nan, f64_is_nan_from_bits};

pub const fn ker_f32_is_nan_from_bits(bits: u32) -> bool {
    f32_is_nan_from_bits(bits)
}

pub const fn ker_f32_is_nan(num: f32) -> bool {
    f32_is_nan(num)
}

pub const fn ker_f64_is_nan_from_bits(bits: u64) -> bool {
    f64_is_nan_from_bits(bits)
}

pub const fn ker_f64_is_nan(num: f64) -> bool {
    f64_is_nan(num)
}