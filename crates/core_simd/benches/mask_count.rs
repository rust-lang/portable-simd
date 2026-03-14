//! Comprehensive benchmarks for Mask::count() performance analysis
//!
//! This benchmark suite tests:
//! - Different mask sizes (2, 4, 8, 16, 32, 64 elements)
//! - Different densities (0%, 25%, 50%, 75%, 100% true)
//! - Comparison with manual iteration baseline
//! - Cache behavior and instruction-level performance

#![feature(portable_simd)]
#![feature(test)]

extern crate test;
use cmp::SimdPartialOrd;
use core_simd::simd::*;
use test::{Bencher, black_box};

// ============================================================================
// Mask Size: 2 elements (i64)
// ============================================================================

#[bench]
fn mask2_count_0pct(b: &mut Bencher) {
    let mask = mask64x2::splat(false);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask2_count_50pct(b: &mut Bencher) {
    let mask = mask64x2::from_array([true, false]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask2_count_100pct(b: &mut Bencher) {
    let mask = mask64x2::splat(true);
    b.iter(|| black_box(mask).count());
}

// ============================================================================
// Mask Size: 4 elements (i32)
// ============================================================================

#[bench]
fn mask4_count_0pct(b: &mut Bencher) {
    let mask = mask32x4::splat(false);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask4_count_25pct(b: &mut Bencher) {
    let mask = mask32x4::from_array([true, false, false, false]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask4_count_50pct(b: &mut Bencher) {
    let mask = mask32x4::from_array([true, false, true, false]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask4_count_75pct(b: &mut Bencher) {
    let mask = mask32x4::from_array([true, true, true, false]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask4_count_100pct(b: &mut Bencher) {
    let mask = mask32x4::splat(true);
    b.iter(|| black_box(mask).count());
}

// Baseline: manual iteration for mask4
#[bench]
fn mask4_count_manual_50pct(b: &mut Bencher) {
    let mask = mask32x4::from_array([true, false, true, false]);
    b.iter(|| {
        let m = black_box(mask);
        let mut count = 0;
        for i in 0..4 {
            if m.test(i) {
                count += 1;
            }
        }
        black_box(count)
    });
}

// ============================================================================
// Mask Size: 8 elements (i32)
// ============================================================================

#[bench]
fn mask8_count_0pct(b: &mut Bencher) {
    let mask = mask32x8::splat(false);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask8_count_25pct(b: &mut Bencher) {
    let mask = mask32x8::from_array([true, false, false, false, true, false, false, false]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask8_count_50pct(b: &mut Bencher) {
    let mask = mask32x8::from_array([true, false, true, false, true, false, true, false]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask8_count_75pct(b: &mut Bencher) {
    let mask = mask32x8::from_array([true, true, true, false, true, true, true, false]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask8_count_100pct(b: &mut Bencher) {
    let mask = mask32x8::splat(true);
    b.iter(|| black_box(mask).count());
}

// Baseline: manual iteration for mask8
#[bench]
fn mask8_count_manual_50pct(b: &mut Bencher) {
    let mask = mask32x8::from_array([true, false, true, false, true, false, true, false]);
    b.iter(|| {
        let m = black_box(mask);
        let mut count = 0;
        for i in 0..8 {
            if m.test(i) {
                count += 1;
            }
        }
        black_box(count)
    });
}

// ============================================================================
// Mask Size: 16 elements (i32)
// ============================================================================

#[bench]
fn mask16_count_0pct(b: &mut Bencher) {
    let mask = mask32x16::splat(false);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask16_count_25pct(b: &mut Bencher) {
    let mask = mask32x16::from_array([
        true, false, false, false, true, false, false, false, true, false, false, false, true,
        false, false, false,
    ]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask16_count_50pct(b: &mut Bencher) {
    let mask = mask32x16::from_array([
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask16_count_75pct(b: &mut Bencher) {
    let mask = mask32x16::from_array([
        true, true, true, false, true, true, true, false, true, true, true, false, true, true,
        true, false,
    ]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask16_count_100pct(b: &mut Bencher) {
    let mask = mask32x16::splat(true);
    b.iter(|| black_box(mask).count());
}

// Baseline: manual iteration for mask16
#[bench]
fn mask16_count_manual_50pct(b: &mut Bencher) {
    let mask = mask32x16::from_array([
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ]);
    b.iter(|| {
        let m = black_box(mask);
        let mut count = 0;
        for i in 0..16 {
            if m.test(i) {
                count += 1;
            }
        }
        black_box(count)
    });
}

// ============================================================================
// Real-world scenario: filtering based on comparison
// ============================================================================

#[bench]
fn real_world_filter_count_f32x8(b: &mut Bencher) {
    let data = f32x8::from_array([1.0, 5.5, 3.2, 7.8, 2.1, 9.5, 4.3, 6.7]);
    let threshold = f32x8::splat(5.0);

    b.iter(|| {
        let d = black_box(data);
        let t = black_box(threshold);
        let mask = d.simd_gt(t);
        black_box(mask.count())
    });
}

#[bench]
fn real_world_filter_count_f32x16(b: &mut Bencher) {
    let data = f32x16::from_array([
        1.0, 5.5, 3.2, 7.8, 2.1, 9.5, 4.3, 6.7, 1.5, 5.2, 3.8, 7.1, 2.9, 9.2, 4.8, 6.1,
    ]);
    let threshold = f32x16::splat(5.0);

    b.iter(|| {
        let d = black_box(data);
        let t = black_box(threshold);
        let mask = d.simd_gt(t);
        black_box(mask.count())
    });
}

// ============================================================================
// Stress test: multiple counts in tight loop
// ============================================================================

#[bench]
fn stress_multiple_counts_mask8(b: &mut Bencher) {
    let masks = [
        mask32x8::from_array([true, false, true, false, true, false, true, false]),
        mask32x8::from_array([false, true, false, true, false, true, false, true]),
        mask32x8::from_array([true, true, false, false, true, true, false, false]),
        mask32x8::from_array([false, false, true, true, false, false, true, true]),
    ];

    b.iter(|| {
        let ms = black_box(&masks);
        let total = ms[0].count() + ms[1].count() + ms[2].count() + ms[3].count();
        black_box(total)
    });
}

// ============================================================================
// Cache behavior test: alternating access pattern
// ============================================================================

#[bench]
fn cache_alternating_access(b: &mut Bencher) {
    let mask1 = mask32x8::from_array([true, false, true, false, true, false, true, false]);
    let mask2 = mask32x8::from_array([false, true, false, true, false, true, false, true]);

    b.iter(|| {
        let m1 = black_box(mask1);
        let m2 = black_box(mask2);
        black_box(m1.count() + m2.count())
    });
}

// ============================================================================
// Test different element types (i64 vs i32)
// ============================================================================

#[bench]
fn mask4_i64_count_50pct(b: &mut Bencher) {
    let mask = mask64x4::from_array([true, false, true, false]);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn mask8_i64_count_50pct(b: &mut Bencher) {
    let mask = mask64x8::from_array([true, false, true, false, true, false, true, false]);
    b.iter(|| black_box(mask).count());
}

// ============================================================================
// Edge cases
// ============================================================================

#[bench]
fn edge_case_all_false_mask16(b: &mut Bencher) {
    let mask = mask32x16::splat(false);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn edge_case_all_true_mask16(b: &mut Bencher) {
    let mask = mask32x16::splat(true);
    b.iter(|| black_box(mask).count());
}

#[bench]
fn edge_case_single_true_mask16(b: &mut Bencher) {
    let mut arr = [false; 16];
    arr[7] = true;
    let mask = mask32x16::from_array(arr);
    b.iter(|| black_box(mask).count());
}
