/// The error type returned when converting an integer to a mask fails.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromMaskError(());

impl core::fmt::Display for TryFromMaskError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "mask must have all bits set or unset")
    }
}

macro_rules! define_mask {
    { $(#[$attr:meta])* struct $name:ident($type:ty); } => {
        $(#[$attr])*
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name(pub(crate) $type);

        impl $name {
            /// Construct a mask from the given value.
            pub const fn new(value: bool) -> Self {
                if value {
                    Self(!0)
                } else {
                    Self(0)
                }
            }

            /// Test if the mask is set.
            pub const fn test(&self) -> bool {
                self.0 != 0
            }
        }

        impl core::convert::From<bool> for $name {
            fn from(value: bool) -> Self {
                Self::new(value)
            }
        }

        impl core::convert::From<$name> for bool {
            fn from(mask: $name) -> Self {
                mask.test()
            }
        }

        impl core::convert::TryFrom<$type> for $name {
            type Error = TryFromMaskError;
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                if value == 0 || !value == 0 {
                    Ok(Self(value))
                } else {
                    Err(TryFromMaskError(()))
                }
            }
        }

        impl core::convert::From<$name> for $type {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.test().fmt(f)
            }
        }

        impl core::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl core::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl core::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self {
                Self(!self.0)
            }
        }
    }
}

define_mask! {
    /// 8-bit mask
    struct mask8(i8);
}

define_mask! {
    /// 16-bit mask
    struct mask16(i16);
}

define_mask! {
    /// 32-bit mask
    struct mask32(i32);
}

define_mask! {
    /// 64-bit mask
    struct mask64(i64);
}

define_mask! {
    /// 128-bit mask
    struct mask128(i128);
}

define_mask! {
    /// `isize`-wide mask
    struct masksize(isize);
}
