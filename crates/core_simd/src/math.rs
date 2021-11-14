use crate::simd::intrinsics;
use crate::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;
impl<T, const LANES: usize> Sealed for Simd<T, LANES>
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: Int,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Lanewise saturating add.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "std")] use core_simd::Simd;
    /// # #[cfg(not(feature = "std"))] use core::simd::Simd;
    /// let x = Simd::from_array([i32::MIN, 0, 1, i32::MAX]);
    /// let max = Simd::splat(i32::MAX);
    /// let unsat = x + max;
    /// let sat = x.saturating_add(max);
    /// assert_eq!(unsat, Simd::from_array([-1, i32::MAX, i32::MIN, -2]));
    /// assert_eq!(sat, Simd::from_array([-1, i32::MAX, i32::MAX, i32::MAX]));
    /// ```
    #[inline]
    pub fn saturating_add(self, other: Self) -> Self {
        unsafe { intrinsics::simd_saturating_add(self, other) }
    }

    /// Lanewise saturating subtract.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "std")] use core_simd::Simd;
    /// # #[cfg(not(feature = "std"))] use core::simd::Simd;
    /// let x = Simd::from_array([i32::MIN, -2, -1, i32::MAX]);
    /// let max = Simd::splat(i32::MAX);
    /// let unsat = x - max;
    /// let sat = x.saturating_sub(max);
    /// assert_eq!(unsat, Simd::from_array([1, i32::MAX, i32::MIN, 0]));
    /// assert_eq!(sat, Simd::from_array([i32::MIN, i32::MIN, i32::MIN, 0]));
    #[inline]
    pub fn saturating_sub(self, other: Self) -> Self {
        unsafe { intrinsics::simd_saturating_sub(self, other) }
    }
}

pub trait Int: SimdElement + PartialOrd {
    const BITS: u32;
}

impl Int for u8 {
    const BITS: u32 = 8;
}

impl Int for i8 {
    const BITS: u32 = 8;
}

impl Int for u16 {
    const BITS: u32 = 16;
}

impl Int for i16 {
    const BITS: u32 = 16;
}

impl Int for u32 {
    const BITS: u32 = 32;
}

impl Int for i32 {
    const BITS: u32 = 32;
}

impl Int for u64 {
    const BITS: u32 = 64;
}

impl Int for i64 {
    const BITS: u32 = 64;
}

impl Int for usize {
    const BITS: u32 = usize::BITS;
}

impl Int for isize {
    const BITS: u32 = isize::BITS;
}

pub trait SimdSignum: Sealed {
    fn signum(self) -> Self;
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    Self: SimdSignum,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Replaces each lane with a number that represents its sign.
    ///
    /// For floats:
    /// * `1.0` if the number is positive, `+0.0`, or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0`, or `NEG_INFINITY`
    /// * `NAN` if the number is `NAN`
    ///
    /// For signed integers:
    /// * `0` if the number is zero
    /// * `1` if the number is positive
    /// * `-1` if the number is negative
    #[inline]
    pub fn signum(self) -> Self {
        <Self as SimdSignum>::signum(self)
    }
}

pub trait SimdAbs: Sealed {
    /// Returns a vector where every lane has the absolute value of the
    /// equivalent index in `self`.
    fn abs(self) -> Self;
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    Self: SimdAbs,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Returns a vector where every lane has the absolute value of the
    /// equivalent index in `self`.
    ///
    /// # Examples
    /// ```rust
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "std")] use core_simd::Simd;
    /// # #[cfg(not(feature = "std"))] use core::simd::Simd;
    /// let xs = Simd::from_array([i32::MIN, i32::MIN +1, -5, 0]);
    /// assert_eq!(xs.abs(), Simd::from_array([i32::MIN, i32::MAX, 5, 0]));
    /// ```
    #[inline]
    pub fn abs(self) -> Self {
        <Self as SimdAbs>::abs(self)
    }
}
