#![feature(portable_simd)]

use core_simd::simd::{
    Simd,
    ptr::{self, SimdConstPtr, SimdMutPtr},
};

macro_rules! common_tests {
    { $constness:ident } => {
        test_helpers::test_lanes! {
            fn is_null<const LANES: usize>() {
                test_helpers::test_unary_mask_elementwise(
                    &|x: Simd::<*$constness u32, LANES>| x.is_null(),
                    &<*$constness u32>::is_null,
                    &|_| true,
                );
            }

            fn addr<const LANES: usize>() {
                test_helpers::test_unary_elementwise(
                    &|x: Simd::<*$constness u32, LANES>| x.addr(),
                    &<*$constness u32>::addr,
                    &|_| true,
                );
            }

            fn with_addr<const LANES: usize>() {
                test_helpers::test_binary_elementwise(
                    &|x: Simd::<*$constness u32, LANES>, y: Simd<usize, LANES>| x.with_addr(y),
                    &<*$constness u32>::with_addr,
                    &|_, _| true,
                );
            }

            fn expose_provenance<const LANES: usize>() {
                test_helpers::test_unary_elementwise(
                    &|x: Simd::<*$constness u32, LANES>| x.expose_provenance(),
                    &<*$constness u32>::expose_provenance,
                    &|_| true,
                );
            }

            fn wrapping_offset<const LANES: usize>() {
                test_helpers::test_binary_elementwise(
                    &|x: Simd::<*$constness u32, LANES>, y: Simd<isize, LANES>| x.wrapping_offset(y),
                    &<*$constness u32>::wrapping_offset,
                    &|_, _| true,
                );
            }

            fn wrapping_add<const LANES: usize>() {
                test_helpers::test_binary_elementwise(
                    &|x: Simd::<*$constness u32, LANES>, y: Simd<usize, LANES>| x.wrapping_add(y),
                    &<*$constness u32>::wrapping_add,
                    &|_, _| true,
                );
            }

            fn wrapping_sub<const LANES: usize>() {
                test_helpers::test_binary_elementwise(
                    &|x: Simd::<*$constness u32, LANES>, y: Simd<usize, LANES>| x.wrapping_sub(y),
                    &<*$constness u32>::wrapping_sub,
                    &|_, _| true,
                );
            }
        }
    }
}

mod const_ptr {
    use super::*;
    common_tests! { const }

    test_helpers::test_lanes! {
        fn cast_mut<const LANES: usize>() {
            test_helpers::test_unary_elementwise(
                &|x: Simd::<*const u32, LANES>| x.cast_mut(),
                &<*const u32>::cast_mut,
                &|_| true,
            );
        }

        fn with_exposed_provenance<const LANES: usize>() {
            test_helpers::test_unary_elementwise(
                &ptr::with_exposed_provenance::<u32, LANES>,
                &core::ptr::with_exposed_provenance::<u32>,
                &|_| true,
            );
        }

        fn without_provenance<const LANES: usize>() {
            test_helpers::test_unary_elementwise(
                &ptr::without_provenance::<u32, LANES>,
                &core::ptr::without_provenance::<u32>,
                &|_| true,
            );
        }

    }
}

mod mut_ptr {
    use super::*;
    common_tests! { mut }

    test_helpers::test_lanes! {
        fn cast_const<const LANES: usize>() {
            test_helpers::test_unary_elementwise(
                &|x: Simd::<*mut u32, LANES>| x.cast_const(),
                &<*mut u32>::cast_const,
                &|_| true,
            );
        }

        fn with_exposed_provenance<const LANES: usize>() {
            test_helpers::test_unary_elementwise(
                &ptr::with_exposed_provenance_mut::<u32, LANES>,
                &core::ptr::with_exposed_provenance_mut::<u32>,
                &|_| true,
            );
        }

        fn without_provenance<const LANES: usize>() {
            test_helpers::test_unary_elementwise(
                &ptr::without_provenance_mut::<u32, LANES>,
                &core::ptr::without_provenance_mut::<u32>,
                &|_| true,
            );
        }

    }
}
