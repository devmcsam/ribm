use crate::helpers::constants::{
    DBL_EXP_INF_NAN, DBL_MASK_ABS, DBL_SMALLEST_NORMAL, FLT_EXP_INF_NAN, FLT_MASK_ABS,
    FLT_SMALLEST_NORMAL,
};

// Avoid generic classification helpers in kernels.
// They often recompute masks/shifts or extract fields that are already
// available, introducing redundant instructions or extra branches.
// When it does not effect performance, it is better to use the classify functions.
// However in this case it does affect performance.

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FpCategory {
    Normal,
    Subnormal,
    Zero,
    Infinite,
    Nan,
}

// Note: There also is a branchless way to do this.
// However, since normal is almost always the case for this function,
// branch predictors will predict normal correctly nearly every time.
// Also if LLVM decides that branchless is faster on a certain architecture it will optimize it to that anyway.

pub const fn ker_f32_classify_from_bits(bits: u32) -> FpCategory {
    let abs_bits = bits & FLT_MASK_ABS;

    if abs_bits == 0 {
        FpCategory::Zero
    } else if abs_bits < FLT_SMALLEST_NORMAL {
        FpCategory::Subnormal
    } else if abs_bits < FLT_EXP_INF_NAN {
        FpCategory::Normal
    } else if abs_bits == FLT_EXP_INF_NAN {
        FpCategory::Infinite
    } else {
        FpCategory::Nan
    }
}

pub const fn ker_f32_classify(num: f32) -> FpCategory {
    ker_f32_classify_from_bits(num.to_bits())
}

pub const fn ker_f64_classify_from_bits(bits: u64) -> FpCategory {
    let abs_bits = bits & DBL_MASK_ABS;

    if abs_bits == 0 {
        FpCategory::Zero
    } else if abs_bits < DBL_SMALLEST_NORMAL {
        FpCategory::Subnormal
    } else if abs_bits < DBL_EXP_INF_NAN {
        FpCategory::Normal
    } else if abs_bits == DBL_EXP_INF_NAN {
        FpCategory::Infinite
    } else {
        FpCategory::Nan
    }
}

pub const fn ker_f64_classify(num: f64) -> FpCategory {
    ker_f64_classify_from_bits(num.to_bits())
}