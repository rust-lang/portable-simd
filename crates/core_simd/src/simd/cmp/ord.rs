use crate::simd::{
    Mask, Select, Simd, SimdElement,
    cmp::SimdPartialEq,
    ptr::{SimdConstPtr, SimdMutPtr},
};

/// Parallel `PartialOrd`.
pub trait SimdPartialOrd: SimdPartialEq {
    /// Test if each element is less than the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_lt<const N: usize>(
        self: Simd<Self, N>,
        other: Simd<Self, N>,
    ) -> Mask<<Self as SimdElement>::Mask, N>;

    /// Test if each element is less than or equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_le<const N: usize>(
        self: Simd<Self, N>,
        other: Simd<Self, N>,
    ) -> Mask<<Self as SimdElement>::Mask, N>;

    /// Test if each element is greater than the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_gt<const N: usize>(
        self: Simd<Self, N>,
        other: Simd<Self, N>,
    ) -> Mask<<Self as SimdElement>::Mask, N>;

    /// Test if each element is greater than or equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ge<const N: usize>(
        self: Simd<Self, N>,
        other: Simd<Self, N>,
    ) -> Mask<<Self as SimdElement>::Mask, N>;
}

/// Parallel `Ord`.
pub trait SimdOrd: SimdPartialOrd {
    /// Returns the element-wise maximum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_max<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Simd<Self, N>;

    /// Returns the element-wise minimum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_min<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Simd<Self, N>;

    /// Restrict each element to a certain interval.
    ///
    /// For each element, returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max` on any element.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_clamp<const N: usize>(
        self: Simd<Self, N>,
        min: Simd<Self, N>,
        max: Simd<Self, N>,
    ) -> Simd<Self, N>;
}

macro_rules! impl_integer {
    { $($integer:ty),* } => {
        $(
        impl SimdPartialOrd for $integer {
            #[inline]
            fn simd_lt<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_lt(self, other)) }
            }

            #[inline]
            fn simd_le<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_le(self, other)) }
            }

            #[inline]
            fn simd_gt<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_gt(self, other)) }
            }

            #[inline]
            fn simd_ge<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_ge(self, other)) }
            }
        }

        impl SimdOrd for $integer {
            #[inline]
            fn simd_max<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Simd<Self, N> {
                self.simd_lt(other).select(other, self)
            }

            #[inline]
            fn simd_min<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Simd<Self, N> {
                self.simd_gt(other).select(other, self)
            }

            #[inline]
            #[track_caller]
            fn simd_clamp<const N: usize>(
                self: Simd<Self, N>,
                min: Simd<Self, N>,
                max: Simd<Self, N>,
            ) -> Simd<Self, N> {
                assert!(
                    min.simd_le(max).all(),
                    "each element in `min` must be less than or equal to the corresponding element in `max`",
                );
                self.simd_max(min).simd_min(max)
            }
        }
        )*
    }
}

impl_integer! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

macro_rules! impl_float {
    { $($float:ty),* } => {
        $(
        impl SimdPartialOrd for $float {
            #[inline]
            fn simd_lt<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_lt(self, other)) }
            }

            #[inline]
            fn simd_le<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_le(self, other)) }
            }

            #[inline]
            fn simd_gt<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_gt(self, other)) }
            }

            #[inline]
            fn simd_ge<const N: usize>(
                self: Simd<Self, N>,
                other: Simd<Self, N>,
            ) -> Mask<<Self as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_ge(self, other)) }
            }
        }
        )*
    }
}

impl_float! { f16, f32, f64 }

impl<T> SimdPartialOrd for *const T {
    #[inline]
    fn simd_lt<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_lt(other.addr())
    }

    #[inline]
    fn simd_le<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_le(other.addr())
    }

    #[inline]
    fn simd_gt<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_gt(other.addr())
    }

    #[inline]
    fn simd_ge<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_ge(other.addr())
    }
}

impl<T> SimdOrd for *const T {
    #[inline]
    fn simd_max<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Simd<Self, N> {
        self.simd_lt(other).select(other, self)
    }

    #[inline]
    fn simd_min<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Simd<Self, N> {
        self.simd_gt(other).select(other, self)
    }

    #[inline]
    #[track_caller]
    fn simd_clamp<const N: usize>(
        self: Simd<Self, N>,
        min: Simd<Self, N>,
        max: Simd<Self, N>,
    ) -> Simd<Self, N> {
        assert!(
            min.simd_le(max).all(),
            "each element in `min` must be less than or equal to the corresponding element in `max`",
        );
        self.simd_max(min).simd_min(max)
    }
}

impl<T> SimdPartialOrd for *mut T {
    #[inline]
    fn simd_lt<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_lt(other.addr())
    }

    #[inline]
    fn simd_le<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_le(other.addr())
    }

    #[inline]
    fn simd_gt<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_gt(other.addr())
    }

    #[inline]
    fn simd_ge<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Mask<isize, N> {
        self.addr().simd_ge(other.addr())
    }
}

impl<T> SimdOrd for *mut T {
    #[inline]
    fn simd_max<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Simd<Self, N> {
        self.simd_lt(other).select(other, self)
    }

    #[inline]
    fn simd_min<const N: usize>(self: Simd<Self, N>, other: Simd<Self, N>) -> Simd<Self, N> {
        self.simd_gt(other).select(other, self)
    }

    #[inline]
    #[track_caller]
    fn simd_clamp<const N: usize>(
        self: Simd<Self, N>,
        min: Simd<Self, N>,
        max: Simd<Self, N>,
    ) -> Simd<Self, N> {
        assert!(
            min.simd_le(max).all(),
            "each element in `min` must be less than or equal to the corresponding element in `max`",
        );
        self.simd_max(min).simd_min(max)
    }
}
