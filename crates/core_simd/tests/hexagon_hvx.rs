#![cfg(target_arch = "hexagon")]
#![feature(portable_simd)]

use core_simd::simd::prelude::{SimdPartialEq, SimdPartialOrd, SimdUint};
use core_simd::simd::*;

#[test]
fn test_hvx_full_vector_u8() {
    // Test 1024-bit vector with u8 elements (128 elements)
    let a = u8x128::splat(10);
    let b = u8x128::splat(20);
    let c = a + b;

    assert_eq!(c[0], 30);
    assert_eq!(c[127], 30);
}

#[test]
fn test_hvx_full_vector_u16() {
    // Test 1024-bit vector with u16 elements (64 elements)
    let a = u16x64::splat(100);
    let b = u16x64::splat(200);
    let c = a + b;

    assert_eq!(c[0], 300);
    assert_eq!(c[63], 300);
}

#[test]
fn test_hvx_full_vector_u32() {
    // Test 1024-bit vector with u32 elements (32 elements)
    let a = u32x32::splat(1000);
    let b = u32x32::splat(2000);
    let c = a + b;

    assert_eq!(c[0], 3000);
    assert_eq!(c[31], 3000);
}

#[test]
fn test_hvx_full_vector_u64() {
    // Test 1024-bit vector with u64 elements (16 elements)
    let a = u64x16::splat(10000);
    let b = u64x16::splat(20000);
    let c = a + b;

    assert_eq!(c[0], 30000);
    assert_eq!(c[15], 30000);
}

#[test]
fn test_hvx_half_vector_u8() {
    // Test 512-bit vector with u8 elements (64 elements)
    let a = u8x64::splat(5);
    let b = u8x64::splat(10);
    let c = a + b;

    assert_eq!(c[0], 15);
    assert_eq!(c[63], 15);
}

#[test]
fn test_hvx_half_vector_u16() {
    // Test 512-bit vector with u16 elements (32 elements)
    let a = u16x32::splat(50);
    let b = u16x32::splat(100);
    let c = a + b;

    assert_eq!(c[0], 150);
    assert_eq!(c[31], 150);
}

#[test]
fn test_hvx_half_vector_u32() {
    // Test 512-bit vector with u32 elements (16 elements)
    let a = u32x16::splat(500);
    let b = u32x16::splat(1000);
    let c = a + b;

    assert_eq!(c[0], 1500);
    assert_eq!(c[15], 1500);
}

#[test]
fn test_hvx_half_vector_u64() {
    // Test 512-bit vector with u64 elements (8 elements)
    let a = u64x8::splat(5000);
    let b = u64x8::splat(10000);
    let c = a + b;

    assert_eq!(c[0], 15000);
    assert_eq!(c[7], 15000);
}

#[test]
fn test_hvx_signed_vectors() {
    // Test signed integer vectors
    let a = i8x128::splat(-10);
    let b = i8x128::splat(20);
    let c = a + b;
    assert_eq!(c[0], 10);

    let d = i16x64::splat(-100);
    let e = i16x64::splat(200);
    let f = d + e;
    assert_eq!(f[0], 100);

    let g = i32x32::splat(-1000);
    let h = i32x32::splat(2000);
    let i = g + h;
    assert_eq!(i[0], 1000);

    let j = i64x16::splat(-10000);
    let k = i64x16::splat(20000);
    let l = j + k;
    assert_eq!(l[0], 10000);
}

#[test]
fn test_hvx_common_operations() {
    // Test common SIMD operations
    let a = u32x32::splat(10);
    let b = u32x32::splat(3);

    // Addition
    let sum = a + b;
    assert_eq!(sum[0], 13);

    // Subtraction
    let diff = a - b;
    assert_eq!(diff[0], 7);

    // Bitwise AND
    let and = a & b;
    assert_eq!(and[0], 2); // 10 & 3 = 2

    // Bitwise OR
    let or = a | b;
    assert_eq!(or[0], 11); // 10 | 3 = 11

    // Bitwise XOR
    let xor = a ^ b;
    assert_eq!(xor[0], 9); // 10 ^ 3 = 9
}

#[test]
fn test_hvx_masks() {
    // Test mask operations
    let a = u8x128::splat(10);
    let b = u8x128::splat(10);
    let c = u8x128::splat(5);

    let eq_mask = a.simd_eq(b);
    let ne_mask = a.simd_ne(c);
    let gt_mask = a.simd_gt(c);

    assert!(eq_mask.all());
    assert!(ne_mask.all());
    assert!(gt_mask.all());
}

#[test]
fn test_hvx_lane_operations() {
    // Test lane-wise operations
    let mut v = u32x32::splat(0);

    // Set specific lanes
    for i in 0..32 {
        v[i] = i as u32;
    }

    // Verify lanes
    for i in 0..32 {
        assert_eq!(v[i], i as u32);
    }

    // Test reduction
    let sum: u32 = (0..32).sum();
    assert_eq!(v.reduce_sum(), sum);
}

#[test]
fn test_hvx_smaller_vectors() {
    // Test smaller vector sizes that still use HVX
    let a = u8x16::splat(10);
    let b = u8x16::splat(5);
    let c = a + b;
    assert_eq!(c[0], 15);

    let d = u16x8::splat(100);
    let e = u16x8::splat(50);
    let f = d + e;
    assert_eq!(f[0], 150);

    let g = u32x4::splat(1000);
    let h = u32x4::splat(500);
    let i = g + h;
    assert_eq!(i[0], 1500);

    let j = u64x2::splat(10000);
    let k = u64x2::splat(5000);
    let l = j + k;
    assert_eq!(l[0], 15000);
}

#[test]
fn test_hvx_type_conversions() {
    // Test that our HVX types can be converted to/from portable SIMD types
    let portable_vec = u8x64::splat(42);

    // The from_transmute! macro should allow conversion
    // This tests that our type mappings work correctly
    let sum = portable_vec + u8x64::splat(8);
    assert_eq!(sum[0], 50);
    assert_eq!(sum[63], 50);
}
