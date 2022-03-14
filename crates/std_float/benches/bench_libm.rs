#![feature(test)]
#![feature(portable_simd)]

extern crate test;
use std_float::StdLibm;

use test::{Bencher, black_box};

const N : usize = 1024;

// These fuctions are not inlined to make it easier to check the asm.
//
// Build with:
//
// RUSTFLAGS="-C target-cpu=native --emit asm" cargo bench

#[inline(never)]
pub fn bench_sin_f32x16(x: &[core_simd::f32x16], y: &mut [core_simd::f32x16]) {
    for (x, y) in x.iter().zip(y.iter_mut()) {
        *y = x.sin();
    }
}

#[inline(never)]
pub fn bench_sin_f32x4(x: &[core_simd::f32x4], y: &mut [core_simd::f32x4]) {
    for (x, y) in x.iter().zip(y.iter_mut()) {
        *y = x.sin();
    }
}

#[inline(never)]
pub fn bench_sin_f32(x: &[f32], y: &mut [f32]) {
    for (x, y) in x.iter().zip(y.iter_mut()) {
        *y = x.sin();
    }
}

#[bench]
fn sin_f32x4(b: &mut Bencher) {
    type Type = core_simd::f32x4;
    let x = vec![Type::splat(black_box(1.0)); N/4];
    let mut y = vec![Type::splat(0.0); N/4];
    b.iter(|| {
        bench_sin_f32x4(&x, &mut y);
    })
}

#[bench]
fn sin_f32x16(b: &mut Bencher) {
    type Type = core_simd::f32x16;
    let x = vec![Type::splat(black_box(1.0)); N/16];
    let mut y = vec![Type::splat(0.0); N/16];
    b.iter(|| {
        bench_sin_f32x16(&x, &mut y);
    })
}

#[bench]
fn sin_f32(b: &mut Bencher) {
    let x = black_box(vec![f32::from(1.0); N]);
    let mut y = vec![f32::from(0.0); N];
    b.iter(|| {
        bench_sin_f32(&x, &mut y);
    })
}
