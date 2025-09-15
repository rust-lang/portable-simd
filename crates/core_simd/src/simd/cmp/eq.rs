use crate::simd::{
    LaneCount, Mask, Simd, SimdElement, SupportedLaneCount,
    ptr::{SimdConstPtr, SimdMutPtr},
};

/// Parallel `PartialEq`.
pub trait SimdPartialEq {
    /// The mask type returned by each comparison.
    type Mask;

    /// Test if each element is equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_eq(self, other: Self) -> Self::Mask;

    /// Test if each element is not equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ne(self, other: Self) -> Self::Mask;
}

macro_rules! impl_number {
    { $($number:ty),* } => {
        $(
        impl<const N: usize> SimdPartialEq for Simd<$number, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            type Mask = Mask<$number, N>;

            #[inline]
            fn simd_eq(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_eq(self, other)) }
            }

            #[inline]
            fn simd_ne(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_simd_unchecked(core::intrinsics::simd::simd_ne(self, other)) }
            }
        }
        )*
    }
}

impl_number! { f32, f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

// Masks compare lane-wise by comparing their underlying integer representations
impl<T, const N: usize> SimdPartialEq for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Mask = Self;

    #[inline]
    fn simd_eq(self, other: Self) -> Self::Mask {
        // Safety: `self` is a vector, and the result of the comparison is always a valid mask.
        unsafe {
            Self::from_simd_unchecked(core::intrinsics::simd::simd_eq(
                self.to_simd(),
                other.to_simd(),
            ))
        }
    }

    #[inline]
    fn simd_ne(self, other: Self) -> Self::Mask {
        // Safety: `self` is a vector, and the result of the comparison is always a valid mask.
        unsafe {
            Self::from_simd_unchecked(core::intrinsics::simd::simd_ne(
                self.to_simd(),
                other.to_simd(),
            ))
        }
    }
}

impl<T, const N: usize> SimdPartialEq for Simd<*const T, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Mask = Mask<*const T, N>;

    #[inline]
    fn simd_eq(self, other: Self) -> Self::Mask {
        self.addr().simd_eq(other.addr()).cast::<*const T>()
    }

    #[inline]
    fn simd_ne(self, other: Self) -> Self::Mask {
        self.addr().simd_ne(other.addr()).cast::<*const T>()
    }
}

impl<T, const N: usize> SimdPartialEq for Simd<*mut T, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Mask = Mask<*mut T, N>;

    #[inline]
    fn simd_eq(self, other: Self) -> Self::Mask {
        self.addr().simd_eq(other.addr()).cast::<*mut T>()
    }

    #[inline]
    fn simd_ne(self, other: Self) -> Self::Mask {
        self.addr().simd_ne(other.addr()).cast::<*mut T>()
    }
}
