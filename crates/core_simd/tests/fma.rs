#![feature(portable_simd)]

use core_simd::simd::prelude::*;

#[test]
fn test_mul_add_basic() {
    let a = f32x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let b = f32x4::from_array([10.0, 10.0, 10.0, 10.0]);
    let c = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(a.mul_add(b, c), f32x4::from_array([21.0, 32.0, 43.0, 54.0]));
}

#[test]
fn test_mul_add_f64() {
    let a = f64x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let b = f64x4::from_array([10.0, 10.0, 10.0, 10.0]);
    let c = f64x4::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(a.mul_add(b, c), f64x4::from_array([21.0, 32.0, 43.0, 54.0]));
}

#[test]
fn test_mul_sub_basic() {
    let a = f32x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let b = f32x4::from_array([10.0, 10.0, 10.0, 10.0]);
    let c = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(a.mul_sub(b, c), f32x4::from_array([19.0, 28.0, 37.0, 46.0]));
}

#[test]
fn test_mul_sub_f64() {
    let a = f64x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let b = f64x4::from_array([10.0, 10.0, 10.0, 10.0]);
    let c = f64x4::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(a.mul_sub(b, c), f64x4::from_array([19.0, 28.0, 37.0, 46.0]));
}

#[test]
fn test_fma_accuracy_catastrophic_cancellation() {
    let epsilon = 1e-4_f32;
    let x = 1.0 + epsilon;
    let y = 1.0 - epsilon;

    let a = f32x4::splat(x);
    let b = f32x4::splat(y);
    let c = f32x4::splat(-1.0);

    let fma_result = a.mul_add(b, c);
    let separate_result = a * b + c;

    let expected = -epsilon * epsilon;

    let fma_error = (fma_result[0] - expected).abs();
    let sep_error = (separate_result[0] - expected).abs();

    assert!(fma_error <= sep_error);
}

#[test]
fn test_fma_accuracy_discriminant() {
    let b = f64x2::splat(1e8);
    let four_ac = f64x2::splat(1.0);

    let fma_discriminant = b.mul_add(b, -four_ac);
    let sep_discriminant = b * b - four_ac;

    let expected = 1e16 - 1.0;

    let fma_error = ((fma_discriminant[0] - expected) / expected).abs();
    let sep_error = ((sep_discriminant[0] - expected) / expected).abs();

    assert!(fma_error <= sep_error);
}

#[test]
fn test_fma_accuracy_polynomial() {
    let x = f64x2::splat(1.00001);
    let a = f64x2::splat(1.0);
    let b = f64x2::splat(-2.0);
    let c = f64x2::splat(1.0);

    let fma_result = a.mul_add(x, b).mul_add(x, c);
    let sep_result = (a * x + b) * x + c;

    let expected = (x[0] - 1.0) * (x[0] - 1.0);

    let fma_error = (fma_result[0] - expected).abs();
    let sep_error = (sep_result[0] - expected).abs();

    assert!(fma_error < sep_error || (fma_error - sep_error).abs() < 1e-15);
}

#[test]
fn test_negative_values() {
    let a = f32x4::from_array([-2.0, -3.0, -4.0, -5.0]);
    let b = f32x4::splat(2.0);
    let c = f32x4::splat(1.0);
    assert_eq!(a.mul_add(b, c), f32x4::from_array([-3.0, -5.0, -7.0, -9.0]));
    assert_eq!(
        a.mul_sub(b, c),
        f32x4::from_array([-5.0, -7.0, -9.0, -11.0])
    );
}

#[test]
fn test_infinity() {
    let a = f32x4::from_array([f32::INFINITY, 1.0, 2.0, 3.0]);
    let b = f32x4::splat(2.0);
    let c = f32x4::splat(1.0);
    let result = a.mul_add(b, c);
    assert_eq!(result[0], f32::INFINITY);
    assert_eq!(result[1], 3.0);
}

#[test]
fn test_nan_propagation() {
    let a = f32x4::from_array([f32::NAN, 2.0, 3.0, 4.0]);
    let b = f32x4::splat(2.0);
    let c = f32x4::splat(1.0);
    let result = a.mul_add(b, c);
    assert!(result[0].is_nan());
    assert_eq!(result[1], 5.0);
}

#[test]
fn test_different_sizes() {
    let a2 = f32x2::from_array([3.0, 4.0]);
    let b2 = f32x2::from_array([2.0, 2.0]);
    let c2 = f32x2::from_array([1.0, 1.0]);
    assert_eq!(a2.mul_add(b2, c2), f32x2::from_array([7.0, 9.0]));

    let a8 = f32x8::splat(2.0);
    let b8 = f32x8::splat(3.0);
    let c8 = f32x8::splat(4.0);
    assert_eq!(a8.mul_add(b8, c8), f32x8::splat(10.0));
}

#[test]
fn test_polynomial_evaluation() {
    let x = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let result = f32x4::splat(2.0)
        .mul_add(x, f32x4::splat(3.0))
        .mul_add(x, f32x4::splat(5.0));
    assert_eq!(result, f32x4::from_array([10.0, 19.0, 32.0, 49.0]));
}

#[test]
fn test_max_min_values() {
    let a = f32x4::from_array([f32::MAX, f32::MIN, 1.0, -1.0]);
    let b = f32x4::splat(1.0);
    let c = f32x4::splat(0.0);
    let result = a.mul_add(b, c);
    assert_eq!(result[0], f32::MAX);
    assert_eq!(result[1], f32::MIN);
}

#[test]
fn test_subnormal_values() {
    let subnormal = f32::MIN_POSITIVE / 2.0;
    let a = f32x4::splat(subnormal);
    let b = f32x4::splat(2.0);
    let c = f32x4::splat(0.0);
    let result = a.mul_add(b, c);
    assert!(result[0].is_finite());

    // On platforms with flush-to-zero (FTZ) mode (e.g., ARM NEON), subnormal
    // values in SIMD operations may be flushed to zero for performance.
    // We accept either the mathematically correct result or zero.
    let expected = subnormal * 2.0;
    assert!(
        result[0] == expected || result[0] == 0.0,
        "Expected {} (or 0.0 due to FTZ), got {}",
        expected,
        result[0]
    );
}
