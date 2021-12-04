use crate::simd::intrinsics;
use crate::simd::{LaneCount, Mask, Simd, SimdElement, SupportedLaneCount};

/// SIMD vector element types that can be compared for equality.
pub trait SimdEq: SimdElement {
    /// Implementation detail of [`Simd::lanes_eq`].
    fn lanes_eq<const LANES: usize>(
        lhs: Simd<Self, LANES>,
        rhs: Simd<Self, LANES>,
    ) -> Mask<Self::Mask, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::lanes_ne`].
    fn lanes_ne<const LANES: usize>(
        lhs: Simd<Self, LANES>,
        rhs: Simd<Self, LANES>,
    ) -> Mask<Self::Mask, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;
}

/// SIMD vector element types that can be compared for order.
///
/// Note that this trait has less strict requirements than [`Ord`].  The type does not need to form a
/// total order, but it does need to have a defined behavior for computing the minimum and maximum.
pub trait SimdOrd: SimdElement {
    /// Implementation detail of [`Simd::lanes_lt`].
    fn lanes_lt<const LANES: usize>(
        lhs: Simd<Self, LANES>,
        rhs: Simd<Self, LANES>,
    ) -> Mask<Self::Mask, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::lanes_gt`].
    fn lanes_gt<const LANES: usize>(
        lhs: Simd<Self, LANES>,
        rhs: Simd<Self, LANES>,
    ) -> Mask<Self::Mask, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::lanes_le`].
    fn lanes_le<const LANES: usize>(
        lhs: Simd<Self, LANES>,
        rhs: Simd<Self, LANES>,
    ) -> Mask<Self::Mask, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::lanes_ge`].
    fn lanes_ge<const LANES: usize>(
        lhs: Simd<Self, LANES>,
        rhs: Simd<Self, LANES>,
    ) -> Mask<Self::Mask, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::min`].
    fn min<const LANES: usize>(lhs: Simd<Self, LANES>, rhs: Simd<Self, LANES>) -> Simd<Self, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::max`].
    fn max<const LANES: usize>(lhs: Simd<Self, LANES>, rhs: Simd<Self, LANES>) -> Simd<Self, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::horizontal_min`].
    fn horizontal_min<const LANES: usize>(x: Simd<Self, LANES>) -> Self
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::horizontal_max`].
    fn horizontal_max<const LANES: usize>(x: Simd<Self, LANES>) -> Self
    where
        LaneCount<LANES>: SupportedLaneCount;

    /// Implementation detail of [`Simd::clamp`].
    #[inline]
    fn clamp<const LANES: usize>(
        mut x: Simd<Self, LANES>,
        min: Simd<Self, LANES>,
        max: Simd<Self, LANES>,
    ) -> Simd<Self, LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        assert!(
            min.lanes_le(max).all(),
            "each lane in `min` must be less than or equal to the corresponding lane in `max`",
        );
        x = x.lanes_lt(min).select(min, x);
        x = x.lanes_gt(max).select(max, x);
        x
    }
}

macro_rules! impl_integer {
    { $($type:ty),* } => {
        $(
        impl SimdEq for $type {
            #[inline]
            fn lanes_eq<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_eq(lhs, rhs)) }
            }

            #[inline]
            fn lanes_ne<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_ne(lhs, rhs)) }
            }
        }

        impl SimdOrd for $type {
            #[inline]
            fn lanes_lt<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_lt(lhs, rhs)) }
            }

            #[inline]
            fn lanes_gt<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_gt(lhs, rhs)) }
            }

            #[inline]
            fn lanes_le<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_le(lhs, rhs)) }
            }

            #[inline]
            fn lanes_ge<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_ge(lhs, rhs)) }
            }

            #[inline]
            fn min<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Simd<Self, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                // TODO consider using an intrinsic
                lhs.lanes_ge(rhs).select(rhs, lhs)
            }

            #[inline]
            fn max<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Simd<Self, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                // TODO consider using an intrinsic
                lhs.lanes_le(rhs).select(rhs, lhs)
            }

            #[inline]
            fn horizontal_min<const LANES: usize>(x: Simd<Self, LANES>) -> Self
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { intrinsics::simd_reduce_min(x) }
            }

            #[inline]
            fn horizontal_max<const LANES: usize>(x: Simd<Self, LANES>) -> Self
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { intrinsics::simd_reduce_max(x) }
            }
        }
        )*
    }
}

macro_rules! impl_float {
    { $($type:ty),* } => {
        $(
        impl SimdEq for $type {
            #[inline]
            fn lanes_eq<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_eq(lhs, rhs)) }
            }

            #[inline]
            fn lanes_ne<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_ne(lhs, rhs)) }
            }
        }

        impl SimdOrd for $type {
            #[inline]
            fn lanes_lt<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_lt(lhs, rhs)) }
            }

            #[inline]
            fn lanes_gt<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_gt(lhs, rhs)) }
            }

            #[inline]
            fn lanes_le<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_le(lhs, rhs)) }
            }

            #[inline]
            fn lanes_ge<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Mask<Self::Mask, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { Mask::from_int_unchecked(intrinsics::simd_ge(lhs, rhs)) }
            }

            #[inline]
            fn min<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Simd<Self, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                // TODO consider using an intrinsic
                lhs.is_nan()
                    .select(rhs, lhs.lanes_ge(rhs).select(rhs, lhs))
            }

            #[inline]
            fn max<const LANES: usize>(
                lhs: Simd<Self, LANES>,
                rhs: Simd<Self, LANES>,
            ) -> Simd<Self, LANES>
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                // TODO consider using an intrinsic
                lhs.is_nan()
                    .select(rhs, lhs.lanes_le(rhs).select(rhs, lhs))
            }

            #[inline]
            fn horizontal_min<const LANES: usize>(x: Simd<Self, LANES>) -> Self
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { intrinsics::simd_reduce_min(x) }
            }

            #[inline]
            fn horizontal_max<const LANES: usize>(x: Simd<Self, LANES>) -> Self
            where
                LaneCount<LANES>: SupportedLaneCount
            {
                unsafe { intrinsics::simd_reduce_max(x) }
            }
        }
        )*
    }
}

impl_integer! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }
impl_float! { f32, f64 }

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: SimdEq,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Test if each lane is equal to the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_eq(self, other: Self) -> Mask<T::Mask, LANES> {
        T::lanes_eq(self, other)
    }

    /// Test if each lane is not equal to the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_ne(self, other: Self) -> Mask<T::Mask, LANES> {
        T::lanes_ne(self, other)
    }
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: SimdOrd,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Test if each lane is less than the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_lt(self, other: Self) -> Mask<T::Mask, LANES> {
        T::lanes_lt(self, other)
    }

    /// Test if each lane is greater than the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_gt(self, other: Self) -> Mask<T::Mask, LANES> {
        T::lanes_gt(self, other)
    }

    /// Test if each lane is less than or equal to the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_le(self, other: Self) -> Mask<T::Mask, LANES> {
        T::lanes_le(self, other)
    }

    /// Test if each lane is greater than or equal to the corresponding lane in `other`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn lanes_ge(self, other: Self) -> Mask<T::Mask, LANES> {
        T::lanes_ge(self, other)
    }

    /// Returns the minimum of each lane.
    ///
    /// # Note
    /// For `f32` and `f64`, if one of the values is `NAN`, then the other value is returned.
    /// If the compared values are `0.0` and `-0.0`, the sign of the result is unspecified.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    pub fn min(self, other: Self) -> Self {
        T::min(self, other)
    }

    /// Returns the maximum of each lane.
    ///
    /// # Note
    /// For `f32` and `f64`, if one of the values is `NAN`, then the other value is returned.
    /// If the compared values are `0.0` and `-0.0`, the sign of the result is unspecified.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    pub fn max(self, other: Self) -> Self {
        T::max(self, other)
    }

    /// Restrict each lane to a certain interval.
    ///
    /// For each lane in `self`, returns the corresponding lane in `max` if the lane is
    /// greater than `max`, and the corresponding lane in `min` if the lane is less
    /// than `min`. Otherwise returns the lane in `self`.
    ///
    /// # Note
    /// For `f32` and `f64`, if any value is `NAN`, then the other value is returned.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        T::clamp(self, min, max)
    }

    /// Horizontal maximum.  Returns the maximum lane in the vector.
    ///
    /// # Note
    /// For `f32` and `f64`, only returns `NAN` if all lanes are `NAN`.
    /// If the vector contains both `0.0` and `-0.0` and the result is 0, the sign of the result is
    /// unspecified.
    #[inline]
    pub fn horizontal_max(self) -> T {
        T::horizontal_max(self)
    }

    /// Horizontal minimum.  Returns the minimum lane in the vector.
    ///
    /// # Note
    /// For `f32` and `f64`, only returns `NAN` if all lanes are `NAN`.
    /// If the vector contains both `0.0` and `-0.0` and the result is 0, the sign of the result is
    /// unspecified.
    #[inline]
    pub fn horizontal_min(self) -> T {
        T::horizontal_min(self)
    }
}
