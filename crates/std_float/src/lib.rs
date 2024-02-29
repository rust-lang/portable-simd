#![cfg_attr(feature = "as_crate", no_std)] // We are std!
#![cfg_attr(
    feature = "as_crate",
    feature(core_intrinsics),
    feature(portable_simd),
    allow(internal_features)
)]
#[cfg(not(feature = "as_crate"))]
use core::simd;
#[cfg(feature = "as_crate")]
use core_simd::simd;

use core::intrinsics::simd as intrinsics;

use simd::{LaneCount, Simd, SupportedLaneCount};

mod libm32;
mod libm64;

#[cfg(test)]
mod test_libm;

#[cfg(feature = "as_crate")]
mod experimental {
    pub trait Sealed {}
}

#[cfg(feature = "as_crate")]
use experimental as sealed;

use crate::sealed::Sealed;

/// This trait provides a possibly-temporary implementation of float functions
/// that may, in the absence of hardware support, canonicalize to calling an
/// operating system's `math.h` dynamically-loaded library (also known as a
/// shared object). As these conditionally require runtime support, they
/// should only appear in binaries built assuming OS support: `std`.
///
/// However, there is no reason SIMD types, in general, need OS support,
/// as for many architectures an embedded binary may simply configure that
/// support itself. This means these types must be visible in `core`
/// but have these functions available in `std`.
///
/// [`f32`] and [`f64`] achieve a similar trick by using "lang items", but
/// due to compiler limitations, it is harder to implement this approach for
/// abstract data types like [`Simd`]. From that need, this trait is born.
///
/// It is possible this trait will be replaced in some manner in the future,
/// when either the compiler or its supporting runtime functions are improved.
/// For now this trait is available to permit experimentation with SIMD float
/// operations that may lack hardware support, such as `mul_add`.
pub trait StdFloat: Sealed + Sized {
    /// Fused multiply-add.  Computes `(self * a) + b` with only one rounding error,
    /// yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated `fma` CPU instruction.  However, this is not always
    /// true, and will be heavily dependent on designing algorithms with specific target
    /// hardware in mind.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn mul_add(self, a: Self, b: Self) -> Self {
        unsafe { intrinsics::simd_fma(self, a, b) }
    }

    /// Produces a vector where every lane has the square root value
    /// of the equivalently-indexed lane in `self`
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn sqrt(self) -> Self {
        unsafe { intrinsics::simd_fsqrt(self) }
    }

    /// Returns the smallest integer greater than or equal to each lane.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    #[inline]
    fn ceil(self) -> Self {
        unsafe { intrinsics::simd_ceil(self) }
    }

    /// Returns the largest integer value less than or equal to each lane.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    #[inline]
    fn floor(self) -> Self {
        unsafe { intrinsics::simd_floor(self) }
    }

    /// Rounds to the nearest integer value. Ties round toward zero.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    #[inline]
    fn round(self) -> Self {
        unsafe { intrinsics::simd_round(self) }
    }

    /// Returns the floating point's integer value, with its fractional part removed.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    #[inline]
    fn trunc(self) -> Self {
        unsafe { intrinsics::simd_trunc(self) }
    }

    /// Returns the floating point's fractional value, with its integer part removed.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn fract(self) -> Self;
}

pub trait StdLibm: StdFloat {
    /// Signed integer type with the same number of bits as this floating point type.
    type IntType;

    /// Unsigned integer type with the same number of bits as this floating point type.
    type UintType;

    /// Computes the sine of a number (in radians).
    fn sin(self) -> Self;

    /// Computes the cosine of a number (in radians).
    fn cos(self) -> Self;

    /// Computes the tangent of a number (in radians).
    fn tan(self) -> Self;

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    fn asin(self) -> Self;

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    fn acos(self) -> Self;

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    fn atan(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    fn atan2(self, x: Self) -> Self;

    /// Returns `2^(self)`.
    fn exp2(self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(self) -> Self;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    fn exp_m1(self) -> Self;

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    fn ln_1p(self) -> Self;

    /// Returns the natural logarithm of the number.
    fn ln(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    fn log10(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn log(self, base: Self) -> Self;

    /// Raises a number to a floating point power.
    fn powf(self, y: Self) -> Self;

    /// Raises a number to an integer power.
    fn powi(self, y: Self::IntType) -> Self;

    /// Hyperbolic sine function.
    fn sinh(self) -> Self;

    /// Hyperbolic cosine function.
    fn cosh(self) -> Self;

    /// Hyperbolic tangent function.
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine function.
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine function.
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent function.
    fn atanh(self) -> Self;

    /// Returns the cube root of a number.
    fn cbrt(self) -> Self;

    /// Calculates the length of the hypotenuse of a right-angle triangle given
    /// legs of length `x` and `y`.
    fn hypot(self, other: Self) -> Self;
}

impl<const N: usize> Sealed for Simd<f32, N> where LaneCount<N>: SupportedLaneCount {}
impl<const N: usize> Sealed for Simd<f64, N> where LaneCount<N>: SupportedLaneCount {}

// We can safely just use all the defaults.
impl<const N: usize> StdFloat for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    /// Returns the floating point's fractional value, with its integer part removed.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    #[inline]
    fn fract(self) -> Self {
        self - self.trunc()
    }
}

impl<const N: usize> StdFloat for Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    /// Returns the floating point's fractional value, with its integer part removed.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    #[inline]
    fn fract(self) -> Self {
        self - self.trunc()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simd::prelude::*;

    #[test]
    fn everything_works() {
        let x = f32x4::from_array([0.1, 0.5, 0.6, -1.5]);
        let x2 = x + x;
        let _xc = x.ceil();
        let _xf = x.floor();
        let _xr = x.round();
        let _xt = x.trunc();
        let _xfma = x.mul_add(x, x);
        let _xsqrt = x.sqrt();
        let _ = x2.abs() * x2;
        let _ = x.sin();
    }
}
