#![feature(portable_simd)]

#[macro_use]
mod ops_macros;
impl_float_tests! { SimdF64, f64, i64 }
