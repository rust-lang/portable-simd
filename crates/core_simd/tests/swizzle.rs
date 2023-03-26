#![feature(portable_simd)]
use core_simd::simd::{Simd, Swizzle};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn swizzle() {
    struct Index;
    impl Swizzle<4, 4> for Index {
        const INDEX: [usize; 4] = [2, 1, 3, 0];
    }
    impl Swizzle<4, 2> for Index {
        const INDEX: [usize; 2] = [1, 1];
    }

    let vector = Simd::from_array([2, 4, 1, 9]);
    assert_eq!(Index::swizzle(vector).to_array(), [1, 4, 9, 2]);
    assert_eq!(Index::swizzle(vector).to_array(), [4, 4]);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn reverse() {
    let a = Simd::from_array([1, 2, 3, 4]);
    assert_eq!(a.reverse().to_array(), [4, 3, 2, 1]);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn rotate() {
    let a = Simd::from_array([1, 2, 3, 4]);
    assert_eq!(a.rotate_lanes_left::<0>().to_array(), [1, 2, 3, 4]);
    assert_eq!(a.rotate_lanes_left::<1>().to_array(), [2, 3, 4, 1]);
    assert_eq!(a.rotate_lanes_left::<2>().to_array(), [3, 4, 1, 2]);
    assert_eq!(a.rotate_lanes_left::<3>().to_array(), [4, 1, 2, 3]);
    assert_eq!(a.rotate_lanes_left::<4>().to_array(), [1, 2, 3, 4]);
    assert_eq!(a.rotate_lanes_left::<5>().to_array(), [2, 3, 4, 1]);
    assert_eq!(a.rotate_lanes_right::<0>().to_array(), [1, 2, 3, 4]);
    assert_eq!(a.rotate_lanes_right::<1>().to_array(), [4, 1, 2, 3]);
    assert_eq!(a.rotate_lanes_right::<2>().to_array(), [3, 4, 1, 2]);
    assert_eq!(a.rotate_lanes_right::<3>().to_array(), [2, 3, 4, 1]);
    assert_eq!(a.rotate_lanes_right::<4>().to_array(), [1, 2, 3, 4]);
    assert_eq!(a.rotate_lanes_right::<5>().to_array(), [4, 1, 2, 3]);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn interleave() {
    let a = Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7]);
    let b = Simd::from_array([8, 9, 10, 11, 12, 13, 14, 15]);
    let (lo, hi) = a.interleave(b);
    assert_eq!(lo.to_array(), [0, 8, 1, 9, 2, 10, 3, 11]);
    assert_eq!(hi.to_array(), [4, 12, 5, 13, 6, 14, 7, 15]);
    let (even, odd) = lo.deinterleave(hi);
    assert_eq!(even, a);
    assert_eq!(odd, b);
}

// portable-simd#298
#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn interleave_one() {
    let a = Simd::from_array([0]);
    let b = Simd::from_array([1]);
    let (lo, hi) = a.interleave(b);
    assert_eq!(lo.to_array(), [0]);
    assert_eq!(hi.to_array(), [1]);
    let (even, odd) = lo.deinterleave(hi);
    assert_eq!(even, a);
    assert_eq!(odd, b);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn slice() {
    let a = Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7]);
    let [lo, hi] = a.split();
    assert_eq!(lo.to_array(), [0, 1, 2, 3]);
    assert_eq!(hi.to_array(), [4, 5, 6, 7]);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn concat_equal_width() {
    let x = Simd::from_array([0, 1, 2, 3]);
    let y = Simd::from_array([4, 5, 6, 7]);
    let z = x.concat(y);
    assert_eq!(z.to_array(), [0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "all_lane_counts")]
fn concat_different_width() {
    let x = Simd::from_array([0, 1, 2, 3]);
    let y = Simd::from_array([4, 5]);
    let z = x.concat(y);
    assert_eq!(z.to_array(), [0, 1, 2, 3, 4, 5]);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn general_reverse() {
    let x = Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7]);
    // Swap adjacent lanes:
    assert_eq!(
        x.swizzle_to_xor_indices::<1>().to_array(),
        [1, 0, 3, 2, 5, 4, 7, 6]
    );
    // Swap lanes separated by distance 2:
    assert_eq!(
        x.swizzle_to_xor_indices::<2>().to_array(),
        [2, 3, 0, 1, 6, 7, 4, 5]
    );
    // Swap lanes separated by distance 4:
    assert_eq!(
        x.swizzle_to_xor_indices::<4>().to_array(),
        [4, 5, 6, 7, 0, 1, 2, 3]
    );
    // Reverse lanes, within each 4-lane group:
    assert_eq!(
        x.swizzle_to_xor_indices::<3>().to_array(),
        [3, 2, 1, 0, 7, 6, 5, 4]
    );
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn general_reverse_sum() {
    let x = Simd::from_array([0u32, 1, 2, 3, 4, 5, 6, 7]);
    let x = x + x.swizzle_to_xor_indices::<1>();
    let x = x + x.swizzle_to_xor_indices::<2>();
    let x = x + x.swizzle_to_xor_indices::<4>();
    assert_eq!(x.to_array(), [28, 28, 28, 28, 28, 28, 28, 28]);
}
