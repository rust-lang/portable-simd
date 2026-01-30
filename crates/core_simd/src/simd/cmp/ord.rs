use crate::simd::{
    Mask, Select, Simd, SimdElement,
    cmp::SimdPartialEq,
    ptr::{SimdConstPtr, SimdMutPtr},
};

/// Parallel `PartialOrd`.
pub trait SimdPartialOrd<T, const N: usize>: SimdPartialEq<T, N>
where
    T: SimdElement,
{
    /// Test if each element is less than the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_lt(self, other: Self) -> Mask<<T as SimdElement>::Mask, N>;

    /// Test if each element is less than or equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_le(self, other: Self) -> Mask<<T as SimdElement>::Mask, N>;

    /// Test if each element is greater than the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_gt(self, other: Self) -> Mask<<T as SimdElement>::Mask, N>;

    /// Test if each element is greater than or equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ge(self, other: Self) -> Mask<<T as SimdElement>::Mask, N>;
}

/// Parallel `Ord`.
pub trait SimdOrd<T, const N: usize>: SimdPartialOrd<T, N>
where
    T: SimdElement,
{
    /// Returns the element-wise maximum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_max(self, other: Self) -> Self;

    /// Returns the element-wise minimum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_min(self, other: Self) -> Self;

    /// Restrict each element to a certain interval.
    ///
    /// For each element, returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max` on any element.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_clamp(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_integer {
    { $($integer:ty),* } => {
        $(
        impl<const N: usize> SimdPartialOrd<$integer, N> for Simd<$integer, N>
        {
            #[inline]
            fn simd_lt(self, other: Self) -> Mask<<$integer as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_lt(self, other)) }
            }

            #[inline]
            fn simd_le(self, other: Self) -> Mask<<$integer as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_le(self, other)) }
            }

            #[inline]
            fn simd_gt(self, other: Self) -> Mask<<$integer as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_gt(self, other)) }
            }

            #[inline]
            fn simd_ge(self, other: Self) -> Mask<<$integer as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_ge(self, other)) }
            }
        }

        impl<const N: usize> SimdOrd<$integer, N> for Simd<$integer, N>
        {
            #[inline]
            fn simd_max(self, other: Self) -> Self {
                self.simd_lt(other).select(other, self)
            }

            #[inline]
            fn simd_min(self, other: Self) -> Self {
                self.simd_gt(other).select(other, self)
            }

            #[inline]
            #[track_caller]
            fn simd_clamp(self, min: Self, max: Self) -> Self {
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
        impl<const N: usize> SimdPartialOrd<$float, N> for Simd<$float, N>
        {
            #[inline]
            fn simd_lt(self, other: Self) -> Mask<<$float as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_lt(self, other)) }
            }

            #[inline]
            fn simd_le(self, other: Self) -> Mask<<$float as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_le(self, other)) }
            }

            #[inline]
            fn simd_gt(self, other: Self) -> Mask<<$float as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_gt(self, other)) }
            }

            #[inline]
            fn simd_ge(self, other: Self) -> Mask<<$float as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_ge(self, other)) }
            }
        }
        )*
    }
}

impl_float! { f32, f64 }

macro_rules! impl_mask {
    { $($integer:ty),* } => {
        $(
        impl<const N: usize> SimdPartialOrd<$integer, N> for Mask<$integer, N>
        {
            #[inline]
            fn simd_lt(self, other: Self) -> Mask<<$integer as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Self::from_simd_unchecked(core::intrinsics::simd::simd_lt(self.to_simd(), other.to_simd())) }
            }

            #[inline]
            fn simd_le(self, other: Self) -> Mask<<$integer as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Self::from_simd_unchecked(core::intrinsics::simd::simd_le(self.to_simd(), other.to_simd())) }
            }

            #[inline]
            fn simd_gt(self, other: Self) -> Mask<<$integer as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Self::from_simd_unchecked(core::intrinsics::simd::simd_gt(self.to_simd(), other.to_simd())) }
            }

            #[inline]
            fn simd_ge(self, other: Self) -> Mask<<$integer as SimdElement>::Mask, N> {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Self::from_simd_unchecked(core::intrinsics::simd::simd_ge(self.to_simd(), other.to_simd())) }
            }
        }

        impl<const N: usize> SimdOrd<$integer, N> for Mask<$integer, N>
        {
            #[inline]
            fn simd_max(self, other: Self) -> Self {
                self.simd_gt(other).select(other, self)
            }

            #[inline]
            fn simd_min(self, other: Self) -> Self {
                self.simd_lt(other).select(other, self)
            }

            #[inline]
            #[track_caller]
            fn simd_clamp(self, min: Self, max: Self) -> Self {
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

impl_mask! { i8, i16, i32, i64, isize }

impl<T, const N: usize> SimdPartialOrd<*const T, N> for Simd<*const T, N> {
    #[inline]
    fn simd_lt(self, other: Self) -> Mask<isize, N> {
        self.addr().simd_lt(other.addr())
    }

    #[inline]
    fn simd_le(self, other: Self) -> Mask<isize, N> {
        self.addr().simd_le(other.addr())
    }

    #[inline]
    fn simd_gt(self, other: Self) -> Mask<isize, N> {
        self.addr().simd_gt(other.addr())
    }

    #[inline]
    fn simd_ge(self, other: Self) -> Mask<isize, N> {
        self.addr().simd_ge(other.addr())
    }
}

impl<T, const N: usize> SimdOrd<*const T, N> for Simd<*const T, N> {
    #[inline]
    fn simd_max(self, other: Self) -> Self {
        self.simd_lt(other).select(other, self)
    }

    #[inline]
    fn simd_min(self, other: Self) -> Self {
        self.simd_gt(other).select(other, self)
    }

    #[inline]
    #[track_caller]
    fn simd_clamp(self, min: Self, max: Self) -> Self {
        assert!(
            min.simd_le(max).all(),
            "each element in `min` must be less than or equal to the corresponding element in `max`",
        );
        self.simd_max(min).simd_min(max)
    }
}

impl<T, const N: usize> SimdPartialOrd<*mut T, N> for Simd<*mut T, N> {
    #[inline]
    fn simd_lt(self, other: Self) -> Mask<isize, N> {
        self.addr().simd_lt(other.addr())
    }

    #[inline]
    fn simd_le(self, other: Self) -> Mask<isize, N> {
        self.addr().simd_le(other.addr())
    }

    #[inline]
    fn simd_gt(self, other: Self) -> Mask<isize, N> {
        self.addr().simd_gt(other.addr())
    }

    #[inline]
    fn simd_ge(self, other: Self) -> Mask<isize, N> {
        self.addr().simd_ge(other.addr())
    }
}

impl<T, const N: usize> SimdOrd<*mut T, N> for Simd<*mut T, N> {
    #[inline]
    fn simd_max(self, other: Self) -> Self {
        self.simd_lt(other).select(other, self)
    }

    #[inline]
    fn simd_min(self, other: Self) -> Self {
        self.simd_gt(other).select(other, self)
    }

    #[inline]
    #[track_caller]
    fn simd_clamp(self, min: Self, max: Self) -> Self {
        assert!(
            min.simd_le(max).all(),
            "each element in `min` must be less than or equal to the corresponding element in `max`",
        );
        self.simd_max(min).simd_min(max)
    }
}
