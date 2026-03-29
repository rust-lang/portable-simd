#![feature(portable_simd)]
#![feature(f16)]

#[macro_use]
mod ops_macros;

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
impl_float_tests! { f16, i16 }
