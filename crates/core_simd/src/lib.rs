#![cfg_attr(not(feature = "std"), no_std)]
#![feature(
    const_fn_trait_bound,
    const_panic,
    platform_intrinsics,
    repr_simd,
    simd_ffi,
    staged_api,
    stdsimd
)]
#![cfg_attr(feature = "generic_const_exprs", feature(generic_const_exprs))]
#![cfg_attr(feature = "generic_const_exprs", allow(incomplete_features))]
#![warn(missing_docs)]
#![unstable(feature = "portable_simd", issue = "86656")]
//! Portable SIMD module.

#[path = "mod.rs"]
mod core_simd;
pub use self::core_simd::simd;
pub use simd::*;
