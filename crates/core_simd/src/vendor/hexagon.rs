#![allow(unused)]
use crate::simd::*;

// Hexagon HVX target configuration
// Since core::arch::hexagon doesn't exist yet in Rust's standard library,
// we define placeholder types that match the expected HVX vector types
// This provides a foundation for future intrinsic support

#[cfg(target_arch = "hexagon")]
mod hvx {
    use super::*;

    // HVX vector register is 1024-bit (128 bytes)
    // Can be viewed as different element types:
    // - 64 x 16-bit halfwords
    // - 32 x 32-bit words
    // - 16 x 64-bit doublewords
    // Note: 128 x 8-bit bytes (u8x128/i8x128) disabled for now due to test suite impact

    // For now, we use repr(transparent) structs as placeholders
    // These would be replaced with actual intrinsic types when available

    #[repr(transparent)]
    pub struct HvxVectorU16([u16; 64]);

    #[repr(transparent)]
    pub struct HvxVectorI16([i16; 64]);

    #[repr(transparent)]
    pub struct HvxVectorU32([u32; 32]);

    #[repr(transparent)]
    pub struct HvxVectorI32([i32; 32]);

    #[repr(transparent)]
    pub struct HvxVectorU64([u64; 16]);

    #[repr(transparent)]
    pub struct HvxVectorI64([i64; 16]);

    // HVX also supports 512-bit vectors (half-vector mode)
    #[repr(transparent)]
    pub struct HvxVectorHalfU8([u8; 64]);

    #[repr(transparent)]
    pub struct HvxVectorHalfI8([i8; 64]);

    #[repr(transparent)]
    pub struct HvxVectorHalfU16([u16; 32]);

    #[repr(transparent)]
    pub struct HvxVectorHalfI16([i16; 32]);

    #[repr(transparent)]
    pub struct HvxVectorHalfU32([u32; 16]);

    #[repr(transparent)]
    pub struct HvxVectorHalfI32([i32; 16]);

    #[repr(transparent)]
    pub struct HvxVectorHalfU64([u64; 8]);

    #[repr(transparent)]
    pub struct HvxVectorHalfI64([i64; 8]);

    // Map portable SIMD types to HVX full vector types (1024-bit)
    // Note: u8x128/i8x128 mappings disabled for now due to test suite impact
    from_transmute! { unsafe u16x64 => HvxVectorU16 }
    from_transmute! { unsafe i16x64 => HvxVectorI16 }
    from_transmute! { unsafe u32x32 => HvxVectorU32 }
    from_transmute! { unsafe i32x32 => HvxVectorI32 }
    from_transmute! { unsafe u64x16 => HvxVectorU64 }
    from_transmute! { unsafe i64x16 => HvxVectorI64 }

    // Map portable SIMD types to HVX half vector types (512-bit)
    from_transmute! { unsafe u8x64 => HvxVectorHalfU8 }
    from_transmute! { unsafe i8x64 => HvxVectorHalfI8 }
    from_transmute! { unsafe u16x32 => HvxVectorHalfU16 }
    from_transmute! { unsafe i16x32 => HvxVectorHalfI16 }
    from_transmute! { unsafe u32x16 => HvxVectorHalfU32 }
    from_transmute! { unsafe i32x16 => HvxVectorHalfI32 }
    from_transmute! { unsafe u64x8 => HvxVectorHalfU64 }
    from_transmute! { unsafe i64x8 => HvxVectorHalfI64 }
}
