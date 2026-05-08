use super::sealed::Sealed;
use crate::simd::{Mask, Simd, SimdElement, cmp::SimdPartialEq, num::SimdUint};

/// Operations on SIMD vectors of mutable pointers.
pub trait SimdMutPtr: SimdElement + Copy + Sealed {
    /// The pointee type referenced by these pointers.
    type Pointee: core::ptr::Pointee<Metadata = ()>;

    /// Returns `true` for each element that is null.
    fn is_null<const N: usize>(self: Simd<Self, N>) -> Mask<isize, N>;

    /// Casts to a pointer of another type.
    ///
    /// Equivalent to calling [`pointer::cast`] on each element.
    fn cast<U, const N: usize>(self: Simd<Self, N>) -> Simd<*mut U, N>
    where
        U: core::ptr::Pointee<Metadata = ()>;

    /// Changes constness without changing the type.
    ///
    /// Equivalent to calling [`pointer::cast_const`] on each element.
    fn cast_const<const N: usize>(self: Simd<Self, N>) -> Simd<*const Self::Pointee, N>;

    /// Gets the "address" portion of the pointer.
    ///
    /// Equivalent to calling [`pointer::addr`] on each element.
    fn addr<const N: usize>(self: Simd<Self, N>) -> Simd<usize, N>;

    /// Creates a new pointer with the given address.
    ///
    /// Equivalent to calling [`pointer::with_addr`] on each element.
    fn with_addr<const N: usize>(self: Simd<Self, N>, addr: Simd<usize, N>) -> Simd<Self, N>;

    /// Exposes the "provenance" part of the pointer for future use in
    /// [`super::with_exposed_provenance_mut`] and returns the "address" portion.
    ///
    /// Equivalent to calling [`pointer::expose_provenance`] on each element.
    ///
    /// See [`super::with_exposed_provenance_mut`] for the matching constructor.
    fn expose_provenance<const N: usize>(self: Simd<Self, N>) -> Simd<usize, N>;

    /// Calculates the offset from a pointer using wrapping arithmetic.
    ///
    /// Equivalent to calling [`pointer::wrapping_offset`] on each element.
    fn wrapping_offset<const N: usize>(
        self: Simd<Self, N>,
        offset: Simd<isize, N>,
    ) -> Simd<Self, N>;

    /// Calculates the offset from a pointer using wrapping arithmetic.
    ///
    /// Equivalent to calling [`pointer::wrapping_add`] on each element.
    fn wrapping_add<const N: usize>(self: Simd<Self, N>, count: Simd<usize, N>) -> Simd<Self, N>;

    /// Calculates the offset from a pointer using wrapping arithmetic.
    ///
    /// Equivalent to calling [`pointer::wrapping_sub`] on each element.
    fn wrapping_sub<const N: usize>(self: Simd<Self, N>, count: Simd<usize, N>) -> Simd<Self, N>;
}

impl<T> SimdMutPtr for *mut T
where
    T: core::ptr::Pointee<Metadata = ()>,
{
    type Pointee = T;

    #[inline]
    fn is_null<const N: usize>(self: Simd<Self, N>) -> Mask<isize, N> {
        Simd::splat(core::ptr::null_mut::<T>()).simd_eq(self)
    }

    #[inline]
    fn cast<U, const N: usize>(self: Simd<Self, N>) -> Simd<*mut U, N>
    where
        U: core::ptr::Pointee<Metadata = ()>,
    {
        // SimdElement currently requires zero-sized metadata, so this should never fail.
        // If this ever changes, `simd_cast_ptr` should produce a post-mono error.
        use core::ptr::Pointee;
        assert_eq!(size_of::<<T as Pointee>::Metadata>(), 0);
        assert_eq!(size_of::<<U as Pointee>::Metadata>(), 0);

        // Safety: pointers can be cast
        unsafe { core::intrinsics::simd::simd_cast_ptr(self) }
    }

    #[inline]
    fn cast_const<const N: usize>(self: Simd<Self, N>) -> Simd<*const Self::Pointee, N> {
        // Safety: pointers can be cast
        unsafe { core::intrinsics::simd::simd_cast_ptr(self) }
    }

    #[inline]
    fn addr<const N: usize>(self: Simd<Self, N>) -> Simd<usize, N> {
        // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
        // SAFETY: Pointer-to-integer transmutes are valid (if you are okay with losing the
        // provenance).
        unsafe { core::mem::transmute_copy(&self) }
    }

    #[inline]
    fn with_addr<const N: usize>(self: Simd<Self, N>, addr: Simd<usize, N>) -> Simd<Self, N> {
        // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
        //
        // In the mean-time, this operation is defined to be "as if" it was
        // a wrapping_offset, so we can emulate it as such. This should properly
        // restore pointer provenance even under today's compiler.
        self.cast::<u8, N>()
            .wrapping_offset(addr.cast::<isize, N>() - self.addr().cast::<isize, N>())
            .cast::<T, N>()
    }

    #[inline]
    fn expose_provenance<const N: usize>(self: Simd<Self, N>) -> Simd<usize, N> {
        // Safety: `self` is a pointer vector
        unsafe { core::intrinsics::simd::simd_expose_provenance(self) }
    }

    #[inline]
    fn wrapping_offset<const N: usize>(
        self: Simd<Self, N>,
        count: Simd<isize, N>,
    ) -> Simd<Self, N> {
        // Safety: simd_arith_offset takes a vector of pointers and a vector of offsets
        unsafe { core::intrinsics::simd::simd_arith_offset(self, count) }
    }

    #[inline]
    fn wrapping_add<const N: usize>(self: Simd<Self, N>, count: Simd<usize, N>) -> Simd<Self, N> {
        self.wrapping_offset(count.cast())
    }

    #[inline]
    fn wrapping_sub<const N: usize>(self: Simd<Self, N>, count: Simd<usize, N>) -> Simd<Self, N> {
        self.wrapping_offset(-count.cast::<isize, N>())
    }
}
