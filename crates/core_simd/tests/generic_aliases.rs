#![feature(portable_simd)]

//! Tests for generic type shorthand aliases
//!
//! Tests cover:
//! - Basic usage (u32xN, f32xN, etc.)
//! - Mask generic aliases (mask32xN, etc.)
//! - Edge cases (N=64, non-power-of-2)
//! - Generic functions and structs

use core_simd::simd::prelude::*;
use core_simd::simd::*;

// ============================================================================
// Basic Generic Functions
// ============================================================================

fn add_generic<const N: usize>(a: u32xN<N>, b: u32xN<N>) -> u32xN<N> {
    a + b
}

fn multiply_add_f32<const N: usize>(x: f32xN<N>, y: f32xN<N>, z: f32xN<N>) -> f32xN<N> {
    x * y + z
}

fn dot_product<const N: usize>(a: f32xN<N>, b: f32xN<N>) -> f32 {
    (a * b).reduce_sum()
}

// Generic struct with SIMD fields
struct Point<const N: usize> {
    x: f32xN<N>,
    y: f32xN<N>,
    z: f32xN<N>,
}

impl<const N: usize> Point<N> {
    fn new(x: f32xN<N>, y: f32xN<N>, z: f32xN<N>) -> Self {
        Self { x, y, z }
    }

    fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x + self.y * other.y + self.z * other.z).reduce_sum()
    }

    fn scale(&self, factor: f32) -> Self {
        let s = f32xN::<N>::splat(factor);
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

// ============================================================================
// Basic Functionality Tests
// ============================================================================

#[test]
fn test_basic_usage() {
    let a = Simd::<u32, 4>::from_array([1, 2, 3, 4]);
    let b = Simd::<u32, 4>::from_array([5, 6, 7, 8]);
    let result: u32xN<4> = add_generic(a, b);
    assert_eq!(result.as_array(), &[6, 8, 10, 12]);

    let x = Simd::<f32, 4>::from_array([1.0, 2.0, 3.0, 4.0]);
    let y = Simd::<f32, 4>::from_array([2.0, 3.0, 4.0, 5.0]);
    let z = Simd::<f32, 4>::from_array([0.5, 0.5, 0.5, 0.5]);
    let result = multiply_add_f32(x, y, z);
    assert_eq!(result.as_array(), &[2.5, 6.5, 12.5, 20.5]);
}

#[test]
fn test_all_integer_types() {
    // Signed
    let i8_vec: i8xN<4> = Simd::from_array([-1, -2, -3, -4]);
    let result: i8xN<4> = i8_vec + Simd::splat(1i8);
    assert_eq!(result.as_array(), &[0, -1, -2, -3]);

    let i16_vec: i16xN<4> = Simd::from_array([-10, -20, -30, -40]);
    let result: i16xN<4> = i16_vec + Simd::splat(5i16);
    assert_eq!(result.as_array(), &[-5, -15, -25, -35]);

    let i32_vec: i32xN<4> = Simd::from_array([-100, -200, -300, -400]);
    let result: i32xN<4> = i32_vec + Simd::splat(50i32);
    assert_eq!(result.as_array(), &[-50, -150, -250, -350]);

    let i64_vec: i64xN<4> = Simd::from_array([-1000, -2000, -3000, -4000]);
    let result: i64xN<4> = i64_vec + Simd::splat(500i64);
    assert_eq!(result.as_array(), &[-500, -1500, -2500, -3500]);

    let isize_vec: isizexN<4> = Simd::from_array([-10, -20, -30, -40]);
    let result: isizexN<4> = isize_vec + Simd::splat(5isize);
    assert_eq!(result.as_array(), &[-5, -15, -25, -35]);

    // Unsigned
    let u8_vec: u8xN<4> = Simd::from_array([1, 2, 3, 4]);
    let result: u8xN<4> = u8_vec + Simd::splat(1u8);
    assert_eq!(result.as_array(), &[2, 3, 4, 5]);

    let u16_vec: u16xN<4> = Simd::from_array([10, 20, 30, 40]);
    let result: u16xN<4> = u16_vec + Simd::splat(5u16);
    assert_eq!(result.as_array(), &[15, 25, 35, 45]);

    let u32_vec: u32xN<4> = Simd::from_array([100, 200, 300, 400]);
    let result: u32xN<4> = u32_vec + Simd::splat(50u32);
    assert_eq!(result.as_array(), &[150, 250, 350, 450]);

    let u64_vec: u64xN<4> = Simd::from_array([1000, 2000, 3000, 4000]);
    let result: u64xN<4> = u64_vec + Simd::splat(500u64);
    assert_eq!(result.as_array(), &[1500, 2500, 3500, 4500]);

    let usize_vec: usizexN<4> = Simd::from_array([10, 20, 30, 40]);
    let result: usizexN<4> = usize_vec + Simd::splat(5usize);
    assert_eq!(result.as_array(), &[15, 25, 35, 45]);

    // Float
    let f32_vec: f32xN<4> = Simd::from_array([1.0, 2.0, 3.0, 4.0]);
    let result: f32xN<4> = f32_vec * Simd::splat(2.0);
    assert_eq!(result.as_array(), &[2.0, 4.0, 6.0, 8.0]);

    let f64_vec: f64xN<4> = Simd::from_array([1.5, 2.5, 3.5, 4.5]);
    let result: f64xN<4> = f64_vec + Simd::splat(0.5);
    assert_eq!(result.as_array(), &[2.0, 3.0, 4.0, 5.0]);
}

#[test]
fn test_struct_with_generic_simd() {
    let p1: Point<4> = Point::new(
        Simd::from_array([1.0, 2.0, 3.0, 4.0]),
        Simd::from_array([5.0, 6.0, 7.0, 8.0]),
        Simd::from_array([9.0, 10.0, 11.0, 12.0]),
    );

    let p2: Point<4> = Point::new(Simd::splat(1.0), Simd::splat(1.0), Simd::splat(1.0));

    assert_eq!(p1.dot(&p2), 78.0);

    let scaled = p1.scale(2.0);
    assert_eq!(scaled.x.as_array(), &[2.0, 4.0, 6.0, 8.0]);
    assert_eq!(scaled.y.as_array(), &[10.0, 12.0, 14.0, 16.0]);
    assert_eq!(scaled.z.as_array(), &[18.0, 20.0, 22.0, 24.0]);
}

// ============================================================================
// Edge Case: Maximum Lane Count (N=64)
// ============================================================================

#[test]
fn test_max_lanes_64() {
    let u32_max: u32xN<64> = Simd::splat(123456);
    assert_eq!(u32_max.as_array()[0], 123456);
    assert_eq!(u32_max.as_array()[63], 123456);

    let i32_max: i32xN<64> = Simd::splat(-123456);
    assert_eq!(i32_max.as_array()[0], -123456);
    assert_eq!(i32_max.as_array()[63], -123456);

    let f32_max: f32xN<64> = Simd::splat(1.25);
    assert_eq!(f32_max.as_array()[0], 1.25);
    assert_eq!(f32_max.as_array()[63], 1.25);

    // Operations at max size
    let a: u32xN<64> = Simd::splat(10);
    let b: u32xN<64> = Simd::splat(20);
    let sum = a + b;
    assert_eq!(sum, Simd::splat(30));
}

// ============================================================================
// Edge Case: Non-Power-of-2 Lane Counts
// ============================================================================

#[test]
fn test_non_power_of_2_lanes() {
    // N=3
    let v3: u32xN<3> = Simd::from_array([1, 2, 3]);
    let r3 = v3 + Simd::splat(10);
    assert_eq!(r3.as_array(), &[11, 12, 13]);

    // N=5
    let v5: i32xN<5> = Simd::from_array([1, 2, 3, 4, 5]);
    let r5 = v5 * Simd::splat(2);
    assert_eq!(r5.as_array(), &[2, 4, 6, 8, 10]);

    // N=7
    let v7: f32xN<7> = Simd::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    let sum = v7.reduce_sum();
    assert_eq!(sum, 28.0);

    // N=6
    let v6: u32xN<6> = Simd::splat(5);
    assert_eq!(v6.as_array(), &[5, 5, 5, 5, 5, 5]);

    // N=9
    let v9: u32xN<9> = Simd::splat(3);
    let sum9 = v9.reduce_sum();
    assert_eq!(sum9, 27);
}

// ============================================================================
// Mask Generic Aliases
// ============================================================================

#[test]
fn test_mask_aliases_basic() {
    let values: i32xN<4> = Simd::from_array([1, -2, 3, -4]);
    let zero = i32xN::<4>::splat(0);

    let mask: mask32xN<4> = values.simd_lt(zero);
    let result = mask.select(zero, values);
    assert_eq!(result.as_array(), &[1, 0, 3, 0]);
}

#[test]
fn test_mask_all_sizes() {
    // mask8xN
    let v8: i8xN<4> = Simd::from_array([1, -1, 2, -2]);
    let m8: mask8xN<4> = v8.simd_lt(Simd::splat(0));
    assert!(!m8.test(0));
    assert!(m8.test(1));

    // mask16xN
    let v16: i16xN<4> = Simd::from_array([100, -100, 200, -200]);
    let m16: mask16xN<4> = v16.simd_lt(Simd::splat(0));
    assert_eq!(m16.to_array(), [false, true, false, true]);

    // mask32xN
    let v32: i32xN<4> = Simd::from_array([1000, -1000, 2000, -2000]);
    let m32: mask32xN<4> = v32.simd_gt(Simd::splat(0));
    assert_eq!(m32.to_array(), [true, false, true, false]);

    // mask64xN
    let v64: i64xN<4> = Simd::from_array([10000, -10000, 20000, -20000]);
    let m64: mask64xN<4> = v64.simd_eq(Simd::splat(10000));
    assert_eq!(m64.to_array(), [true, false, false, false]);

    // masksizexN
    let vsize: isizexN<4> = Simd::from_array([1, 2, 3, 4]);
    let msize: masksizexN<4> = vsize.simd_ge(Simd::splat(3));
    assert_eq!(msize.to_array(), [false, false, true, true]);
}

#[test]
fn test_mask_operations() {
    let a: i32xN<4> = Simd::from_array([1, -2, 3, -4]);
    let zero = i32xN::<4>::splat(0);

    let neg_mask: mask32xN<4> = a.simd_lt(zero);
    let pos_mask: mask32xN<4> = a.simd_gt(zero);

    assert!(neg_mask.any());
    assert!(pos_mask.any());
    assert!(!neg_mask.all());
    assert!(!pos_mask.all());

    let clamped = neg_mask.select(zero, a);
    assert_eq!(clamped.as_array(), &[1, 0, 3, 0]);
}

// ============================================================================
// Various Lane Counts
// ============================================================================

#[test]
fn test_various_lane_counts() {
    // N=1
    let a1: u32xN<1> = Simd::from_array([42]);
    let b1: u32xN<1> = Simd::from_array([8]);
    assert_eq!(add_generic(a1, b1).as_array(), &[50]);

    // N=2
    let a2: u32xN<2> = Simd::from_array([1, 2]);
    let b2: u32xN<2> = Simd::from_array([3, 4]);
    assert_eq!(add_generic(a2, b2).as_array(), &[4, 6]);

    // N=8
    let a8: u32xN<8> = Simd::splat(100);
    let b8: u32xN<8> = Simd::splat(200);
    assert_eq!(add_generic(a8, b8), Simd::splat(300));

    // N=16
    let a16: u32xN<16> = Simd::splat(10);
    let b16: u32xN<16> = Simd::splat(5);
    assert_eq!(add_generic(a16, b16), Simd::splat(15));

    // N=32
    let a32: u32xN<32> = Simd::splat(7);
    let b32: u32xN<32> = Simd::splat(3);
    assert_eq!(add_generic(a32, b32), Simd::splat(10));
}

// ============================================================================
// Type Inference and Conversions
// ============================================================================

#[test]
fn test_type_inference() {
    let a = Simd::<f32, 4>::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = Simd::<f32, 4>::from_array([5.0, 6.0, 7.0, 8.0]);
    let result = dot_product(a, b);
    assert_eq!(result, 70.0);
}

#[test]
fn test_conversion_between_types() {
    let u32_vec: u32xN<4> = Simd::from_array([1, 2, 3, 4]);
    let array: [u32; 4] = u32_vec.to_array();
    assert_eq!(array, [1, 2, 3, 4]);

    let back: u32xN<4> = Simd::from_array(array);
    assert_eq!(back, u32_vec);
}

#[test]
fn test_turbofish_syntax() {
    let a = u32xN::<4>::splat(42);
    assert_eq!(a.as_array(), &[42, 42, 42, 42]);

    let b = f32xN::<8>::splat(2.5);
    assert_eq!(b.as_array()[0], 2.5);
    assert_eq!(b.as_array()[7], 2.5);

    let m = mask32xN::<4>::splat(true);
    assert!(m.all());
}
