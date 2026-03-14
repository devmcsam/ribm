use crate::helpers::extraction::{f32_extract_sign_from_bits, f64_extract_sign_from_bits};

pub const fn ker_f32_signbit_from_bits(bits: u32) -> bool {
    f32_extract_sign_from_bits(bits) != 0
}

pub const fn ker_f32_signbit(num: f32) -> bool {
    ker_f32_signbit_from_bits(num.to_bits())
}

pub const fn ker_f64_signbit_from_bits(bits: u64) -> bool {
    f64_extract_sign_from_bits(bits) != 0
}

pub const fn ker_f64_signbit(num: f64) -> bool {
    ker_f64_signbit_from_bits(num.to_bits())
}