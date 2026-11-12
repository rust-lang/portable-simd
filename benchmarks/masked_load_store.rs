#![feature(portable_simd)]

//! Focused microbenchmark for portable-SIMD masked contiguous loads and stores.
//!
//! Run on nightly, for example:
//!   rustc +nightly -O -C target-cpu=x86-64 -C target-feature=+sse2 \
//!     benchmarks/masked_load_store.rs -o /tmp/masked-load-store
//!   /tmp/masked-load-store
//!
//! Keep the compiler version, target triple, CPU, and flags with any recorded result.
//! This is a diagnostic benchmark, not a substitute for Criterion or end-to-end workloads.

use core::simd::{Mask, Simd};
use std::hint::black_box;
use std::time::{Duration, Instant};

const LANES: usize = 8;
const ITERATIONS: u32 = 5_000_000;

#[inline(never)]
fn simd_load(slice: &[f32]) -> Simd<f32, LANES> {
    Simd::load_or_default(slice)
}

#[inline(never)]
fn scalar_load(slice: &[f32]) -> Simd<f32, LANES> {
    let mut result = [0.0; LANES];
    for (&source, destination) in slice.iter().zip(result.iter_mut()) {
        *destination = source;
    }
    Simd::from_array(result)
}

#[inline(never)]
fn simd_store(value: Simd<f32, LANES>, slice: &mut [f32]) {
    value.store_select(slice, Mask::splat(true));
}

#[inline(never)]
fn scalar_store(value: Simd<f32, LANES>, slice: &mut [f32]) {
    for (destination, &source) in slice.iter_mut().zip(value.as_array().iter()) {
        *destination = source;
    }
}

fn measure(label: &str, mut operation: impl FnMut()) -> Duration {
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        operation();
    }
    let elapsed = start.elapsed();
    println!(
        "{label:24} {:>10.3} ns/iteration ({ITERATIONS} iterations)",
        elapsed.as_nanos() as f64 / f64::from(ITERATIONS)
    );
    elapsed
}

fn main() {
    let source = [0.0_f32, 1.0, 2.0, 3.0, 4.0];
    let value = Simd::from_array([10.0_f32, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0]);

    // Verify semantics before timing: the final three lanes must use the default.
    let expected = Simd::from_array([0.0, 1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0]);
    assert_eq!(simd_load(&source), expected);
    assert_eq!(scalar_load(&source), expected);

    let mut simd_output = [0.0_f32; 5];
    let mut scalar_output = [0.0_f32; 5];
    simd_store(value, &mut simd_output);
    scalar_store(value, &mut scalar_output);
    assert_eq!(simd_output, scalar_output);

    println!("lanes={LANES}, slice_len={}, iterations={ITERATIONS}", source.len());
    measure("SIMD load_or_default", || {
        black_box(simd_load(black_box(&source)));
    });
    measure("scalar load loop", || {
        black_box(scalar_load(black_box(&source)));
    });
    measure("SIMD store_select", || {
        simd_store(black_box(value), black_box(&mut simd_output));
        black_box(&simd_output);
    });
    measure("scalar store loop", || {
        scalar_store(black_box(value), black_box(&mut scalar_output));
        black_box(&scalar_output);
    });
}
