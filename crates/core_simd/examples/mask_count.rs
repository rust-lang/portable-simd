//! Demonstrates `Mask::count()` to count matching elements.

#![feature(portable_simd)]
use cmp::SimdPartialOrd;
use core_simd::simd::*;

fn main() {
    // Count elements above threshold
    let data = [1.0, 5.0, 3.0, 7.0, 2.0, 9.0, 4.0, 6.0];
    let values = f32x8::from_array(data);
    let threshold = f32x8::splat(5.0);
    let mask = values.simd_gt(threshold);
    println!("Values above 5.0: {}", mask.count());

    // Use count() to pre-allocate for filtering
    let chunks = data.chunks_exact(8);
    let mut total = 0;
    for chunk in chunks.clone() {
        let v = f32x8::from_slice(chunk);
        total += v.simd_gt(f32x8::splat(5.0)).count();
    }

    let mut results = Vec::with_capacity(total);
    for chunk in chunks {
        let v = f32x8::from_slice(chunk);
        let m = v.simd_gt(f32x8::splat(5.0));
        for (i, &val) in chunk.iter().enumerate() {
            if m.test(i) {
                results.push(val);
            }
        }
    }

    println!("Filtered: {:?}", results);
}
