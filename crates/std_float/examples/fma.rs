//! Demonstrates fused multiply-add (FMA) operations.

#![feature(portable_simd)]
use core_simd::simd::prelude::*;
use std_float::StdFloat;

fn main() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let c = f32x4::from_array([10.0, 10.0, 10.0, 10.0]);

    println!("FMA: a*b + c");
    println!("a = {:?}", a.to_array());
    println!("b = {:?}", b.to_array());
    println!("c = {:?}", c.to_array());
    println!("result = {:?}", a.mul_add(b, c).to_array());
    println!();

    // Polynomial: p(x) = 2x³ + 3x² + 4x + 5
    // Horner form: ((2x + 3)x + 4)x + 5
    let x = f32x8::from_array([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    let result = f32x8::splat(2.0)
        .mul_add(x, f32x8::splat(3.0))
        .mul_add(x, f32x8::splat(4.0))
        .mul_add(x, f32x8::splat(5.0));

    println!("Polynomial p(x) = 2x³ + 3x² + 4x + 5");
    println!("x      = {:?}", x.to_array());
    println!("p(x)   = {:?}", result.to_array());
    println!();

    let v1 = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let v2 = f32x4::from_array([5.0, 6.0, 7.0, 8.0]);

    let mut acc = 0.0;
    for i in 0..4 {
        acc = v1[i].mul_add(v2[i], acc);
    }

    println!("Dot product using FMA:");
    println!("v1 · v2 = {}", acc);
    println!();

    let large = f32x4::splat(1e10);
    let small = f32x4::splat(1.0);

    let fma_result = large.mul_add(f32x4::splat(1.0), small);
    let separate_result = large * f32x4::splat(1.0) + small;

    println!("Accuracy comparison (1e10 * 1.0 + 1.0):");
    println!("FMA result:      {:?}", fma_result.to_array());
    println!("Separate ops:    {:?}", separate_result.to_array());
    println!("Both preserve precision in this case");
}
