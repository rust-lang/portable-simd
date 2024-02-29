#![feature(test)]
#![feature(portable_simd)]
#![feature(concat_idents)]

extern crate test;
use std_float::StdLibm;

use test::{black_box, Bencher};

use core_simd::{f32x16, f32x4, f64x4, f64x8};

const N: usize = 1024;

fn init_f32x4() -> Vec<f32x4> {
    vec![f32x4::splat(black_box(0.5)); N / 4]
}

fn init_f32x16() -> Vec<f32x16> {
    vec![f32x16::splat(black_box(0.5)); N / 16]
}

fn init_f32() -> Vec<f32> {
    vec![black_box(0.5); N]
}

fn init_f64x4() -> Vec<f64x4> {
    vec![f64x4::splat(black_box(0.5)); N / 4]
}

fn init_f64x8() -> Vec<f64x8> {
    vec![f64x8::splat(black_box(0.5)); N / 8]
}

fn init_f64() -> Vec<f64> {
    vec![black_box(1.0); N]
}

// These fuctions are not inlined to make it easier to check the asm.
//
// Build with:
//
// RUSTFLAGS="-C target-cpu=native --emit asm" cargo bench

macro_rules! benchmark_libm {
    (
        functions ($(
            $names : ident,
            $functions : expr,
            $init : expr
        )*)
    ) => {

        $(
            #[bench]
            #[inline(never)]
            fn $names(b: &mut Bencher) {
                let x = $init;
                let mut y = $init;
                b.iter(|| {
                    for (x, y) in x.iter().zip(y.iter_mut()) {
                        *y = ($functions)(*x);
                    }
                })
            }
        )*
    }
}

benchmark_libm! {
    functions (
        sin_f32x4,  |x : f32x4| x.sin(),  init_f32x4()
        sin_f32x16, |x : f32x16| x.sin(), init_f32x16()
        sin_f32,    |x : f32| x.sin(),    init_f32()
        sin_f64x4,  |x : f64x4| x.sin(),  init_f64x4()
        sin_f64x8,  |x : f64x8| x.sin(),  init_f64x8()
        sin_f64,    |x : f64| x.sin(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        cos_f32x4,  |x : f32x4| x.cos(),  init_f32x4()
        cos_f32x16, |x : f32x16| x.cos(), init_f32x16()
        cos_f32,    |x : f32| x.cos(),    init_f32()
        cos_f64x4,  |x : f64x4| x.cos(),  init_f64x4()
        cos_f64x8,  |x : f64x8| x.cos(),  init_f64x8()
        cos_f64,    |x : f64| x.cos(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        tan_f32x4,  |x : f32x4| x.tan(),  init_f32x4()
        tan_f32x16, |x : f32x16| x.tan(), init_f32x16()
        tan_f32,    |x : f32| x.tan(),    init_f32()
        tan_f64x4,  |x : f64x4| x.tan(),  init_f64x4()
        tan_f64x8,  |x : f64x8| x.tan(),  init_f64x8()
        tan_f64,    |x : f64| x.tan(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        asin_f32x4,  |x : f32x4| x.asin(),  init_f32x4()
        asin_f32x16, |x : f32x16| x.asin(), init_f32x16()
        asin_f32,    |x : f32| x.asin(),    init_f32()
        asin_f64x4,  |x : f64x4| x.asin(),  init_f64x4()
        asin_f64x8,  |x : f64x8| x.asin(),  init_f64x8()
        asin_f64,    |x : f64| x.asin(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        acos_f32x4,  |x : f32x4| x.acos(),  init_f32x4()
        acos_f32x16, |x : f32x16| x.acos(), init_f32x16()
        acos_f32,    |x : f32| x.acos(),    init_f32()
        acos_f64x4,  |x : f64x4| x.acos(),  init_f64x4()
        acos_f64x8,  |x : f64x8| x.acos(),  init_f64x8()
        acos_f64,    |x : f64| x.acos(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        atan_f32x4,  |x : f32x4| x.atan(),  init_f32x4()
        atan_f32x16, |x : f32x16| x.atan(), init_f32x16()
        atan_f32,    |x : f32| x.atan(),    init_f32()
        atan_f64x4,  |x : f64x4| x.atan(),  init_f64x4()
        atan_f64x8,  |x : f64x8| x.atan(),  init_f64x8()
        atan_f64,    |x : f64| x.atan(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        exp2_f32x4,  |x : f32x4| x.exp2(),  init_f32x4()
        exp2_f32x16, |x : f32x16| x.exp2(), init_f32x16()
        exp2_f32,    |x : f32| x.exp2(),    init_f32()
        exp2_f64x4,  |x : f64x4| x.exp2(),  init_f64x4()
        exp2_f64x8,  |x : f64x8| x.exp2(),  init_f64x8()
        exp2_f64,    |x : f64| x.exp2(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        exp_f32x4,  |x : f32x4| x.exp(),  init_f32x4()
        exp_f32x16, |x : f32x16| x.exp(), init_f32x16()
        exp_f32,    |x : f32| x.exp(),    init_f32()
        exp_f64x4,  |x : f64x4| x.exp(),  init_f64x4()
        exp_f64x8,  |x : f64x8| x.exp(),  init_f64x8()
        exp_f64,    |x : f64| x.exp(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        log2_f32x4,  |x : f32x4| x.log2(),  init_f32x4()
        log2_f32x16, |x : f32x16| x.log2(), init_f32x16()
        log2_f32,    |x : f32| x.log2(),    init_f32()
        log2_f64x4,  |x : f64x4| x.log2(),  init_f64x4()
        log2_f64x8,  |x : f64x8| x.log2(),  init_f64x8()
        log2_f64,    |x : f64| x.log2(),    init_f64()
    )
}

benchmark_libm! {
    functions (
        ln_f32x4,  |x : f32x4| x.ln(),  init_f32x4()
        ln_f32x16, |x : f32x16| x.ln(), init_f32x16()
        ln_f32,    |x : f32| x.ln(),    init_f32()
        ln_f64x4,  |x : f64x4| x.ln(),  init_f64x4()
        ln_f64x8,  |x : f64x8| x.ln(),  init_f64x8()
        ln_f64,    |x : f64| x.ln(),    init_f64()
    )
}
