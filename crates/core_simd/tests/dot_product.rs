#![feature(portable_simd)]

use core_simd::simd::prelude::*;

#[test]
fn test_dot_basic_f32x4() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([5.0, 6.0, 7.0, 8.0]);
    assert_eq!(a.dot(b), 70.0);
}

#[test]
fn test_dot_basic_f64x4() {
    let a = f64x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f64x4::from_array([5.0, 6.0, 7.0, 8.0]);
    assert_eq!(a.dot(b), 70.0);
}

#[test]
fn test_dot3_ignores_w() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 999.0]);
    let b = f32x4::from_array([4.0, 5.0, 6.0, 888.0]);
    assert_eq!(a.dot3(b), 32.0);
    assert_ne!(a.dot(b), a.dot3(b));
}

#[test]
fn test_dot3_f64() {
    let a = f64x4::from_array([1.0, 2.0, 3.0, 999.0]);
    let b = f64x4::from_array([4.0, 5.0, 6.0, 888.0]);
    assert_eq!(a.dot3(b), 32.0);
}

#[test]
fn test_dot4_all_elements() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([5.0, 6.0, 7.0, 8.0]);
    assert_eq!(a.dot4(b), 70.0);
    assert_eq!(a.dot4(b), a.dot(b));
}

#[test]
fn test_dot4_ignores_beyond_4() {
    let a = f32x8::from_array([1.0, 2.0, 3.0, 4.0, 100.0, 200.0, 300.0, 400.0]);
    let b = f32x8::from_array([5.0, 6.0, 7.0, 8.0, 999.0, 888.0, 777.0, 666.0]);
    assert_eq!(a.dot4(b), 70.0);
    assert_ne!(a.dot(b), a.dot4(b));
}

#[test]
fn test_orthogonal_vectors() {
    let x_axis = f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    let y_axis = f32x4::from_array([0.0, 1.0, 0.0, 0.0]);
    assert_eq!(x_axis.dot3(y_axis), 0.0);
    assert_eq!(x_axis.dot4(y_axis), 0.0);
    assert_eq!(x_axis.dot(y_axis), 0.0);
}

#[test]
fn test_parallel_unit_vectors() {
    let v = f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    assert_eq!(v.dot3(v), 1.0);
    assert_eq!(v.dot4(v), 1.0);
    assert_eq!(v.dot(v), 1.0);
}

#[test]
fn test_negative_values() {
    let a = f32x4::from_array([1.0, -2.0, 3.0, -4.0]);
    let b = f32x4::from_array([-5.0, 6.0, -7.0, 8.0]);
    assert_eq!(a.dot4(b), -70.0);
    assert_eq!(a.dot(b), -70.0);
    assert_eq!(a.dot3(b), -38.0);
}

#[test]
fn test_zero_vectors() {
    let zero = f32x4::splat(0.0);
    let v = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(zero.dot(v), 0.0);
    assert_eq!(zero.dot3(v), 0.0);
    assert_eq!(zero.dot4(v), 0.0);
    assert_eq!(v.dot(zero), 0.0);
}

#[test]
fn test_commutativity() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([5.0, 6.0, 7.0, 8.0]);
    assert_eq!(a.dot(b), b.dot(a));
    assert_eq!(a.dot3(b), b.dot3(a));
    assert_eq!(a.dot4(b), b.dot4(a));
}

#[test]
fn test_distributivity() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([5.0, 6.0, 7.0, 8.0]);
    let c = f32x4::from_array([9.0, 10.0, 11.0, 12.0]);

    let left = a.dot(b + c);
    let right = a.dot(b) + a.dot(c);
    assert!((left - right).abs() < 1e-6);

    let left3 = a.dot3(b + c);
    let right3 = a.dot3(b) + a.dot3(c);
    assert!((left3 - right3).abs() < 1e-6);
}

#[test]
fn test_different_sizes_f32() {
    let a2 = f32x2::from_array([3.0, 4.0]);
    let b2 = f32x2::from_array([5.0, 12.0]);
    assert_eq!(a2.dot(b2), 63.0);

    let a8 = f32x8::splat(2.0);
    let b8 = f32x8::splat(3.0);
    assert_eq!(a8.dot(b8), 48.0);
}

#[test]
fn test_different_sizes_f64() {
    let a2 = f64x2::from_array([3.0, 4.0]);
    let b2 = f64x2::from_array([5.0, 12.0]);
    assert_eq!(a2.dot(b2), 63.0);

    let a8 = f64x8::splat(2.0);
    let b8 = f64x8::splat(3.0);
    assert_eq!(a8.dot(b8), 48.0);
}

#[test]
fn test_magnitude_squared() {
    let v = f32x4::from_array([3.0, 4.0, 0.0, 0.0]);
    assert_eq!(v.dot3(v), 25.0);
    assert_eq!(v.dot4(v), 25.0);
    assert_eq!(v.dot(v), 25.0);
}

#[test]
fn test_dot3_with_larger_vectors() {
    let a = f32x8::from_array([1.0, 2.0, 3.0, 100.0, 200.0, 300.0, 400.0, 500.0]);
    let b = f32x8::from_array([4.0, 5.0, 6.0, 999.0, 888.0, 777.0, 666.0, 555.0]);
    assert_eq!(a.dot3(b), 32.0);
}

#[test]
fn test_dot4_with_larger_vectors() {
    let a = f32x8::from_array([1.0, 2.0, 3.0, 4.0, 100.0, 200.0, 300.0, 400.0]);
    let b = f32x8::from_array([5.0, 6.0, 7.0, 8.0, 999.0, 888.0, 777.0, 666.0]);
    assert_eq!(a.dot4(b), 70.0);
}

#[test]
fn test_equivalence_to_manual_calculation() {
    let a = f32x4::from_array([1.5, 2.5, 3.5, 4.5]);
    let b = f32x4::from_array([5.5, 6.5, 7.5, 8.5]);

    let dot_result = a.dot(b);
    let manual_result = (a * b).reduce_sum();
    assert_eq!(dot_result, manual_result);
}

#[test]
fn test_scale_property() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([5.0, 6.0, 7.0, 8.0]);
    let k = 3.0;

    let left = (a * f32x4::splat(k)).dot(b);
    let right = k * a.dot(b);
    assert!((left - right).abs() < 1e-6);
}

#[test]
fn test_with_special_values() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([f32::INFINITY, 0.0, 0.0, 0.0]);
    assert_eq!(a.dot(b), f32::INFINITY);

    let c = f32x4::from_array([1.0, f32::NEG_INFINITY, 0.0, 0.0]);
    let d = f32x4::from_array([0.0, 1.0, 0.0, 0.0]);
    assert_eq!(c.dot(d), f32::NEG_INFINITY);
}

#[test]
fn test_nan_propagation() {
    let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4::from_array([f32::NAN, 6.0, 7.0, 8.0]);
    assert!(a.dot(b).is_nan());
    assert!(a.dot3(b).is_nan());
    assert!(a.dot4(b).is_nan());
}

#[test]
fn test_mixed_signs() {
    let a = f32x4::from_array([1.0, -1.0, 1.0, -1.0]);
    let b = f32x4::from_array([2.0, 2.0, 2.0, 2.0]);
    assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_precision_f64() {
    let a = f64x4::from_array([0.1, 0.2, 0.3, 0.4]);
    let b = f64x4::from_array([0.5, 0.6, 0.7, 0.8]);
    let result = a.dot(b);
    assert!((result - 0.70).abs() < 1e-10);
}

#[test]
fn test_max_values() {
    let a = f32x4::from_array([f32::MAX, 0.0, 0.0, 0.0]);
    let b = f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    assert_eq!(a.dot(b), f32::MAX);
    assert_eq!(a.dot3(b), f32::MAX);
    assert_eq!(a.dot4(b), f32::MAX);

    let c = f64x4::from_array([f64::MAX, 0.0, 0.0, 0.0]);
    let d = f64x4::from_array([1.0, 0.0, 0.0, 0.0]);
    assert_eq!(c.dot(d), f64::MAX);
}

#[test]
fn test_min_values() {
    let a = f32x4::from_array([f32::MIN, 0.0, 0.0, 0.0]);
    let b = f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    assert_eq!(a.dot(b), f32::MIN);
    assert_eq!(a.dot3(b), f32::MIN);
    assert_eq!(a.dot4(b), f32::MIN);

    let c = f64x4::from_array([f64::MIN, 0.0, 0.0, 0.0]);
    let d = f64x4::from_array([1.0, 0.0, 0.0, 0.0]);
    assert_eq!(c.dot(d), f64::MIN);
}

#[test]
fn test_subnormal_values() {
    let subnormal_f32 = f32::MIN_POSITIVE / 2.0;
    let a = f32x4::from_array([subnormal_f32, 0.0, 0.0, 0.0]);
    let b = f32x4::from_array([2.0, 0.0, 0.0, 0.0]);
    let result = a.dot(b);
    assert!(result.is_finite());

    // On platforms with flush-to-zero (FTZ) mode (e.g., ARM NEON), subnormal
    // values in SIMD operations may be flushed to zero for performance.
    // We accept either the mathematically correct result or zero.
    let expected = subnormal_f32 * 2.0;
    assert!(
        result == expected || result == 0.0,
        "Expected {} (or 0.0 due to FTZ), got {}",
        expected,
        result
    );

    let subnormal_f64 = f64::MIN_POSITIVE / 2.0;
    let c = f64x4::from_array([subnormal_f64, 0.0, 0.0, 0.0]);
    let d = f64x4::from_array([2.0, 0.0, 0.0, 0.0]);
    let result_f64 = c.dot(d);
    assert!(result_f64.is_finite());

    let expected_f64 = subnormal_f64 * 2.0;
    assert!(
        result_f64 == expected_f64 || result_f64 == 0.0,
        "Expected {} (or 0.0 due to FTZ), got {}",
        expected_f64,
        result_f64
    );
}
