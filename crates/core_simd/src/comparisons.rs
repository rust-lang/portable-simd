use crate::simd::intrinsics;
use crate::simd::{LaneCount, Mask, Simd, SimdElement, SupportedLaneCount};

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: SimdElement + PartialEq,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Test if each lane is equal to the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_eq(self, other: Self) -> Mask<T::Mask, LANES> {
        // Safety: `self` is a vector, and the result of the comparison
        // is always a valid mask.
        unsafe { Mask::from_int_unchecked(intrinsics::simd_eq(self, other)) }
    }

    /// Test if each lane is not equal to the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_ne(self, other: Self) -> Mask<T::Mask, LANES> {
        // Safety: `self` is a vector, and the result of the comparison
        // is always a valid mask.
        unsafe { Mask::from_int_unchecked(intrinsics::simd_ne(self, other)) }
    }
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: SimdElement + PartialOrd,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Test if each lane is less than the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_lt(self, other: Self) -> Mask<T::Mask, LANES> {
        // Safety: `self` is a vector, and the result of the comparison
        // is always a valid mask.
        unsafe { Mask::from_int_unchecked(intrinsics::simd_lt(self, other)) }
    }

    /// Test if each lane is greater than the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_gt(self, other: Self) -> Mask<T::Mask, LANES> {
        // Safety: `self` is a vector, and the result of the comparison
        // is always a valid mask.
        unsafe { Mask::from_int_unchecked(intrinsics::simd_gt(self, other)) }
    }

    /// Test if each lane is less than or equal to the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_le(self, other: Self) -> Mask<T::Mask, LANES> {
        // Safety: `self` is a vector, and the result of the comparison
        // is always a valid mask.
        unsafe { Mask::from_int_unchecked(intrinsics::simd_le(self, other)) }
    }

    /// Test if each lane is greater than or equal to the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_ge(self, other: Self) -> Mask<T::Mask, LANES> {
        // Safety: `self` is a vector, and the result of the comparison
        // is always a valid mask.
        unsafe { Mask::from_int_unchecked(intrinsics::simd_ge(self, other)) }
    }
}

macro_rules! impl_min_max_vector {
    { $type:ty } => {
        impl<const LANES: usize> Simd<$type, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            /// Returns the lane-wise minimum with other
            #[must_use = "method returns a new vector and does not mutate the original value"]
            #[inline]
            pub fn min(self, other: Self) -> Self {
                self.lanes_gt(other).select(other, self)
            }

            /// Returns the lane-wise maximum with other
            #[must_use = "method returns a new vector and does not mutate the original value"]
            #[inline]
            pub fn max(self, other: Self) -> Self {
                self.lanes_lt(other).select(other, self)
            }
        }
    }
}

impl_min_max_vector!(i8);
impl_min_max_vector!(i16);
impl_min_max_vector!(i32);
impl_min_max_vector!(i64);
impl_min_max_vector!(isize);
impl_min_max_vector!(u8);
impl_min_max_vector!(u16);
impl_min_max_vector!(u32);
impl_min_max_vector!(u64);
impl_min_max_vector!(usize);
