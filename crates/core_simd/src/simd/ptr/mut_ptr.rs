use super::sealed::Sealed;
use crate::simd::{Mask, Simd, cmp::SimdPartialEq, num::SimdUint};

/// Operations on SIMD vectors of mutable pointers.
pub trait SimdMutPtr<T, const N: usize>: Copy + Sealed {
    /// Returns `true` for each element that is null.
    fn is_null(self) -> Mask<isize, N>;

    /// Casts to a pointer of another type.
    ///
    /// Equivalent to calling [`pointer::cast`] on each element.
    fn cast<U>(self) -> Simd<*mut U, N>;

    /// Changes constness without changing the type.
    ///
    /// Equivalent to calling [`pointer::cast_const`] on each element.
    fn cast_const(self) -> Simd<*const T, N>;

    /// Gets the "address" portion of the pointer.
    ///
    /// This method discards pointer semantic metadata, so the result cannot be
    /// directly cast into a valid pointer.
    ///
    /// Equivalent to calling [`pointer::addr`] on each element.
    fn addr(self) -> Simd<usize, N>;

    /// Converts an address to a pointer without giving it any provenance.
    ///
    /// Without provenance, this pointer is not associated with any actual allocation. Such a
    /// no-provenance pointer may be used for zero-sized memory accesses (if suitably aligned), but
    /// non-zero-sized memory accesses with a no-provenance pointer are UB. No-provenance pointers
    /// are little more than a usize address in disguise.
    ///
    /// This is different from [`Self::with_exposed_provenance`], which creates a pointer that picks up a
    /// previously exposed provenance.
    ///
    /// Equivalent to calling [`core::ptr::without_provenance`] on each element.
    fn without_provenance(addr: Simd<usize, N>) -> Self;

    /// Creates a new pointer with the given address.
    ///
    /// This performs the same operation as a cast, but copies the *address-space* and
    /// *provenance* of `self` to the new pointer.
    ///
    /// Equivalent to calling [`pointer::with_addr`] on each element.
    fn with_addr(self, addr: Simd<usize, N>) -> Self;

    /// Exposes the "provenance" part of the pointer for future use in
    /// [`Self::with_exposed_provenance`] and returns the "address" portion.
    fn expose_provenance(self) -> Simd<usize, N>;

    /// Converts an address back to a pointer, picking up a previously "exposed" provenance.
    ///
    /// Equivalent to calling [`core::ptr::with_exposed_provenance_mut`] on each element.
    fn with_exposed_provenance(addr: Simd<usize, N>) -> Self;

    /// Calculates the offset from a pointer using wrapping arithmetic.
    ///
    /// Equivalent to calling [`pointer::wrapping_offset`] on each element.
    fn wrapping_offset(self, offset: Simd<isize, N>) -> Self;

    /// Calculates the offset from a pointer using wrapping arithmetic.
    ///
    /// Equivalent to calling [`pointer::wrapping_add`] on each element.
    fn wrapping_add(self, count: Simd<usize, N>) -> Self;

    /// Calculates the offset from a pointer using wrapping arithmetic.
    ///
    /// Equivalent to calling [`pointer::wrapping_sub`] on each element.
    fn wrapping_sub(self, count: Simd<usize, N>) -> Self;
}

impl<T, const N: usize> Sealed for Simd<*mut T, N> {}

impl<T, const N: usize> SimdMutPtr<T, N> for Simd<*mut T, N> {
    #[inline]
    fn is_null(self) -> Mask<isize, N> {
        Simd::splat(core::ptr::null_mut()).simd_eq(self)
    }

    #[inline]
    fn cast<U>(self) -> Simd<*mut U, N> {
        // SimdElement currently requires zero-sized metadata, so this should never fail.
        // If this ever changes, `simd_cast_ptr` should produce a post-mono error.
        use core::ptr::Pointee;
        assert_eq!(size_of::<<T as Pointee>::Metadata>(), 0);
        assert_eq!(size_of::<<U as Pointee>::Metadata>(), 0);

        // Safety: pointers can be cast
        unsafe { core::intrinsics::simd::simd_cast_ptr(self) }
    }

    #[inline]
    fn cast_const(self) -> Simd<*const T, N> {
        // Safety: pointers can be cast
        unsafe { core::intrinsics::simd::simd_cast_ptr(self) }
    }

    #[inline]
    fn addr(self) -> Simd<usize, N> {
        // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
        // SAFETY: Pointer-to-integer transmutes are valid (if you are okay with losing the
        // provenance).
        unsafe { core::mem::transmute_copy(&self) }
    }

    #[inline]
    fn without_provenance(addr: Simd<usize, N>) -> Self {
        // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
        // SAFETY: Integer-to-pointer transmutes are valid (if you are okay with not getting any
        // provenance).
        unsafe { core::mem::transmute_copy(&addr) }
    }

    #[inline]
    fn with_addr(self, addr: Simd<usize, N>) -> Self {
        // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
        //
        // In the mean-time, this operation is defined to be "as if" it was
        // a wrapping_offset, so we can emulate it as such. This should properly
        // restore pointer provenance even under today's compiler.
        self.cast::<u8>()
            .wrapping_offset(addr.cast::<isize>() - self.addr().cast::<isize>())
            .cast()
    }

    #[inline]
    fn expose_provenance(self) -> Simd<usize, N> {
        // Safety: `self` is a pointer vector
        unsafe { core::intrinsics::simd::simd_expose_provenance(self) }
    }

    #[inline]
    fn with_exposed_provenance(addr: Simd<usize, N>) -> Self {
        // Safety: `self` is a pointer vector
        unsafe { core::intrinsics::simd::simd_with_exposed_provenance(addr) }
    }

    #[inline]
    fn wrapping_offset(self, count: Simd<isize, N>) -> Self {
        // Safety: simd_arith_offset takes a vector of pointers and a vector of offsets
        unsafe { core::intrinsics::simd::simd_arith_offset(self, count) }
    }

    #[inline]
    fn wrapping_add(self, count: Simd<usize, N>) -> Self {
        self.wrapping_offset(count.cast())
    }

    #[inline]
    fn wrapping_sub(self, count: Simd<usize, N>) -> Self {
        self.wrapping_offset(-count.cast::<isize>())
    }
}
