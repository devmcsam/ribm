use crate::helpers::extraction::{f32_extract_biased_exponent_from_bits, f32_extract_fraction_from_bits,
                                 f32_extract_sign_from_bits, f64_extract_biased_exponent_from_bits,
                                 f64_extract_fraction_from_bits, f64_extract_sign_from_bits};
use crate::helpers::creation::{u32_create_from_bits, u64_create_from_bits};

/// The kernel bit version of the scalbn function for f32.
pub const fn ker_f32_scalbn_bits(num: u32, n: i32) -> u32 {
    let mut exp = f32_extract_biased_exponent_from_bits(num);
    exp += n as u32;
    let frac = f32_extract_fraction_from_bits(num);
    let sign = f32_extract_sign_from_bits(num);
    u32_create_from_bits(sign, exp, frac)
}

/// The kernel version of the scalbn function for f32.
pub const fn ker_f32_scalbn(num: f32, n: i32) -> f32 {
    f32::from_bits(ker_f32_scalbn_bits(num.to_bits(), n))
}

/// The kernel bit version of the scalbn function for f64.
pub const fn ker_f64_scalbn_bits(num: u64, n: i32) -> u64 {
    let mut exp = f64_extract_biased_exponent_from_bits(num);
    exp += n as u64;
    let frac = f64_extract_fraction_from_bits(num);
    let sign = f64_extract_sign_from_bits(num);
    u64_create_from_bits(sign, exp, frac)
}

/// The kernel version of the scalbn function for f64.
pub const fn ker_f64_scalbn(num: f64, n: i32) -> f64 {
    f64::from_bits(ker_f64_scalbn_bits(num.to_bits(), n))
}