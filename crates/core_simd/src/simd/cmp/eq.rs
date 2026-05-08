use crate::simd::{
    Mask, Simd, SimdElement,
    ptr::{SimdConstPtr, SimdMutPtr},
};

/// Parallel `PartialEq`.
pub trait SimdPartialEq: SimdElement {
    /// Test if each element is equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_eq<const N: usize>(
        self: Simd<Self, N>,
        other: Simd<Self, N>,
    ) -> Mask<<Self as SimdElement>::Mask, N>;

    /// Test if each element is not equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ne<const N: usize>(
        self: Simd<Self, N>,
        other: Simd<Self, N>,
    ) -> Mask<<Self as SimdElement>::Mask, N>;
}

macro_rules! impl_number {
    { $($number:ty),* } => {
        $(
        impl SimdPartialEq for $number {
            #[inline]
            fn simd_eq<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_eq(self, other)) }
            }

            #[inline]
            fn simd_ne<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_ne(self, other)) }
            }
        }
        )*
    }
}

impl_number! { f16, f32, f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

impl<T> SimdPartialEq for *const T {
    #[inline]
    fn simd_eq<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_eq(other.addr())
    }

    #[inline]
    fn simd_ne<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_ne(other.addr())
    }
}

impl<T> SimdPartialEq for *mut T {
    #[inline]
    fn simd_eq<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_eq(other.addr())
    }

    #[inline]
    fn simd_ne<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_ne(other.addr())
    }
}
