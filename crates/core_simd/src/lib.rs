#![cfg_attr(not(feature = "std"), no_std)]
#![feature( // rustc internals
    platform_intrinsics,
    repr_simd,
    staged_api,
)]
#![feature( // lang features
    const_fn_trait_bound,
    decl_macro,
    simd_ffi,
)]
#![feature( // library features
    maybe_uninit_slice,
    maybe_uninit_array_assume_init,
    maybe_uninit_uninit_array,
    stdsimd,
)]
#![cfg_attr(feature = "generic_const_exprs", feature(generic_const_exprs))]
#![cfg_attr(feature = "generic_const_exprs", allow(incomplete_features))]
#![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![unstable(feature = "portable_simd", issue = "86656")]
//! Portable SIMD module.

#[path = "mod.rs"]
mod core_simd;
pub use self::core_simd::simd;
pub use simd::*;
