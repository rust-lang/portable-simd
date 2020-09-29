//! This module contains the LLVM intrinsics bindings that provide the functionality for this
//! crate.
//!
//! The LLVM assembly language is documented here: https://llvm.org/docs/LangRef.html

/// These intrinsics aren't linked directly from LLVM and are mostly undocumented, however they are
/// simply lowered to the matching LLVM instructions by the compiler.  The associated instruction
/// is documented alongside each intrinsic.
extern "platform-intrinsic" {
    /// add/fadd
    pub(crate) fn simd_add<T>(x: T, y: T) -> T;

    /// sub/fsub
    pub(crate) fn simd_sub<T>(x: T, y: T) -> T;

    /// mul/fmul
    pub(crate) fn simd_mul<T>(x: T, y: T) -> T;

    /// udiv/sdiv/fdiv
    pub(crate) fn simd_div<T>(x: T, y: T) -> T;

    /// urem/srem/frem
    pub(crate) fn simd_rem<T>(x: T, y: T) -> T;

    /// shl
    pub(crate) fn simd_shl<T>(x: T, y: T) -> T;

    /// lshr/ashr
    pub(crate) fn simd_shr<T>(x: T, y: T) -> T;

    /// and
    pub(crate) fn simd_and<T>(x: T, y: T) -> T;

    /// or
    pub(crate) fn simd_or<T>(x: T, y: T) -> T;

    /// xor
    pub(crate) fn simd_xor<T>(x: T, y: T) -> T;
}
