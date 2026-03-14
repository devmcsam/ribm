//! Bit-level utility constants for IEEE-754 floating-point types.

/// Shift count needed to move the sign bit of an f32 to bit 0.
pub const FLT_SHIFT_SIGN: u32 = 0x1f;

/// Shift count needed to move the exponent field of an f32 to bit 0.
pub const FLT_SHIFT_EXP: u32 = 0x17;

/// Number of fraction (trailing significand) bits in an f32.
pub const FLT_BITS_FRAC: u32 = 0x17;

/// Number of exponent bits in an f32.
pub const FLT_BITS_EXP: u32 = 0x08;

/// Total number of bits in an f32.
pub const FLT_BITS: u32 = 0x20;

/// Exponent bias for an f32 (127).
pub const FLT_EXP_BIAS: u32 = 0x7f;

/// Mask selecting the sign bit of an f32.
pub const FLT_MASK_SIGN: u32 = 0x8000_0000;

/// Mask selecting the exponent field of an f32.
pub const FLT_MASK_EXP: u32 = 0x7f80_0000;

/// Mask selecting the fraction field of an f32.
pub const FLT_MASK_FRAC: u32 = 0x007f_ffff;

/// Mask selecting all non-sign bits of an f32 (magnitude).
pub const FLT_MASK_ABS: u32 = 0x7fff_ffff;

/// Mask selecting only the sign and exponent bits of an f32.
pub const FLT_MASK_SIGN_EXP: u32 = 0xff80_0000;

/// Mask selecting the NaN payload bits of an f32 (same as fraction mask).
pub const FLT_MASK_NAN_PAYLOAD: u32 = 0x003f_ffff;

/// Mask selecting the quiet-NaN indicator bit of an f32 fraction field.
pub const FLT_MASK_QUIET_NAN: u32 = 0x0040_0000;

/// Bit pattern of the implicit leading 1 in a normalized f32 significand.
pub const FLT_HIDDEN_BIT: u32 = 0x0080_0000;

/// Exponent field value for an f32 zero or subnormal number (biased exponent = 0).
pub const FLT_EXP_ZERO_SUBNORMAL: u32 = 0x0000_0000;

/// Exponent field value for an f32 infinity or NaN (biased exponent = 255).
pub const FLT_EXP_INF_NAN: u32 = 0x7f80_0000;

/// Minimum biased exponent for a normal f32 (1).
pub const FLT_EXP_BIASED_MIN_NORMAL: u32 = 0x01;

/// Maximum biased exponent for a normal f32 (254).
pub const FLT_EXP_BIASED_MAX_NORMAL: u32 = 0xfe;

/// Minimum unbiased exponent for a normal f32 (-126).
pub const FLT_EXP_UNBIASED_MIN_NORMAL: i32 = -126;

/// Maximum unbiased exponent for a normal f32 (127).
pub const FLT_EXP_UNBIASED_MAX_NORMAL: i32 = 127;

/// Unbiased exponent assigned to f32 subnormals by IEEE 754 (-126, same as
/// the minimum normal exponent -- the subnormal simply lacks the hidden bit).
pub const FLT_EXP_SUBNORMAL: i32 = -126;

/// Bit pattern of positive infinity for an f32.
pub const FLT_POS_INF: u32 = 0x7f80_0000;

/// Bit pattern of negative infinity for an f32.
pub const FLT_NEG_INF: u32 = 0xff80_0000;

/// Sign-agnostic bit pattern of infinity for an f32.
pub const FLT_INF: u32 = 0x7f80_0000;

/// Bit pattern of positive zero for an f32.
pub const FLT_POS_ZERO: u32 = 0x0000_0000;

/// Bit pattern of negative zero for an f32.
pub const FLT_NEG_ZERO: u32 = 0x8000_0000;

/// Smallest sign-agnostic bit pattern representing an f32 NaN (signaling NaN).
pub const FLT_NAN_MIN: u32 = 0x7f80_0001;

/// Canonical quiet NaN for an f32 (positive, quiet bit set, zero payload).
pub const FLT_QUIET_NAN: u32 = 0x7fc0_0000;

/// Bit pattern of the smallest positive normal f32 (~1.175494e-38).
pub const FLT_SMALLEST_NORMAL: u32 = 0x0080_0000;

/// Bit pattern of the largest positive finite f32 (~3.402823e+38).
pub const FLT_LARGEST_NORMAL: u32 = 0x7f7f_ffff;

/// Bit pattern of the smallest positive subnormal f32 (~1.401298e-45).
pub const FLT_SMALLEST_SUBNORMAL: u32 = 0x0000_0001;

/// Bit pattern of the largest positive subnormal f32 (~1.175494e-38 - ULP).
pub const FLT_LARGEST_SUBNORMAL: u32 = 0x007f_ffff;

/// Bit pattern of 1.0_f32.
pub const FLT_ONE: u32 = 0x3f80_0000;

/// Bit pattern of -1.0_f32.
pub const FLT_NEG_ONE: u32 = 0xbf80_0000;

/// Bit pattern of 0.5_f32.
pub const FLT_HALF: u32 = 0x3f00_0000;

/// Bit pattern of 2.0_f32.
pub const FLT_TWO: u32 = 0x4000_0000;

/// Shift count needed to move the sign bit of an f64 to bit 0.
pub const DBL_SHIFT_SIGN: u32 = 0x3f;

/// Shift count needed to move the exponent field of an f64 to bit 0.
pub const DBL_SHIFT_EXP: u32 = 0x34;

/// Number of fraction (trailing significand) bits in an f64.
pub const DBL_BITS_FRAC: u32 = 0x34;

/// Number of exponent bits in an f64.
pub const DBL_BITS_EXP: u32 = 0x0b;

/// Total number of bits in an f64.
pub const DBL_BITS: u32 = 0x40;

/// Exponent bias for an f64 (1023).
pub const DBL_EXP_BIAS: u32 = 0x3ff;

/// Mask selecting the sign bit of an f64.
pub const DBL_MASK_SIGN: u64 = 0x8000_0000_0000_0000;

/// Mask selecting the exponent field of an f64.
pub const DBL_MASK_EXP: u64 = 0x7ff0_0000_0000_0000;

/// Mask selecting the fraction field of an f64.
pub const DBL_MASK_FRAC: u64 = 0x000f_ffff_ffff_ffff;

/// Mask selecting all non-sign bits of an f64 (magnitude).
pub const DBL_MASK_ABS: u64 = 0x7fff_ffff_ffff_ffff;

/// Mask selecting only the sign and exponent bits of an f64.
pub const DBL_MASK_SIGN_EXP: u64 = 0xfff0_0000_0000_0000;

/// Mask selecting the NaN payload bits of an f64 (quiet bit excluded).
pub const DBL_MASK_NAN_PAYLOAD: u64 = 0x0007_ffff_ffff_ffff;

/// Mask selecting the quiet-NaN indicator bit of an f64 fraction field.
pub const DBL_MASK_QUIET_NAN: u64 = 0x0008_0000_0000_0000;

/// Bit pattern of the implicit leading 1 in a normalized f64 significand.
pub const DBL_HIDDEN_BIT: u64 = 0x0010_0000_0000_0000;

/// Exponent field value for an f64 zero or subnormal number (biased exponent = 0).
pub const DBL_EXP_ZERO_SUBNORMAL: u64 = 0x0000_0000_0000_0000;

/// Exponent field value for an f64 infinity or NaN (biased exponent = 2047).
pub const DBL_EXP_INF_NAN: u64 = 0x7ff0_0000_0000_0000;

/// Minimum biased exponent for a normal f64 (1).
pub const DBL_EXP_BIASED_MIN_NORMAL: u64 = 0x001;

/// Maximum biased exponent for a normal f64 (2046).
pub const DBL_EXP_BIASED_MAX_NORMAL: u64 = 0x7fe;

/// Minimum unbiased exponent for a normal f64 (-1022).
pub const DBL_EXP_UNBIASED_MIN_NORMAL: i32 = -1022;

/// Maximum unbiased exponent for a normal f64 (1023).
pub const DBL_EXP_UNBIASED_MAX_NORMAL: i32 = 1023;

/// Unbiased exponent assigned to f64 subnormals by IEEE 754 (-1022).
pub const DBL_EXP_SUBNORMAL: i32 = -1022;

/// Bit pattern of positive infinity for an f64.
pub const DBL_POS_INF: u64 = 0x7ff0_0000_0000_0000;

/// Bit pattern of negative infinity for an f64.
pub const DBL_NEG_INF: u64 = 0xfff0_0000_0000_0000;

/// Sign-agnostic bit pattern of infinity for an f64.
pub const DBL_INF: u64 = 0x7ff0_0000_0000_0000;

/// Bit pattern of positive zero for an f64.
pub const DBL_POS_ZERO: u64 = 0x0000_0000_0000_0000;

/// Bit pattern of negative zero for an f64.
pub const DBL_NEG_ZERO: u64 = 0x8000_0000_0000_0000;

/// Smallest sign-agnostic bit pattern representing an f64 NaN (signaling NaN).
pub const DBL_NAN_MIN: u64 = 0x7ff0_0000_0000_0001;

/// Canonical quiet NaN for an f64 (positive, quiet bit set, zero payload).
pub const DBL_QUIET_NAN: u64 = 0x7ff8_0000_0000_0000;

/// Bit pattern of the smallest positive normal f64 (~2.2250738585072014e-308).
pub const DBL_SMALLEST_NORMAL: u64 = 0x0010_0000_0000_0000;

/// Bit pattern of the largest positive finite f64 (~1.7976931348623157e+308).
pub const DBL_LARGEST_NORMAL: u64 = 0x7fef_ffff_ffff_ffff;

/// Bit pattern of the smallest positive subnormal f64 (~5e-324).
pub const DBL_SMALLEST_SUBNORMAL: u64 = 0x0000_0000_0000_0001;

/// Bit pattern of the largest positive subnormal f64.
pub const DBL_LARGEST_SUBNORMAL: u64 = 0x000f_ffff_ffff_ffff;

/// Bit pattern of 1.0_f64.
pub const DBL_ONE: u64 = 0x3ff0_0000_0000_0000;

/// Bit pattern of -1.0_f64.
pub const DBL_NEG_ONE: u64 = 0xbff0_0000_0000_0000;

/// Bit pattern of 0.5_f64.
pub const DBL_HALF: u64 = 0x3fe0_0000_0000_0000;

/// Bit pattern of 2.0_f64.
pub const DBL_TWO: u64 = 0x4000_0000_0000_0000;

/// PI
pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

/// PI / 2
pub const FRAC_PI_2: f64 = 1.57079632679489661923132169163975144_f64;

/// PI / 4
pub const FRAC_PI_4: f64 = 0.785398163397448309615660845819875721_f64;

/// 1 / PI
pub const FRAC_1_PI: f64 = 0.318309886183790671537767526745028724_f64;

/// 2 / PI
pub const FRAC_2_PI: f64 = 0.636619772367581343075535053490057448_f64;

/// 2 / sqrt(PI)
pub const FRAC_2_SQRT_PI: f64 = 1.12837916709551257389615890312154517_f64;

/// sqrt(2)
pub const SQRT_2: f64 = 1.41421356237309504880168872420969808_f64;

/// 1 / sqrt(2)  (= sqrt(2) / 2)
pub const FRAC_1_SQRT_2: f64 = 0.707106781186547524400844362104849039_f64;

/// e  (Euler's number)
pub const E: f64 = 2.71828182845904523536028747135266250_f64;

/// log2(e)
pub const LOG2_E: f64 = 1.44269504088896340735992468100189214_f64;

/// log10(e)
pub const LOG10_E: f64 = 0.434294481903251827651128918916605082_f64;

/// ln(2)
pub const LN_2: f64 = 0.693147180559945309417232121458176568_f64;

/// ln(10)
pub const LN_10: f64 = 2.30258509299404568401799145468436421_f64;

/// log10(2)
pub const LOG10_2: f64 = 0.301029995663981195213738894947040750_f64;

/// log2(10)
pub const LOG2_10: f64 = 3.32192809488736234787031942948939018_f64;

/// PI (f32)
pub const PI_F32: f32 = 3.14159265358979323846_f32;

/// PI / 2 (f32)
pub const FRAC_PI_2_F32: f32 = 1.57079632679489661923_f32;

/// PI / 4 (f32)
pub const FRAC_PI_4_F32: f32 = 0.785398163397448309616_f32;

/// 1 / PI (f32)
pub const FRAC_1_PI_F32: f32 = 0.318309886183790671538_f32;

/// 2 / PI (f32)
pub const FRAC_2_PI_F32: f32 = 0.636619772367581343076_f32;

/// 2 / sqrt(PI) (f32)
pub const FRAC_2_SQRT_PI_F32: f32 = 1.12837916709551257390_f32;

/// sqrt(2) (f32)
pub const SQRT_2_F32: f32 = 1.41421356237309504880_f32;

/// 1 / sqrt(2) (f32)
pub const FRAC_1_SQRT_2_F32: f32 = 0.707106781186547524401_f32;

/// e (f32)
pub const E_F32: f32 = 2.71828182845904523536_f32;

/// log2(e) (f32)
pub const LOG2_E_F32: f32 = 1.44269504088896340736_f32;

/// log10(e) (f32)
pub const LOG10_E_F32: f32 = 0.434294481903251827651_f32;

/// ln(2) (f32)
pub const LN_2_F32: f32 = 0.693147180559945309417_f32;

/// ln(10) (f32)
pub const LN_10_F32: f32 = 2.30258509299404568402_f32;