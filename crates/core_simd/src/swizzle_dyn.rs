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
        #![allow(unused_imports, unused_unsafe, unreachable_patterns)]
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
                32 => transize(avx2_pshufb, self, idxs),
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
                #[cfg(any(
                    all(
                        any(
                            target_arch = "aarch64",
                            target_arch = "arm64ec",
                            all(target_arch = "arm", target_feature = "v7")
                        ),
                        target_feature = "neon",
                        target_endian = "little"
                    ),
                    target_feature = "ssse3",
                    target_feature = "simd128"
                ))]
                _ => dispatch_compat(self, idxs),
                _ => swizzle_dyn_scalar(self, idxs),
            }
        }
    }
}

#[inline(always)]
fn swizzle_dyn_scalar<const N: usize>(bytes: Simd<u8, N>, idxs: Simd<u8, N>) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let mut array = [0; N];
    for (i, k) in idxs.to_array().into_iter().enumerate() {
        if (k as usize) < N {
            array[i] = bytes[k as usize];
        };
    }
    array.into()
}

/// Dispatch to swizzle_dyn_compat and swizzle_dyn_zext according to N.
/// Should only be called if there exists some power-of-two size for which
/// the target architecture has a vectorized swizzle_dyn (e.g. pshufb, vqtbl).
#[inline(always)]
fn dispatch_compat<const N: usize>(bytes: Simd<u8, N>, idxs: Simd<u8, N>) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #![allow(
        dead_code,
        unused_unsafe,
        unreachable_patterns,
        non_contiguous_range_endpoints
    )]

    // SAFETY: only unsafe usage is transize, see comment on transize
    unsafe {
        match N {
            5..16 => swizzle_dyn_zext::<N, 16>(bytes, idxs),
            // only arm actually has 8-byte swizzle_dyn
            #[cfg(all(
                any(
                    target_arch = "aarch64",
                    target_arch = "arm64ec",
                    all(target_arch = "arm", target_feature = "v7")
                ),
                target_feature = "neon",
                target_endian = "little"
            ))]
            16 => transize(swizzle_dyn_compat::<16, 8>, bytes, idxs),
            17..32 => swizzle_dyn_zext::<N, 32>(bytes, idxs),
            32 => transize(swizzle_dyn_compat::<32, 16>, bytes, idxs),
            33..64 => swizzle_dyn_zext::<N, 64>(bytes, idxs),
            64 => transize(swizzle_dyn_compat::<64, 32>, bytes, idxs),
            _ => swizzle_dyn_scalar(bytes, idxs),
        }
    }
}

/// Implement swizzle_dyn for N by temporarily zero extending to N_EXT.
#[inline(always)]
#[allow(unused)]
fn swizzle_dyn_zext<const N: usize, const N_EXT: usize>(
    bytes: Simd<u8, N>,
    idxs: Simd<u8, N>,
) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
    LaneCount<N_EXT>: SupportedLaneCount,
{
    assert!(N_EXT.is_power_of_two(), "N_EXT should be power of two!");
    assert!(N < N_EXT, "N_EXT should be larger than N");
    Simd::swizzle_dyn(bytes.resize::<N_EXT>(0), idxs.resize::<N_EXT>(0)).resize::<N>(0)
}

/// "Downgrades" a swizzle_dyn op on N lanes to 4 swizzle_dyn ops on N/2 lanes.
///
/// This only makes sense if swizzle_dyn actually has a vectorized implementation for a lower size (N/2, N/4, N/8, etc).
/// e.g. on x86, swizzle_dyn_compat for N=64 can be efficient if we have at least ssse3 for pshufb
///
/// If there is no vectorized implementation for a lower size,
/// this runs in N*logN time and will be slower than the scalar implementation.
#[inline(always)]
#[allow(unused)]
fn swizzle_dyn_compat<const N: usize, const HALF_N: usize>(
    bytes: Simd<u8, N>,
    idxs: Simd<u8, N>,
) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
    LaneCount<HALF_N>: SupportedLaneCount,
{
    use crate::simd::cmp::SimdPartialOrd;
    assert!(N.is_power_of_two(), "doesn't work for non-power-of-two N");
    assert!(N < u8::MAX as usize, "doesn't work for N >= 256");
    assert_eq!(N / 2, HALF_N, "HALF_N must equal N divided by two");

    let mid = Simd::splat(HALF_N as u8);

    // unset the "mid" bit from the indices, e.g. 8..15 -> 0..7, 16..31 -> 8..15,
    // ensuring that a half-swizzle on the higher half of `bytes` will select the correct indices
    // since N is a power of two, any zeroing indices will remain zeroing
    let idxs_trunc = idxs & !mid;

    let idx_lo = Simd::<u8, HALF_N>::from_slice(&idxs_trunc[..HALF_N]);
    let idx_hi = Simd::<u8, HALF_N>::from_slice(&idxs_trunc[HALF_N..]);

    let bytes_lo = Simd::<u8, HALF_N>::from_slice(&bytes[..HALF_N]);
    let bytes_hi = Simd::<u8, HALF_N>::from_slice(&bytes[HALF_N..]);

    macro_rules! half_swizzle {
        ($bytes:ident) => {{
            let lo = Simd::swizzle_dyn($bytes, idx_lo);
            let hi = Simd::swizzle_dyn($bytes, idx_hi);

            let mut res = [0; N];
            res[..HALF_N].copy_from_slice(&lo[..]);
            res[HALF_N..].copy_from_slice(&hi[..]);
            Simd::from_array(res)
        }};
    }

    let result_lo = half_swizzle!(bytes_lo);
    let result_hi = half_swizzle!(bytes_hi);
    idxs.simd_lt(mid).select(result_lo, result_hi)
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
