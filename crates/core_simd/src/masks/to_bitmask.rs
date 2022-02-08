use super::{mask_impl, Mask, MaskElement};

/// Converts masks to and from integer bitmasks.
///
/// Each bit of the bitmask corresponds to a mask lane, starting with the LSB.
pub trait ToBitMask {
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
pub trait ToBitMaskArray {
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
        impl<T: MaskElement> ToBitMask for Mask<T, $lanes> {
            type BitMask = $int;

            fn to_bitmask(self) -> $int {
                unsafe { self.0.to_bitmask_intrinsic() }
            }

            fn from_bitmask(bitmask: $int) -> Self {
                unsafe { Self(mask_impl::Mask::from_bitmask_intrinsic(bitmask)) }
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
#[cfg(feature = "generic_const_exprs")]
pub const fn bitmask_len(lanes: usize) -> usize {
    (lanes + 7) / 8
}

#[cfg(feature = "generic_const_exprs")]
macro_rules! impl_array_bitmask {
    { $(impl ToBitMask<[u8; _]> for Mask<_, $lanes:literal>)* } => {
        $(
        impl<T: MaskElement> ToBitMaskArray for Mask<T, $lanes>
        {
             const BYTES: usize = bitmask_len($lanes);

             fn to_bitmask_array(self) -> [u8; Self::BYTES] {
                 self.0.to_bitmask()
             }

             fn from_bitmask_array(bitmask: [u8; Self::BYTES]) -> Self {
                 Mask(mask_impl::Mask::from_bitmask(bitmask))
             }
        }
        )*
    }
}

// FIXME this should be specified generically, but it doesn't seem to work with rustc, yet
#[cfg(feature = "generic_const_exprs")]
impl_array_bitmask! {
    impl ToBitMask<[u8; _]> for Mask<_, 1>
    impl ToBitMask<[u8; _]> for Mask<_, 2>
    impl ToBitMask<[u8; _]> for Mask<_, 4>
    impl ToBitMask<[u8; _]> for Mask<_, 8>
    impl ToBitMask<[u8; _]> for Mask<_, 16>
    impl ToBitMask<[u8; _]> for Mask<_, 32>
    impl ToBitMask<[u8; _]> for Mask<_, 64>
}
