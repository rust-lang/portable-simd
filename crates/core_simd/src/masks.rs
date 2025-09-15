//! Types and traits associated with masking elements of vectors.
//! Types representing
#![allow(non_camel_case_types)]

use crate::core_simd::vector::sealed::MaskElement;
use crate::simd::{LaneCount, Select, Simd, SimdElement, SupportedLaneCount};
use core::cmp::Ordering;
use core::{fmt, mem};

pub(crate) trait FixEndianness {
    fn fix_endianness(self) -> Self;
}

macro_rules! impl_fix_endianness {
    { $($int:ty),* } => {
        $(
        impl FixEndianness for $int {
            #[inline(always)]
            fn fix_endianness(self) -> Self {
                if cfg!(target_endian = "big") {
                    <$int>::reverse_bits(self)
                } else {
                    self
                }
            }
        }
        )*
    }
}

impl_fix_endianness! { u8, u16, u32, u64 }

/// A SIMD vector mask for `N` elements matching the element type `T`.
///
/// Masks represent boolean inclusion/exclusion on a per-element basis.
///
/// The layout of this type is unspecified, and may change between platforms
/// and/or Rust versions, and code should not assume that it is equivalent to
/// `[T; N]`.
#[repr(transparent)]
pub struct Mask<T, const N: usize>(Simd<T::Mask, N>)
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount;

impl<T, const N: usize> Copy for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
}

impl<T, const N: usize> Clone for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const N: usize> Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    /// Constructs a mask by setting all elements to the given value.
    #[inline]
    #[rustc_const_unstable(feature = "portable_simd", issue = "86656")]
    pub const fn splat(value: bool) -> Self {
        Self(Simd::splat(if value {
            <T::Mask as MaskElement>::TRUE
        } else {
            <T::Mask as MaskElement>::FALSE
        }))
    }

    /// Converts an array of bools to a SIMD mask.
    #[inline]
    pub fn from_array(array: [bool; N]) -> Self {
        // SAFETY: Rust's bool has a layout of 1 byte (u8) with a value of
        //     true:    0b_0000_0001
        //     false:   0b_0000_0000
        // Thus, an array of bools is also a valid array of bytes: [u8; N]
        // This would be hypothetically valid as an "in-place" transmute,
        // but these are "dependently-sized" types, so copy elision it is!
        unsafe {
            let bytes: [u8; N] = mem::transmute_copy(&array);
            let bools: Simd<i8, N> =
                core::intrinsics::simd::simd_ne(Simd::from_array(bytes), Simd::splat(0u8));
            Mask::from_simd_unchecked(core::intrinsics::simd::simd_cast(bools))
        }
    }

    /// Converts a SIMD mask to an array of bools.
    #[inline]
    pub fn to_array(self) -> [bool; N] {
        // This follows mostly the same logic as from_array.
        // SAFETY: Rust's bool has a layout of 1 byte (u8) with a value of
        //     true:    0b_0000_0001
        //     false:   0b_0000_0000
        // Thus, an array of bools is also a valid array of bytes: [u8; N]
        // Since our masks are equal to integers where all bits are set,
        // we can simply convert them to i8s, and then bitand them by the
        // bitpattern for Rust's "true" bool.
        // This would be hypothetically valid as an "in-place" transmute,
        // but these are "dependently-sized" types, so copy elision it is!
        unsafe {
            let mut bytes: Simd<i8, N> = core::intrinsics::simd::simd_cast(self.to_simd());
            bytes &= Simd::splat(1i8);
            mem::transmute_copy(&bytes)
        }
    }

    /// Converts a vector of integers to a mask, where 0 represents `false` and -1
    /// represents `true`.
    ///
    /// # Safety
    /// All elements must be either 0 or -1.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub unsafe fn from_simd_unchecked(value: Simd<T::Mask, N>) -> Self {
        // Safety: the caller must confirm this invariant
        unsafe {
            core::intrinsics::assume(<T::Mask as MaskElement>::valid(value));
        }
        Self(value)
    }

    /// Converts a vector of integers to a mask, where 0 represents `false` and -1
    /// represents `true`.
    ///
    /// # Panics
    /// Panics if any element is not 0 or -1.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    #[track_caller]
    pub fn from_simd(value: Simd<T::Mask, N>) -> Self {
        assert!(
            <T::Mask as MaskElement>::valid(value),
            "all values must be either 0 or -1",
        );
        // Safety: the validity has been checked
        unsafe { Self::from_simd_unchecked(value) }
    }

    /// Converts the mask to a vector of integers, where 0 represents `false` and -1
    /// represents `true`.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    pub fn to_simd(self) -> Simd<T::Mask, N> {
        self.0
    }

    /// Converts the mask to a mask of any other element size.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn cast<U: SimdElement>(self) -> Mask<U, N> {
        // Safety: mask elements are integers; cast between underlying mask widths
        let ints: Simd<U::Mask, N> = unsafe { core::intrinsics::simd::simd_as(self.0) };
        Mask(ints)
    }

    /// Tests the value of the specified element.
    ///
    /// # Safety
    /// `index` must be less than `self.len()`.
    #[inline]
    #[must_use = "method returns a new bool and does not mutate the original value"]
    pub unsafe fn test_unchecked(&self, index: usize) -> bool {
        // Safety: the caller must confirm this invariant
        unsafe {
            <T::Mask as MaskElement>::eq(
                *self.0.as_array().get_unchecked(index),
                <T::Mask as MaskElement>::TRUE,
            )
        }
    }

    /// Tests the value of the specified element.
    ///
    /// # Panics
    /// Panics if `index` is greater than or equal to the number of elements in the vector.
    #[inline]
    #[must_use = "method returns a new bool and does not mutate the original value"]
    #[track_caller]
    pub fn test(&self, index: usize) -> bool {
        <T::Mask as MaskElement>::eq(self.0[index], <T::Mask as MaskElement>::TRUE)
    }

    /// Sets the value of the specified element.
    ///
    /// # Safety
    /// `index` must be less than `self.len()`.
    #[inline]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
        // Safety: the caller must confirm this invariant
        unsafe {
            *self.0.as_mut_array().get_unchecked_mut(index) = if value {
                <T::Mask as MaskElement>::TRUE
            } else {
                <T::Mask as MaskElement>::FALSE
            }
        }
    }

    /// Sets the value of the specified element.
    ///
    /// # Panics
    /// Panics if `index` is greater than or equal to the number of elements in the vector.
    #[inline]
    #[track_caller]
    pub fn set(&mut self, index: usize, value: bool) {
        self.0[index] = if value {
            <T::Mask as MaskElement>::TRUE
        } else {
            <T::Mask as MaskElement>::FALSE
        }
    }

    /// Returns true if any element is set, or false otherwise.
    #[inline]
    #[must_use = "method returns a new bool and does not mutate the original value"]
    pub fn any(self) -> bool {
        // Safety: `self` is a mask vector
        unsafe { core::intrinsics::simd::simd_reduce_any(self.0) }
    }

    /// Returns true if all elements are set, or false otherwise.
    #[inline]
    #[must_use = "method returns a new bool and does not mutate the original value"]
    pub fn all(self) -> bool {
        // Safety: `self` is a mask vector
        unsafe { core::intrinsics::simd::simd_reduce_all(self.0) }
    }

    /// Creates a bitmask from a mask.
    ///
    /// Each bit is set if the corresponding element in the mask is `true`.
    #[inline]
    #[must_use = "method returns a new integer and does not mutate the original value"]
    pub fn to_bitmask(self) -> u64 {
        const {
            assert!(N <= 64, "number of elements can't be greater than 64");
        }

        #[inline]
        unsafe fn to_bitmask_impl<T, U: FixEndianness, const M: usize, const N: usize>(
            mask: Mask<T, N>,
        ) -> U
        where
            T: SimdElement,
            LaneCount<M>: SupportedLaneCount,
            LaneCount<N>: SupportedLaneCount,
        {
            let resized = mask.resize::<M>(false);

            // Safety: `resized` is an integer vector with length M, which must match T
            let bitmask: U = unsafe { core::intrinsics::simd::simd_bitmask(resized.0) };

            // LLVM assumes bit order should match endianness
            bitmask.fix_endianness()
        }

        // TODO modify simd_bitmask to zero-extend output, making this unnecessary
        if N <= 8 {
            // Safety: bitmask matches length
            unsafe { to_bitmask_impl::<T, u8, 8, N>(self) as u64 }
        } else if N <= 16 {
            // Safety: bitmask matches length
            unsafe { to_bitmask_impl::<T, u16, 16, N>(self) as u64 }
        } else if N <= 32 {
            // Safety: bitmask matches length
            unsafe { to_bitmask_impl::<T, u32, 32, N>(self) as u64 }
        } else {
            // Safety: bitmask matches length
            unsafe { to_bitmask_impl::<T, u64, 64, N>(self) }
        }
    }

    /// Creates a mask from a bitmask.
    ///
    /// For each bit, if it is set, the corresponding element in the mask is set to `true`.
    /// If the mask contains more than 64 elements, the remainder are set to `false`.
    #[inline]
    #[must_use = "method returns a new mask and does not mutate the original value"]
    pub fn from_bitmask(bitmask: u64) -> Self {
        Self(bitmask.select(
            Simd::splat(<T::Mask as MaskElement>::TRUE),
            Simd::splat(<T::Mask as MaskElement>::FALSE),
        ))
    }

    /// Finds the index of the first set element.
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::mask32x8;
    /// assert_eq!(mask32x8::splat(false).first_set(), None);
    /// assert_eq!(mask32x8::splat(true).first_set(), Some(0));
    ///
    /// let mask = mask32x8::from_array([false, true, false, false, true, false, false, true]);
    /// assert_eq!(mask.first_set(), Some(1));
    /// ```
    #[inline]
    #[must_use = "method returns the index and does not mutate the original value"]
    pub fn first_set(self) -> Option<usize> {
        // If bitmasks are efficient, using them is better
        if cfg!(target_feature = "sse") && N <= 64 {
            let tz = self.to_bitmask().trailing_zeros();
            return if tz == 64 { None } else { Some(tz as usize) };
        }

        // To find the first set index:
        // * create a vector 0..N
        // * replace unset mask elements in that vector with -1
        // * perform _unsigned_ reduce-min
        // * check if the result is -1 or an index

        let index = Simd::from_array(
            const {
                let mut index = [0; N];
                let mut i = 0;
                while i < N {
                    index[i] = i;
                    i += 1;
                }
                index
            },
        );

        // Safety: the input and output are integer vectors
        let index: Simd<T::Mask, N> = unsafe { core::intrinsics::simd::simd_cast(index) };

        // Safety: the mask and inputs are integer vectors
        let masked_index = unsafe {
            core::intrinsics::simd::simd_select(self.to_simd(), index, Self::splat(true).to_simd())
        };

        // Safety: the input and output are integer vectors
        let masked_index: Simd<<T::Mask as MaskElement>::Unsigned, N> =
            unsafe { core::intrinsics::simd::simd_cast(masked_index) };

        // Safety: the input is an integer vectors
        let min_index: <T::Mask as MaskElement>::Unsigned =
            unsafe { core::intrinsics::simd::simd_reduce_min(masked_index) };

        // Safety: the return value is the unsigned version of T
        let min_index: T::Mask = unsafe { core::mem::transmute_copy(&min_index) };

        if <T::Mask as MaskElement>::eq(min_index, <T::Mask as MaskElement>::TRUE) {
            None
        } else {
            Some(<T::Mask as MaskElement>::to_usize(min_index))
        }
    }
}

// vector/array conversion
impl<T, const N: usize> From<[bool; N]> for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(array: [bool; N]) -> Self {
        Self::from_array(array)
    }
}

impl<T, const N: usize> From<Mask<T, N>> for [bool; N]
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(vector: Mask<T, N>) -> Self {
        vector.to_array()
    }
}

impl<T, const N: usize> Default for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn default() -> Self {
        Self::splat(false)
    }
}

impl<T, const N: usize> PartialEq for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Use intrinsic to avoid additional bound on T
        // Safety: `self.0` is an integer vector
        unsafe { Self(core::intrinsics::simd::simd_eq(self.0, other.0)).all() }
    }
}

impl<T, const N: usize> PartialOrd for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // TODO use SIMD equality
        self.to_array().partial_cmp(&other.to_array())
    }
}

impl<T, const N: usize> fmt::Debug for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries((0..N).map(|i| self.test(i)))
            .finish()
    }
}

impl<T, const N: usize> core::ops::BitAnd for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        // Use intrinsic to avoid additional bound on T
        // Safety: `self` is an integer vector
        unsafe { Self(core::intrinsics::simd::simd_and(self.0, rhs.0)) }
    }
}

impl<T, const N: usize> core::ops::BitAnd<bool> for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: bool) -> Self {
        self & Self::splat(rhs)
    }
}

impl<T, const N: usize> core::ops::BitAnd<Mask<T, N>> for bool
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Mask<T, N>;
    #[inline]
    fn bitand(self, rhs: Mask<T, N>) -> Mask<T, N> {
        Mask::splat(self) & rhs
    }
}

impl<T, const N: usize> core::ops::BitOr for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        // Use intrinsic to avoid additional bound on T
        // Safety: `self` is an integer vector
        unsafe { Self(core::intrinsics::simd::simd_or(self.0, rhs.0)) }
    }
}

impl<T, const N: usize> core::ops::BitOr<bool> for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: bool) -> Self {
        self | Self::splat(rhs)
    }
}

impl<T, const N: usize> core::ops::BitOr<Mask<T, N>> for bool
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Mask<T, N>;
    #[inline]
    fn bitor(self, rhs: Mask<T, N>) -> Mask<T, N> {
        Mask::splat(self) | rhs
    }
}

impl<T, const N: usize> core::ops::BitXor for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        // Use intrinsic to avoid additional bound on T
        // Safety: `self` is an integer vector
        unsafe { Self(core::intrinsics::simd::simd_xor(self.0, rhs.0)) }
    }
}

impl<T, const N: usize> core::ops::BitXor<bool> for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: bool) -> Self::Output {
        self ^ Self::splat(rhs)
    }
}

impl<T, const N: usize> core::ops::BitXor<Mask<T, N>> for bool
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Mask<T, N>;
    #[inline]
    fn bitxor(self, rhs: Mask<T, N>) -> Self::Output {
        Mask::splat(self) ^ rhs
    }
}

impl<T, const N: usize> core::ops::Not for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Mask<T, N>;
    #[inline]
    fn not(self) -> Self::Output {
        Self::splat(true) ^ self
    }
}

impl<T, const N: usize> core::ops::BitAndAssign for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl<T, const N: usize> core::ops::BitAndAssign<bool> for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn bitand_assign(&mut self, rhs: bool) {
        *self &= Self::splat(rhs);
    }
}

impl<T, const N: usize> core::ops::BitOrAssign for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl<T, const N: usize> core::ops::BitOrAssign<bool> for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn bitor_assign(&mut self, rhs: bool) {
        *self |= Self::splat(rhs);
    }
}

impl<T, const N: usize> core::ops::BitXorAssign for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl<T, const N: usize> core::ops::BitXorAssign<bool> for Mask<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn bitxor_assign(&mut self, rhs: bool) {
        *self ^= Self::splat(rhs);
    }
}

macro_rules! impl_scalar_from_scalar {
    () => {};
    ($head:ty $(, $tail:ty)*) => {
        $(
            impl<const N: usize> From<Mask<$head, N>> for Mask<$tail, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn from(value: Mask<$head, N>) -> Self { value.cast() }
            }
            impl<const N: usize> From<Mask<$tail, N>> for Mask<$head, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn from(value: Mask<$tail, N>) -> Self { value.cast() }
            }
        )*
        impl_scalar_from_scalar! { $( $tail ),* }
    };
}

macro_rules! impl_scalar_from_ptr {
    ( $( $scalar:ty ),* $(,)? ) => {
        $(
            // From pointer to scalar
            impl<P, const N: usize> From<Mask<*const P, N>> for Mask<$scalar, N>
            where
                P: core::ptr::Pointee<Metadata = ()>,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn from(value: Mask<*const P, N>) -> Self { value.cast() }
            }

            impl<P, const N: usize> From<Mask<*mut P, N>> for Mask<$scalar, N>
            where
                P: core::ptr::Pointee<Metadata = ()>,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn from(value: Mask<*mut P, N>) -> Self { value.cast() }
            }

            // From scalar to pointer
            impl<P, const N: usize> From<Mask<$scalar, N>> for Mask<*const P, N>
            where
                P: core::ptr::Pointee<Metadata = ()>,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn from(value: Mask<$scalar, N>) -> Self { value.cast() }
            }

            impl<P, const N: usize> From<Mask<$scalar, N>> for Mask<*mut P, N>
            where
                P: core::ptr::Pointee<Metadata = ()>,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn from(value: Mask<$scalar, N>) -> Self { value.cast() }
            }
        )*
    };
}

macro_rules! impl_scalar_from {
    ( $( $scalar:ty ),* $(,)? ) => {
        impl_scalar_from_scalar! { $( $scalar ),* }
        impl_scalar_from_ptr! { $( $scalar ),* }
    };
}

impl_scalar_from! { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64 }

impl<T, U, const N: usize> From<Mask<*const T, N>> for Mask<*mut U, N>
where
    T: core::ptr::Pointee<Metadata = ()>,
    U: core::ptr::Pointee<Metadata = ()>,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(value: Mask<*const T, N>) -> Self {
        value.cast()
    }
}

impl<T, U, const N: usize> From<Mask<*mut T, N>> for Mask<*const U, N>
where
    T: core::ptr::Pointee<Metadata = ()>,
    U: core::ptr::Pointee<Metadata = ()>,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(value: Mask<*mut T, N>) -> Self {
        value.cast()
    }
}
