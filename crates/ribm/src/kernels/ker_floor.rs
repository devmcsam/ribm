use crate::helpers::constants::{FLT_BITS_FRAC, FLT_POS_ZERO, FLT_NEG_ONE, FLT_MASK_FRAC, DBL_BITS_FRAC,
                                DBL_POS_ZERO, DBL_NEG_ONE, DBL_MASK_FRAC};
use crate::helpers::extraction::{f32_extract_sign_from_bits, f32_extract_unbiased_exponent_from_bits,
                                 f64_extract_sign_from_bits, f64_extract_unbiased_exponent_from_bits};

pub const fn ker_f32_floor_bits(num: u32) -> u32 {
    let sign = f32_extract_sign_from_bits(num);
    let uexp = f32_extract_unbiased_exponent_from_bits(num);

    if uexp >= FLT_BITS_FRAC as i32 {
        // All significand bits represent integer value; already an integer.
        return num;
    }

    if uexp < 0 {
        // |num| < 1.0
        if sign == 0 {
            // pos so +0.0
            return FLT_POS_ZERO;
        }
        // neg so -1.0
        return FLT_NEG_ONE;
    }

    // 0 <= uexp < 23, mask off the fractional bits
    let frac_mask: u32 = FLT_MASK_FRAC >> uexp as u32;
    let frac_bits = num & frac_mask;

    if frac_bits == 0 {
        // Already an integer
        return num;
    }

    // Truncate the fractional bits
    let truncated = num & !frac_mask;

    if sign != 0 {
        // is neg, floor rounds towards neg inf, so add 1 ULP at the integer
        // granularity. The integer-LSB sits at the position of the lowest kept
        // bit, which is (frac_mask + 1).
        truncated + frac_mask + 1
    } else {
        // is pos so truncates
        truncated
    }
}

pub const fn ker_f32_floor(num: f32) -> f32 {
    f32::from_bits(ker_f32_floor_bits(num.to_bits()))
}

pub const fn ker_f64_floor_bits(num: u64) -> u64 {
    let sign = f64_extract_sign_from_bits(num);

    // Unbiased exponent
    let uexp = f64_extract_unbiased_exponent_from_bits(num);

    if uexp >= DBL_BITS_FRAC as i32 {
        // All significand bits represent integer value; already an integer.
        return num;
    }

    if uexp < 0 {
        // |num| < 1.0
        if sign == 0 {
            // pos so +0.0
            return DBL_POS_ZERO;
        }
        // neg so -1.0
        return DBL_NEG_ONE;
    }

    // 0 <= uexp < 52, mask off the fractional bits
    let frac_mask: u64 = DBL_MASK_FRAC >> uexp as u64;
    let frac_bits = num & frac_mask;

    if frac_bits == 0 {
        // Already an integer
        return num;
    }

    // Truncate the fractional bits
    let truncated = num & !frac_mask;

    if sign != 0 {
        // floor rounds to neg infinity, so add 1 ULP at the integer
        // granularity.
        truncated + frac_mask + 1
    } else {
        // is positive so just truncates
        truncated
    }
}

pub const fn ker_f64_floor(num: f64) -> f64 {
    f64::from_bits(ker_f64_floor_bits(num.to_bits()))
}