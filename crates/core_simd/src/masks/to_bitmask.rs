use super::{mask_impl, Mask, MaskElement};
use crate::simd::{LaneCount, SupportedLaneCount};

mod sealed {
    pub trait Sealed {}
}
pub use sealed::Sealed;

impl<T, const LANES: usize> Sealed for Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
}

/// Converts masks to and from integer bitmasks.
///
/// Each bit of the bitmask corresponds to a mask lane, starting with the LSB.
///
/// # Safety
/// This trait is `unsafe` and sealed, since the `BitMask` type must match the number of lanes in
/// the mask.
pub unsafe trait ToBitMask: Sealed {
    /// The integer bitmask type.
    type BitMask;

    /// Converts a mask to a bitmask.
    fn to_bitmask(self) -> Self::BitMask;

    /// Converts a bitmask to a mask.
    fn from_bitmask(bitmask: Self::BitMask) -> Self;
}

/// Converts masks to and from byte array bitmasks.
///
/// Each bit of the bitmask corresponds to a mask lane, starting with the LSB of the first byte.
///
/// # Safety
/// This trait is `unsafe` and sealed, since the `BYTES` value must match the number of lanes in
/// the mask.
pub unsafe trait ToBitMaskArray: Sealed {
    /// The length of the bitmask array.
    const BYTES: usize;

    /// Converts a mask to a bitmask.
    fn to_bitmask_array(self) -> [u8; Self::BYTES];

    /// Converts a bitmask to a mask.
    fn from_bitmask_array(bitmask: [u8; Self::BYTES]) -> Self;
}

macro_rules! impl_integer_intrinsic {
    { $(unsafe impl ToBitMask<BitMask=$int:ty> for Mask<_, $lanes:literal>)* } => {
        $(
        unsafe impl<T: MaskElement> ToBitMask for Mask<T, $lanes> {
            type BitMask = $int;

            fn to_bitmask(self) -> $int {
                self.0.to_bitmask_integer()
            }

            fn from_bitmask(bitmask: $int) -> Self {
                Self(mask_impl::Mask::from_bitmask_integer(bitmask))
            }
        }
        )*
    }
}

impl_integer_intrinsic! {
    unsafe impl ToBitMask<BitMask=u8> for Mask<_, 8>
    unsafe impl ToBitMask<BitMask=u16> for Mask<_, 16>
    unsafe impl ToBitMask<BitMask=u32> for Mask<_, 32>
    unsafe impl ToBitMask<BitMask=u64> for Mask<_, 64>
}

/// Returns the minimum numnber of bytes in a bitmask with `lanes` lanes.
pub const fn bitmask_len(lanes: usize) -> usize {
    (lanes + 7) / 8
}

macro_rules! impl_array_bitmask {
    { $(impl ToBitMaskArray<[u8; _]> for Mask<_, $lanes:literal>)* } => {
        $(
        unsafe impl<T: MaskElement> ToBitMaskArray for Mask<T, $lanes>
        {
             const BYTES: usize = bitmask_len($lanes);

             fn to_bitmask_array(self) -> [u8; Self::BYTES] {
                 self.0.to_bitmask_array()
             }

             fn from_bitmask_array(bitmask: [u8; Self::BYTES]) -> Self {
                 Mask(mask_impl::Mask::from_bitmask_array(bitmask))
             }
        }
        )*
    }
}

// FIXME this should be specified generically, but it doesn't seem to work with rustc, yet
impl_array_bitmask! {
    impl ToBitMaskArray<[u8; _]> for Mask<_, 1>
    impl ToBitMaskArray<[u8; _]> for Mask<_, 2>
    impl ToBitMaskArray<[u8; _]> for Mask<_, 4>
    impl ToBitMaskArray<[u8; _]> for Mask<_, 8>
    impl ToBitMaskArray<[u8; _]> for Mask<_, 16>
    impl ToBitMaskArray<[u8; _]> for Mask<_, 32>
    impl ToBitMaskArray<[u8; _]> for Mask<_, 64>
}
