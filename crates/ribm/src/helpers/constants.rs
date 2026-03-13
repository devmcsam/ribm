//! Bit-level utility constants for IEEE-754 floating-point types.

/// Shift count needed to move the sign bit of an `f32` to bit 0.
pub const FLT_SHIFT_SIGN: u32 = 0x1f;

/// Shift count needed to move the exponent field of an `f32` to bit 0.
pub const FLT_SHIFT_EXP: u32 = 0x17;

/// Number of fraction bits in an `f32`.
pub const FLT_BITS_FRAC: u32 = 0x17;

/// Number of exponent bits in an `f32`.
pub const FLT_BITS_EXP: u32 = 0x08;

/// Total number of bits in an `f32`.
pub const FLT_BITS: u32 = 0x20;

/// Exponent bias for an `f32`.
pub const FLT_EXP_BIAS: u32 = 0x7f;

/// Mask selecting the sign bit of an `f32`.
pub const FLT_MASK_SIGN: u32 = 0x8000_0000;

/// Mask selecting the exponent field of an `f32`.
pub const FLT_MASK_EXP: u32 = 0x7f80_0000;

/// Mask selecting the fraction field of an `f32`.
pub const FLT_MASK_FRAC: u32 = 0x007f_ffff;

/// Mask selecting all non-sign bits of an `f32`.
pub const FLT_MASK_ABS: u32 = 0x7fff_ffff;

/// Mask selecting only the sign and exponent bits of an `f32`.
pub const FLT_MASK_SIGN_EXP: u32 = 0xff80_0000;

/// Mask selecting the NaN payload bits of an `f32`.
pub const FLT_MASK_NAN_PAYLOAD: u32 = 0x007f_ffff;

/// Mask selecting the quiet-NaN indicator bit of an `f32` fraction field.
pub const FLT_MASK_QUIET_NAN: u32 = 0x0040_0000;

/// Bit pattern of the implicit leading 1 in a normalized `f32` significand.
pub const FLT_HIDDEN_BIT: u32 = 0x0080_0000;

/// Exponent field value for an `f32` zero or subnormal number.
pub const FLT_EXP_ZERO_SUBNORMAL: u32 = 0x0000_0000;

/// Exponent field value for an `f32` infinity or NaN.
pub const FLT_EXP_INF_NAN: u32 = 0x7f80_0000;

/// Bit pattern of positive infinity for an `f32`.
pub const FLT_POS_INF: u32 = 0x7f80_0000;

/// Bit pattern of negative infinity for an `f32`.
pub const FLT_NEG_INF: u32 = 0xff80_0000;

/// Sign-agnostic bit pattern of infinity for an `f32`.
pub const FLT_INF: u32 = 0x7f80_0000;

/// Smallest sign-agnostic bit pattern representing an `f32` NaN.
pub const FLT_NAN_MIN: u32 = 0x7f80_0001;

/// Shift count needed to move the sign bit of an `f64` to bit 0.
pub const DBL_SHIFT_SIGN: u32 = 0x3f;

/// Shift count needed to move the exponent field of an `f64` to bit 0.
pub const DBL_SHIFT_EXP: u32 = 0x34;

/// Number of fraction bits in an `f64`.
pub const DBL_BITS_FRAC: u32 = 0x34;

/// Number of exponent bits in an `f64`.
pub const DBL_BITS_EXP: u32 = 0x0b;

/// Total number of bits in an `f64`.
pub const DBL_BITS: u32 = 0x40;

/// Exponent bias for an `f64`.
pub const DBL_EXP_BIAS: u32 = 0x3ff;

/// Mask selecting the sign bit of an `f64`.
pub const DBL_MASK_SIGN: u64 = 0x8000_0000_0000_0000;

/// Mask selecting the exponent field of an `f64`.
pub const DBL_MASK_EXP: u64 = 0x7ff0_0000_0000_0000;

/// Mask selecting the fraction field of an `f64`.
pub const DBL_MASK_FRAC: u64 = 0x000f_ffff_ffff_ffff;

/// Mask selecting all non-sign bits of an `f64`.
pub const DBL_MASK_ABS: u64 = 0x7fff_ffff_ffff_ffff;

/// Mask selecting only the sign and exponent bits of an `f64`.
pub const DBL_MASK_SIGN_EXP: u64 = 0xfff0_0000_0000_0000;

/// Mask selecting the NaN payload bits of an `f64`.
pub const DBL_MASK_NAN_PAYLOAD: u64 = 0x000f_ffff_ffff_ffff;

/// Mask selecting the quiet-NaN indicator bit of an `f64` fraction field.
pub const DBL_MASK_QUIET_NAN: u64 = 0x0008_0000_0000_0000;

/// Bit pattern of the implicit leading 1 in a normalized `f64` significand.
pub const DBL_HIDDEN_BIT: u64 = 0x0010_0000_0000_0000;

/// Exponent field value for an `f64` zero or subnormal number.
pub const DBL_EXP_ZERO_SUBNORMAL: u64 = 0x0000_0000_0000_0000;

/// Exponent field value for an `f64` infinity or NaN.
pub const DBL_EXP_INF_NAN: u64 = 0x7ff0_0000_0000_0000;

/// Bit pattern of positive infinity for an `f64`.
pub const DBL_POS_INF: u64 = 0x7ff0_0000_0000_0000;

/// Bit pattern of negative infinity for an `f64`.
pub const DBL_NEG_INF: u64 = 0xfff0_0000_0000_0000;

/// Sign-agnostic bit pattern of infinity for an `f64`.
pub const DBL_INF: u64 = 0x7ff0_0000_0000_0000;

/// Smallest sign-agnostic bit pattern representing an `f64` NaN.
pub const DBL_NAN_MIN: u64 = 0x7ff0_0000_0000_0001;