//! The kernels for functions in the Ribm library
//!
//! We define a kernel as the core happy path algorithm,
//! this means that it assumes that inputs are within range, valid, and thus it performs little to
//! no branching.

pub mod ker_copysign;
pub mod ker_signbit;
pub mod ker_classify;
pub mod ker_is_nan;
pub mod ker_is_infinte;
pub mod ker_is_finite;
pub mod ker_is_normal;
pub mod ker_scalbn;
pub mod ker_ldexp;
pub mod ker_frexp;
pub mod ker_logb;
pub mod ker_ilogb;

pub use ker_copysign::*;
pub use ker_signbit::*;
pub use ker_classify::*;
pub use ker_is_nan::*;
pub use ker_is_infinte::*;
pub use ker_is_finite::*;
pub use ker_is_normal::*;
pub use ker_scalbn::*;
pub use ker_ldexp::*;
pub use ker_frexp::*;
pub use ker_logb::*;
pub use ker_ilogb::*;