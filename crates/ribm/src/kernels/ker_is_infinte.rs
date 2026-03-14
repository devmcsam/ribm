use crate::helpers::classification::{f32_is_inf_from_bits, f32_is_inf, f64_is_inf_from_bits, f64_is_inf};

/// Returns true if the bit pattern of the number represents 32 bit floating point infinity.
pub const fn ker_f32_is_inf_from_bits(bits: u32) -> bool {
    f32_is_inf_from_bits(bits)
}

/// Returns true if the number represents 32 bit floating point infinity.
pub const fn ker_f32_is_inf(num: f32) -> bool {
    f32_is_inf(num)
}

/// Returns true if the bit pattern of the number represents 64 bit floating point infinity.
pub const fn ker_f64_is_inf_from_bits(bits: u64) -> bool {
    f64_is_inf_from_bits(bits)
}

/// Returns true if the number represents 64 bit floating point infinity.
pub const fn ker_f64_is_inf(num: f64) -> bool {
    f64_is_inf(num)
}