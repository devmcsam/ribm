use crate::helpers::classification::{f32_is_normal_from_bits, f32_is_normal, f64_is_normal_from_bits, f64_is_normal};

/// Returns true if the bit pattern of the number represents a normal floating point number.
pub const fn ker_f32_is_normal_from_bits(bits: u32) -> bool {
    f32_is_normal_from_bits(bits)
}

/// Returns true if the number represents a normal floating point number.
pub const fn ker_f32_is_normal(num: f32) -> bool {
    f32_is_normal(num)
}

/// Returns true if the bit pattern of the number represents a normal floating point number.
pub const fn ker_f64_is_normal_from_bits(bits: u64) -> bool {
    f64_is_normal_from_bits(bits)
}

/// Returns true if the number represents a normal floating point number.
pub const fn ker_f64_is_normal(num: f64) -> bool {
    f64_is_normal(num)
}