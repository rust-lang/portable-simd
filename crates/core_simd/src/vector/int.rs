#![allow(non_camel_case_types)]
use crate::simd::intrinsics;
use crate::simd::{Int, LaneCount, Mask, Simd, SimdAbs, SimdSignum, SupportedLaneCount};
use core::ops::Shr;

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: SInt,
    Self: Default,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Returns true for each positive lane and false if it is zero or negative.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn is_positive(self) -> Mask<T::Mask, LANES> {
        self.lanes_gt(Self::default())
    }

    /// Returns true for each negative lane and false if it is zero or positive.
    #[inline]
    pub fn is_negative(self) -> Mask<T::Mask, LANES> {
        self.lanes_lt(Self::default())
    }

    /// Lanewise saturating absolute value, implemented in Rust.
    /// As abs(), except the Scalar::MIN value becomes Scalar::MAX instead of itself.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "std")] use core_simd::Simd;
    /// # #[cfg(not(feature = "std"))] use core::simd::Simd;
    /// let xs = Simd::from_array([i32::MIN, -2, 0, 3]);
    /// let unsat = xs.abs();
    /// let sat = xs.saturating_abs();
    /// assert_eq!(unsat, Simd::from_array([i32::MIN, 2, 0, 3]));
    /// assert_eq!(sat, Simd::from_array([i32::MAX, 2, 0, 3]));
    /// ```
    #[inline]
    pub fn saturating_abs(self) -> Self {
        // arith shift for -1 or 0 mask based on sign bit, giving 2s complement
        let shr = T::BITS - 1;
        let m = self >> shr;
        unsafe { intrinsics::simd_xor(self, m).saturating_sub(m) }
    }

    /// Lanewise saturating negation, implemented in Rust.
    /// As neg(), except the Scalar::MIN value becomes Scalar::MAX instead of itself.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "std")] use core_simd::Simd;
    /// # #[cfg(not(feature = "std"))] use core::simd::Simd;
    /// let x = Simd::from_array([i32::MIN, -2, 3, i32::MAX]);
    /// let unsat = -x;
    /// let sat = x.saturating_neg();
    /// assert_eq!(unsat, Simd::from_array([i32::MIN, 2, -3, i32::MIN + 1]));
    /// assert_eq!(sat, Simd::from_array([i32::MAX, 2, -3, i32::MIN + 1]));
    /// ```
    #[inline]
    pub fn saturating_neg(self) -> Self {
        Self::default().saturating_sub(self)
    }
}

macro_rules! impl_int_abs {
    ($($ty:ty),+) => {
        $( impl<const LANES: usize> SimdAbs for Simd<$ty, LANES> where LaneCount<LANES>: SupportedLaneCount {

                #[inline]
                fn abs(self) -> Self {
                    const SHR: u32 = <$ty>::BITS - 1;
                    let m = self >> SHR;
                    (self^m) - m
                }
            }
            impl<const LANES: usize> SimdSignum for Simd<$ty, LANES> where LaneCount<LANES>: SupportedLaneCount {
            /// Returns numbers representing the sign of each lane.
            /// * `0` if the number is zero
            /// * `1` if the number is positive
            /// * `-1` if the number is negative
            #[inline]
            fn signum(self) -> Self {
                self.is_positive().select(
                    Self::splat(1),
                    self.is_negative().select(
                        unsafe { intrinsics::simd_neg(Self::splat(1)) },
                        Self::default(),
                    ),
                )
            }
        })+
    }
}
/// A 2s complement signed integer type.
pub trait SInt: Int {}
impl SInt for isize {}

impl SInt for i8 {}

impl SInt for i16 {}

impl SInt for i32 {}

impl SInt for i64 {}

impl<T, const LANES: usize> Shr<u32> for Simd<T, LANES>
where
    T: SInt,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        unsafe { intrinsics::simd_shr(self, intrinsics::simd_cast(Simd::splat(rhs))) }
    }
}

impl_int_abs! { i8, i16, i32, i64, isize }

/// Vector of two `isize` values
pub type isizex2 = Simd<isize, 2>;

/// Vector of four `isize` values
pub type isizex4 = Simd<isize, 4>;

/// Vector of eight `isize` values
pub type isizex8 = Simd<isize, 8>;

/// Vector of two `i16` values
pub type i16x2 = Simd<i16, 2>;

/// Vector of four `i16` values
pub type i16x4 = Simd<i16, 4>;

/// Vector of eight `i16` values
pub type i16x8 = Simd<i16, 8>;

/// Vector of 16 `i16` values
pub type i16x16 = Simd<i16, 16>;

/// Vector of 32 `i16` values
pub type i16x32 = Simd<i16, 32>;

/// Vector of two `i32` values
pub type i32x2 = Simd<i32, 2>;

/// Vector of four `i32` values
pub type i32x4 = Simd<i32, 4>;

/// Vector of eight `i32` values
pub type i32x8 = Simd<i32, 8>;

/// Vector of 16 `i32` values
pub type i32x16 = Simd<i32, 16>;

/// Vector of two `i64` values
pub type i64x2 = Simd<i64, 2>;

/// Vector of four `i64` values
pub type i64x4 = Simd<i64, 4>;

/// Vector of eight `i64` values
pub type i64x8 = Simd<i64, 8>;

/// Vector of four `i8` values
pub type i8x4 = Simd<i8, 4>;

/// Vector of eight `i8` values
pub type i8x8 = Simd<i8, 8>;

/// Vector of 16 `i8` values
pub type i8x16 = Simd<i8, 16>;

/// Vector of 32 `i8` values
pub type i8x32 = Simd<i8, 32>;

/// Vector of 64 `i8` values
pub type i8x64 = Simd<i8, 64>;
