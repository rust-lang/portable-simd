#![feature(portable_simd)]
#![feature(f16)]

#[macro_use]
mod ops_macros;
impl_float_tests! { f16, i16 }
