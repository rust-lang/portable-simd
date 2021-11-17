use crate::simd::intrinsics::{
    simd_reduce_add_ordered, simd_reduce_and, simd_reduce_max, simd_reduce_min,
    simd_reduce_mul_ordered, simd_reduce_or, simd_reduce_xor,
};
use crate::simd::{Int, LaneCount, Simd, SimdElement, SupportedLaneCount};

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: Int,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Horizontal bitwise "and".  Returns the cumulative bitwise "and" across the lanes of
    /// the vector.
    #[inline]
    pub fn horizontal_and(self) -> T {
        unsafe { simd_reduce_and(self) }
    }

    /// Horizontal bitwise "or".  Returns the cumulative bitwise "or" across the lanes of
    /// the vector.
    #[inline]
    pub fn horizontal_or(self) -> T {
        unsafe { simd_reduce_or(self) }
    }

    /// Horizontal bitwise "xor".  Returns the cumulative bitwise "xor" across the lanes of
    /// the vector.
    #[inline]
    pub fn horizontal_xor(self) -> T {
        unsafe { simd_reduce_xor(self) }
    }
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Horizontal maximum.  Returns the maximum lane in the vector.
    ///
    /// Returns values based on equality, so a vector containing both `0.` and `-0.` may
    /// return either.  This function will not return `NaN` unless all lanes are `NaN`.
    #[inline]
    pub fn horizontal_max(self) -> T {
        unsafe { simd_reduce_max(self) }
    }

    /// Horizontal minimum.  Returns the minimum lane in the vector.
    ///
    /// Returns values based on equality, so a vector containing both `0.` and `-0.` may
    /// return either.  This function will not return `NaN` unless all lanes are `NaN`.
    #[inline]
    pub fn horizontal_min(self) -> T {
        unsafe { simd_reduce_min(self) }
    }
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    Self: HorizontalArith<Scalar = T>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Horizontal add.  Returns the sum of the lanes of the vector.
    #[inline]
    pub fn horizontal_sum(self) -> T {
        <Self as HorizontalArith>::horizontal_sum(self)
    }

    /// Horizontal multiply.  Returns the product of the lanes of the vector.
    #[inline]
    pub fn horizontal_product(self) -> T {
        <Self as HorizontalArith>::horizontal_product(self)
    }
}

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

pub trait HorizontalArith: Sealed {
    type Scalar: SimdElement;
    /// Horizontal add.  Returns the sum of the lanes of the vector.
    fn horizontal_sum(self) -> Self::Scalar;

    /// Horizontal multiply.  Returns the product of the lanes of the vector.
    fn horizontal_product(self) -> Self::Scalar;
}

macro_rules! impl_integer_reductions {
    { $scalar:ty } => {
        impl<const LANES: usize> HorizontalArith for Simd<$scalar, LANES>
        where
        LaneCount<LANES>: SupportedLaneCount,

{
            type Scalar = $scalar;
            /// Horizontal wrapping add.  Returns the sum of the lanes of the vector, with wrapping addition.
            #[inline]
            fn horizontal_sum(self) -> $scalar {
                unsafe { simd_reduce_add_ordered(self, 0) }
            }

            /// Horizontal wrapping multiply.  Returns the product of the lanes of the vector, with wrapping multiplication.
            #[inline]
            fn horizontal_product(self) -> $scalar {
                unsafe { simd_reduce_mul_ordered(self, 1) }
            }
        }
    }
}

impl_integer_reductions! { i8 }
impl_integer_reductions! { i16 }
impl_integer_reductions! { i32 }
impl_integer_reductions! { i64 }
impl_integer_reductions! { isize }
impl_integer_reductions! { u8 }
impl_integer_reductions! { u16 }
impl_integer_reductions! { u32 }
impl_integer_reductions! { u64 }
impl_integer_reductions! { usize }

macro_rules! impl_float_reductions {
    { $scalar:ty } => {
        impl<const LANES: usize> HorizontalArith for Simd<$scalar, LANES>
        where
        LaneCount<LANES>: SupportedLaneCount,

{
            type Scalar = $scalar;

            /// Horizontal add.  Returns the sum of the lanes of the vector.
            #[inline]
            fn horizontal_sum(self) -> $scalar {
                // LLVM sum is inaccurate on i586
                if cfg!(all(target_arch = "x86", not(target_feature = "sse2"))) {
                    self.as_array().iter().sum()
                } else {
                    unsafe { simd_reduce_add_ordered(self, 0.) }
                }
            }

            /// Horizontal multiply.  Returns the product of the lanes of the vector.
            #[inline]
            fn horizontal_product(self) -> $scalar {
                // LLVM product is inaccurate on i586
                if cfg!(all(target_arch = "x86", not(target_feature = "sse2"))) {
                    self.as_array().iter().product()
                } else {
                    unsafe { simd_reduce_mul_ordered(self, 1.) }
                }
            }
        }
    }
}

impl_float_reductions! { f32 }
impl_float_reductions! { f64 }
