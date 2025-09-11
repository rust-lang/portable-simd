use crate::simd::{LaneCount, Simd, SupportedLaneCount};
use core::mem;

impl<const N: usize> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    /// Swizzle a vector of bytes according to the index vector.
    /// Indices within range select the appropriate byte.
    /// Indices "out of bounds" instead select 0.
    ///
    /// Note that the current implementation is selected during build-time
    /// of the standard library, so `cargo build -Zbuild-std` may be necessary
    /// to unlock better performance, especially for larger vectors.
    /// A planned compiler improvement will enable using `#[target_feature]` instead.
    #[inline]
    pub fn swizzle_dyn(self, idxs: Simd<u8, N>) -> Self {
        #![allow(unused_imports, unused_unsafe)]
        #[cfg(all(
            any(target_arch = "aarch64", target_arch = "arm64ec"),
            target_endian = "little"
        ))]
        use core::arch::aarch64::{uint8x8_t, vqtbl1q_u8, vtbl1_u8};
        #[cfg(all(
            target_arch = "arm",
            target_feature = "v7",
            target_feature = "neon",
            target_endian = "little"
        ))]
        use core::arch::arm::{uint8x8_t, vtbl1_u8};
        #[cfg(target_arch = "wasm32")]
        use core::arch::wasm32 as wasm;
        #[cfg(target_arch = "wasm64")]
        use core::arch::wasm64 as wasm;
        #[cfg(target_arch = "x86")]
        use core::arch::x86;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64 as x86;
        // SAFETY: Intrinsics covered by cfg
        unsafe {
            match N {
                #[cfg(all(
                    any(
                        target_arch = "aarch64",
                        target_arch = "arm64ec",
                        all(target_arch = "arm", target_feature = "v7")
                    ),
                    target_feature = "neon",
                    target_endian = "little"
                ))]
                8 => transize(vtbl1_u8, self, idxs),
                #[cfg(target_feature = "ssse3")]
                16 => transize(x86::_mm_shuffle_epi8, self, zeroing_idxs(idxs)),
                #[cfg(target_feature = "simd128")]
                16 => transize(wasm::i8x16_swizzle, self, idxs),
                #[cfg(all(
                    any(target_arch = "aarch64", target_arch = "arm64ec"),
                    target_feature = "neon",
                    target_endian = "little"
                ))]
                16 => transize(vqtbl1q_u8, self, idxs),
                #[cfg(all(
                    target_arch = "arm",
                    target_feature = "v7",
                    target_feature = "neon",
                    target_endian = "little"
                ))]
                16 => transize(armv7_neon_swizzle_u8x16, self, idxs),
                #[cfg(all(target_feature = "avx2", not(target_feature = "avx512vbmi")))]
                32 => transize(avx2_pshufb, self, idxs),
                #[cfg(all(target_feature = "avx512vl", target_feature = "avx512vbmi"))]
                32 => {
                    // Unlike vpshufb, vpermb doesn't zero out values in the result based on the index high bit
                    let swizzler = |bytes, idxs| {
                        let mask = x86::_mm256_cmp_epu8_mask::<{ x86::_MM_CMPINT_LT }>(
                            idxs,
                            Simd::<u8, 32>::splat(N as u8).into(),
                        );
                        x86::_mm256_maskz_permutexvar_epi8(mask, idxs, bytes)
                    };
                    transize(swizzler, self, idxs)
                }
                #[cfg(all(target_feature = "avx2", not(target_feature = "avx512vbmi")))]
                64 => transize(avx2_pshufb512, self, idxs),
                // Notable absence: avx512bw pshufb shuffle
                #[cfg(all(target_feature = "avx512vl", target_feature = "avx512vbmi"))]
                64 => {
                    // Unlike vpshufb, vpermb doesn't zero out values in the result based on the index high bit
                    let swizzler = |bytes, idxs| {
                        let mask = x86::_mm512_cmp_epu8_mask::<{ x86::_MM_CMPINT_LT }>(
                            idxs,
                            Simd::<u8, 64>::splat(N as u8).into(),
                        );
                        x86::_mm512_maskz_permutexvar_epi8(mask, idxs, bytes)
                    };
                    transize(swizzler, self, idxs)
                }
                _ => {
                    let mut array = [0; N];
                    for (i, k) in idxs.to_array().into_iter().enumerate() {
                        if (k as usize) < N {
                            array[i] = self[k as usize];
                        };
                    }
                    array.into()
                }
            }
        }
    }
}

/// armv7 neon supports swizzling `u8x16` by swizzling two u8x8 blocks
/// with a u8x8x2 lookup table.
///
/// # Safety
/// This requires armv7 neon to work
#[cfg(all(
    target_arch = "arm",
    target_feature = "v7",
    target_feature = "neon",
    target_endian = "little"
))]
unsafe fn armv7_neon_swizzle_u8x16(bytes: Simd<u8, 16>, idxs: Simd<u8, 16>) -> Simd<u8, 16> {
    use core::arch::arm::{uint8x8x2_t, vcombine_u8, vget_high_u8, vget_low_u8, vtbl2_u8};
    // SAFETY: Caller promised arm neon support
    unsafe {
        let bytes = uint8x8x2_t(vget_low_u8(bytes.into()), vget_high_u8(bytes.into()));
        let lo = vtbl2_u8(bytes, vget_low_u8(idxs.into()));
        let hi = vtbl2_u8(bytes, vget_high_u8(idxs.into()));
        vcombine_u8(lo, hi).into()
    }
}

/// "vpshufb like it was meant to be" on AVX2
///
/// # Safety
/// This requires AVX2 to work
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[allow(unused)]
#[inline]
#[allow(clippy::let_and_return)]
unsafe fn avx2_pshufb(bytes: Simd<u8, 32>, idxs: Simd<u8, 32>) -> Simd<u8, 32> {
    use crate::simd::cmp::SimdPartialOrd;
    #[cfg(target_arch = "x86")]
    use core::arch::x86;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64 as x86;
    use x86::_mm256_permute2x128_si256 as avx2_cross_shuffle;
    use x86::_mm256_shuffle_epi8 as avx2_half_pshufb;
    let mid = Simd::splat(16u8);
    let high = mid + mid;
    // SAFETY: Caller promised AVX2
    unsafe {
        // This is ordering sensitive, and LLVM will order these how you put them.
        // Most AVX2 impls use ~5 "ports", and only 1 or 2 are capable of permutes.
        // But the "compose" step will lower to ops that can also use at least 1 other port.
        // So this tries to break up permutes so composition flows through "open" ports.
        // Comparative benches should be done on multiple AVX2 CPUs before reordering this

        let hihi = avx2_cross_shuffle::<0x11>(bytes.into(), bytes.into());
        let hi_shuf = Simd::from(avx2_half_pshufb(
            hihi,        // duplicate the vector's top half
            idxs.into(), // so that using only 4 bits of an index still picks bytes 16-31
        ));
        // A zero-fill during the compose step gives the "all-Neon-like" OOB-is-0 semantics
        let compose = idxs.simd_lt(high).select(hi_shuf, Simd::splat(0));
        let lolo = avx2_cross_shuffle::<0x00>(bytes.into(), bytes.into());
        let lo_shuf = Simd::from(avx2_half_pshufb(lolo, idxs.into()));
        // Repeat, then pick indices < 16, overwriting indices 0-15 from previous compose step
        let compose = idxs.simd_lt(mid).select(lo_shuf, compose);
        compose
    }
}

/// The above function but for 64 bytes
///
/// # Safety
/// This requires AVX2 to work
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[allow(unused)]
#[inline]
#[allow(clippy::let_and_return)]
unsafe fn avx2_pshufb512(bytes: Simd<u8, 64>, idxs: Simd<u8, 64>) -> Simd<u8, 64> {
    use crate::simd::cmp::SimdPartialOrd;
    #[cfg(target_arch = "x86")]
    use core::arch::x86;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64 as x86;
    use x86::_mm256_blendv_epi8 as avx2_blend;
    use x86::_mm256_permute2x128_si256 as avx2_cross_shuffle;
    use x86::_mm256_shuffle_epi8 as avx2_half_pshufb;
    let high = Simd::splat(64u8);
    // SAFETY: Caller promised AVX2
    unsafe {
        let half_swizzler = |bytes0: Simd<u8, 32>, bytes1: Simd<u8, 32>, idxs: Simd<u8, 32>| {
            let mask0 = idxs << 2;
            let mask1 = idxs << 3;

            let lolo0 = avx2_cross_shuffle::<0x00>(bytes0.into(), bytes0.into());
            let hihi0 = avx2_cross_shuffle::<0x11>(bytes0.into(), bytes0.into());
            let lolo0 = avx2_half_pshufb(lolo0, idxs.into());
            let hihi0 = avx2_half_pshufb(hihi0, idxs.into());
            let x = avx2_blend(lolo0, hihi0, mask1.into());

            let lolo1 = avx2_cross_shuffle::<0x00>(bytes1.into(), bytes1.into());
            let hihi1 = avx2_cross_shuffle::<0x11>(bytes1.into(), bytes1.into());
            let lolo1 = avx2_half_pshufb(lolo1, idxs.into());
            let hihi1 = avx2_half_pshufb(hihi1, idxs.into());
            let y = avx2_blend(lolo1, hihi1, mask1.into());

            avx2_blend(x, y, mask0.into())
        };

        let bytes0 = bytes.extract::<0, 32>();
        let bytes1 = bytes.extract::<32, 32>();
        let idxs0 = idxs.extract::<0, 32>();
        let idxs1 = idxs.extract::<32, 32>();

        let z0 = half_swizzler(bytes0, bytes1, idxs0);
        let z1 = half_swizzler(bytes0, bytes1, idxs1);

        // SAFETY: Concatenation of two 32-element vectors to one 64-element vector
        let z = mem::transmute::<[Simd<u8, 32>; 2], Simd<u8, 64>>([z0.into(), z1.into()]);

        idxs.simd_lt(high).select(z, Simd::splat(0u8))
    }
}

/// This sets up a call to an architecture-specific function, and in doing so
/// it persuades rustc that everything is the correct size. Which it is.
/// This would not be needed if one could convince Rust that, by matching on N,
/// N is that value, and thus it would be valid to substitute e.g. 16.
///
/// # Safety
/// The correctness of this function hinges on the sizes agreeing in actuality.
#[allow(dead_code)]
#[inline(always)]
unsafe fn transize<T, const N: usize>(
    f: unsafe fn(T, T) -> T,
    a: Simd<u8, N>,
    b: Simd<u8, N>,
) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    // SAFETY: Same obligation to use this function as to use mem::transmute_copy.
    unsafe { mem::transmute_copy(&f(mem::transmute_copy(&a), mem::transmute_copy(&b))) }
}

/// Make indices that yield 0 for x86
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[allow(unused)]
#[inline(always)]
fn zeroing_idxs<const N: usize>(idxs: Simd<u8, N>) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    use crate::simd::cmp::SimdPartialOrd;
    idxs.simd_lt(Simd::splat(N as u8))
        .select(idxs, Simd::splat(u8::MAX))
}
