/// Checks if the right-hand side argument of a left- or right-shift would cause overflow.
fn invalid_shift_rhs<T>(rhs: T) -> bool
where
    T: Default + PartialOrd + core::convert::TryFrom<usize>,
    <T as core::convert::TryFrom<usize>>::Error: core::fmt::Debug,
{
    let bits_in_type = T::try_from(8 * core::mem::size_of::<T>()).unwrap();
    rhs < T::default() || rhs >= bits_in_type
}

/// Automatically implements operators over references in addition to the provided operator.
macro_rules! impl_ref_ops {
    // binary op
    {
        impl core::ops::$trait:ident<$rhs:ty> for $type:ty {
            type Output = $output:ty;

            $(#[$attrs:meta])*
            fn $fn:ident($self_tok:ident, $rhs_arg:ident: $rhs_arg_ty:ty) -> Self::Output $body:tt
        }
    } => {
        impl core::ops::$trait<$rhs> for $type {
            type Output = $output;

            $(#[$attrs])*
            fn $fn($self_tok, $rhs_arg: $rhs_arg_ty) -> Self::Output $body
        }

        impl core::ops::$trait<&'_ $rhs> for $type {
            type Output = <$type as core::ops::$trait<$rhs>>::Output;

            $(#[$attrs])*
            fn $fn($self_tok, $rhs_arg: &$rhs) -> Self::Output {
                core::ops::$trait::$fn($self_tok, *$rhs_arg)
            }
        }

        impl core::ops::$trait<$rhs> for &'_ $type {
            type Output = <$type as core::ops::$trait<$rhs>>::Output;

            $(#[$attrs])*
            fn $fn($self_tok, $rhs_arg: $rhs) -> Self::Output {
                core::ops::$trait::$fn(*$self_tok, $rhs_arg)
            }
        }

        impl core::ops::$trait<&'_ $rhs> for &'_ $type {
            type Output = <$type as core::ops::$trait<$rhs>>::Output;

            $(#[$attrs])*
            fn $fn($self_tok, $rhs_arg: &$rhs) -> Self::Output {
                core::ops::$trait::$fn(*$self_tok, *$rhs_arg)
            }
        }
    };

    // binary assignment op
    {
        impl core::ops::$trait:ident<$rhs:ty> for $type:ty {
            $(#[$attrs:meta])*
            fn $fn:ident(&mut $self_tok:ident, $rhs_arg:ident: $rhs_arg_ty:ty) $body:tt
        }
    } => {
        impl core::ops::$trait<$rhs> for $type {
            $(#[$attrs])*
            fn $fn(&mut $self_tok, $rhs_arg: $rhs_arg_ty) $body
        }

        impl core::ops::$trait<&'_ $rhs> for $type {
            $(#[$attrs])*
            fn $fn(&mut $self_tok, $rhs_arg: &$rhs_arg_ty) {
                core::ops::$trait::$fn($self_tok, *$rhs_arg)
            }
        }
    };

    // unary op
    {
        impl core::ops::$trait:ident for $type:ty {
            type Output = $output:ty;
            fn $fn:ident($self_tok:ident) -> Self::Output $body:tt
        }
    } => {
        impl core::ops::$trait for $type {
            type Output = $output;
            fn $fn($self_tok) -> Self::Output $body
        }

        impl core::ops::$trait for &'_ $type {
            type Output = <$type as core::ops::$trait>::Output;
            fn $fn($self_tok) -> Self::Output {
                core::ops::$trait::$fn(*$self_tok)
            }
        }
    }
}

/// Implements op traits for masks
macro_rules! impl_mask_ops {
    { $($mask:ty),* } => {
        $(
            impl_ref_ops! {
                impl core::ops::BitAnd<$mask> for $mask {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self::Output {
                        Self(self.0 & rhs.0)
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::BitAndAssign<$mask> for $mask {
                    fn bitand_assign(&mut self, rhs: Self) {
                        *self = *self & rhs;
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::BitOr<$mask> for $mask {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self::Output {
                        Self(self.0 | rhs.0)
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::BitOrAssign<$mask> for $mask {
                    fn bitor_assign(&mut self, rhs: Self) {
                        *self = *self | rhs;
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::BitXor<$mask> for $mask {
                    type Output = Self;
                    fn bitxor(self, rhs: Self) -> Self::Output {
                        Self(self.0 ^ rhs.0)
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::BitXorAssign<$mask> for $mask {
                    fn bitxor_assign(&mut self, rhs: Self) {
                        *self = *self ^ rhs;
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::Not for $mask {
                    type Output = Self;
                    fn not(self) -> Self::Output {
                        Self(!self.0)
                    }
                }
            }
        )*
    }
}
impl_mask_ops! { crate::mask8, crate::mask16, crate::mask32, crate::mask64, crate::mask128, crate::masksize }

/// Automatically implements operators over vectors and scalars for a particular vector.
macro_rules! impl_op {
    { impl Add for $type:ty } => {
        impl_op! { impl Add<$type> for $type }
        impl_op! { impl Add< <$type as crate::Vector>::Scalar > for $type }
    };
    { impl Sub for $type:ty } => {
        impl_op! { impl Sub<$type> for $type }
        impl_op! { impl Sub< <$type as crate::Vector>::Scalar > for $type }
    };
    { impl Mul for $type:ty } => {
        impl_op! { impl Mul<$type> for $type }
        impl_op! { impl Mul< <$type as crate::Vector>::Scalar > for $type }
    };
    { impl Div for $type:ty } => {
        impl_op! { impl Div<$type> for $type }
        impl_op! { impl Div< <$type as crate::Vector>::Scalar > for $type }
    };
    { impl Rem for $type:ty } => {
        impl_op! { impl Rem<$type> for $type }
        impl_op! { impl Rem< <$type as crate::Vector>::Scalar > for $type }
    };
    { impl BitAnd for $type:ty } => {
        impl_op! { impl BitAnd<$type> for $type }
        impl_op! { impl BitAnd< <$type as crate::Vector>::Scalar > for $type }
    };
    { impl BitOr for $type:ty } => {
        impl_op! { impl BitOr<$type> for $type }
        impl_op! { impl BitOr< <$type as crate::Vector>::Scalar > for $type }
    };
    { impl BitXor for $type:ty } => {
        impl_op! { impl BitXor<$type> for $type }
        impl_op! { impl BitXor< <$type as crate::Vector>::Scalar > for $type }
    };

    { impl Add<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, Add::add, AddAssign::add_assign, simd_add }
    };
    { impl Sub<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, Sub::sub, SubAssign::sub_assign, simd_sub }
    };
    { impl Mul<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, Mul::mul, MulAssign::mul_assign, simd_mul }
    };
    { impl Div<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, Div::div, DivAssign::div_assign, simd_div }
    };
    { impl Rem<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, Rem::rem, RemAssign::rem_assign, simd_rem }
    };
    { impl Shl<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, Shl::shl, ShlAssign::shl_assign, simd_shl }
    };
    { impl Shr<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, Shr::shr, ShrAssign::shr_assign, simd_shr }
    };
    { impl BitAnd<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, BitAnd::bitand, BitAndAssign::bitand_assign, simd_and }
    };
    { impl BitOr<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, BitOr::bitor, BitOrAssign::bitor_assign, simd_or }
    };
    { impl BitXor<$rhs:ty> for $type:ty } => {
        impl_op! { @binary $type, $rhs, BitXor::bitxor, BitXorAssign::bitxor_assign, simd_xor }
    };

    // Neg and Not implementations
    { impl Not for $type:ty } => {
        impl_ref_ops! {
            impl core::ops::Not for $type {
                type Output = Self;
                fn not(self) -> Self::Output {
                    self ^ <$type>::splat(true.into())
                }
            }
        }
    };

    { impl Neg for $type:ty } => {
        impl_ref_ops! {
            impl core::ops::Neg for $type {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    self * <$type>::splat(-1 as <$type as crate::Vector>::Scalar)
                }
            }
        }
    };

    // generic binary op with assignment when output is `Self`
    { @binary $type:ty, $rhs:ty, $trait:ident :: $trait_fn:ident, $assign_trait:ident :: $assign_trait_fn:ident, $intrinsic:ident } => {
        impl_ref_ops! {
            impl core::ops::$trait<$rhs> for $type {
                type Output = Self;

                #[inline]
                fn $trait_fn(self, rhs: $rhs) -> Self::Output {
                    unsafe {
                        crate::intrinsics::$intrinsic(self, rhs.into())
                    }
                }
            }
        }

        impl_ref_ops! {
            impl core::ops::$assign_trait<$rhs> for $type {
                #[inline]
                fn $assign_trait_fn(&mut self, rhs: $rhs) {
                    unsafe {
                        *self = crate::intrinsics::$intrinsic(*self, rhs.into());
                    }
                }
            }
        }
    };
}

/// Implements relevant operators for the provided types.
macro_rules! impl_op_meta {
    { float: $($vector:ty,)* } => {
        $(
            impl_op! { impl Add for $vector }
            impl_op! { impl Sub for $vector }
            impl_op! { impl Mul for $vector }
            impl_op! { impl Div for $vector }
            impl_op! { impl Rem for $vector }
            impl_op! { impl Neg for $vector }
            impl_op_meta! { @index $vector }
        )*
    };
    { mask: $($vector:ty,)* } => {
        $(
            impl_op! { impl BitAnd for $vector }
            impl_op! { impl BitOr for $vector }
            impl_op! { impl BitXor for $vector }
            impl_op! { impl Not for $vector }
            impl_op_meta! { @index $vector }
        )*
    };
    { unsigned integer: $($vector:ty,)* } => {

        $(
            impl_op! { impl Add for $vector }
            impl_op! { impl Sub for $vector }
            impl_op! { impl Mul for $vector }
            impl_op! { impl Rem for $vector }
            impl_op! { impl BitAnd for $vector }
            impl_op! { impl BitOr for $vector }
            impl_op! { impl BitXor for $vector }
            impl_op! { impl Not for $vector }
            impl_op_meta! { @index $vector }

            // Integers panic on divide by 0
            impl_ref_ops! {
                impl core::ops::Div<$vector> for $vector {
                    type Output = Self;

                    #[inline]
                    fn div(self, rhs: $vector) -> Self::Output {
                        // TODO there is probably a better way of doing this
                        if AsRef::<[<Self as crate::Vector>::Scalar]>::as_ref(&rhs)
                            .iter()
                            .any(|x| *x == 0)
                        {
                            panic!("attempt to divide by zero");
                        }
                        unsafe { crate::intrinsics::simd_div(self, rhs) }
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::Div< <$vector as crate::Vector>::Scalar > for $vector {
                    type Output = $vector;

                    #[inline]
                    fn div(self, rhs: <$vector as crate::Vector>::Scalar) -> Self::Output {
                        if rhs == 0 {
                            panic!("attempt to divide by zero");
                        }
                        let rhs = Self::splat(rhs);
                        unsafe { crate::intrinsics::simd_div(self, rhs) }
                    }
                }
            }


            impl_ref_ops! {
                impl core::ops::DivAssign<$vector> for $vector {
                    #[inline]
                    fn div_assign(&mut self, rhs: Self) {
                        *self = *self / rhs;
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::DivAssign< <Self as crate::Vector>::Scalar > for $vector {
                    #[inline]
                    fn div_assign(&mut self, rhs: <Self as crate::Vector>::Scalar) {
                        *self = *self / rhs;
                    }
                }
            }

            // shifts panic on overflow
            impl_ref_ops! {
                impl core::ops::Shl<$vector> for $vector {
                    type Output = Self;

                    #[inline]
                    fn shl(self, rhs: $vector) -> Self::Output {
                        // TODO there is probably a better way of doing this
                        if AsRef::<[<Self as crate::Vector>::Scalar]>::as_ref(&rhs)
                            .iter()
                            .copied()
                            .any(invalid_shift_rhs)
                        {
                            panic!("attempt to shift left with overflow");
                        }
                        unsafe { crate::intrinsics::simd_shl(self, rhs) }
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::Shl< <$vector as crate::Vector>::Scalar > for $vector {
                    type Output = $vector;

                    #[inline]
                    fn shl(self, rhs: <$vector as crate::Vector>::Scalar) -> Self::Output {
                        if invalid_shift_rhs(rhs) {
                            panic!("attempt to shift left with overflow");
                        }
                        let rhs = Self::splat(rhs);
                        unsafe { crate::intrinsics::simd_shl(self, rhs) }
                    }
                }
            }


            impl_ref_ops! {
                impl core::ops::ShlAssign<$vector> for $vector {
                    #[inline]
                    fn shl_assign(&mut self, rhs: Self) {
                        *self = *self << rhs;
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::ShlAssign< <Self as crate::Vector>::Scalar > for $vector {
                    #[inline]
                    fn shl_assign(&mut self, rhs: <Self as crate::Vector>::Scalar) {
                        *self = *self << rhs;
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::Shr<$vector> for $vector {
                    type Output = Self;

                    #[inline]
                    fn shr(self, rhs: $vector) -> Self::Output {
                        // TODO there is probably a better way of doing this
                        if AsRef::<[<Self as crate::Vector>::Scalar]>::as_ref(&rhs)
                            .iter()
                            .copied()
                            .any(invalid_shift_rhs)
                        {
                            panic!("attempt to shift with overflow");
                        }
                        unsafe { crate::intrinsics::simd_shr(self, rhs) }
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::Shr< <$vector as crate::Vector>::Scalar > for $vector {
                    type Output = $vector;

                    #[inline]
                    fn shr(self, rhs: <$vector as crate::Vector>::Scalar) -> Self::Output {
                        if invalid_shift_rhs(rhs) {
                            panic!("attempt to shift with overflow");
                        }
                        let rhs = Self::splat(rhs);
                        unsafe { crate::intrinsics::simd_shr(self, rhs) }
                    }
                }
            }


            impl_ref_ops! {
                impl core::ops::ShrAssign<$vector> for $vector {
                    #[inline]
                    fn shr_assign(&mut self, rhs: Self) {
                        *self = *self >> rhs;
                    }
                }
            }

            impl_ref_ops! {
                impl core::ops::ShrAssign< <Self as crate::Vector>::Scalar > for $vector {
                    #[inline]
                    fn shr_assign(&mut self, rhs: <Self as crate::Vector>::Scalar) {
                        *self = *self >> rhs;
                    }
                }
            }
        )*
    };
    { signed integer: $($vector:ty,)* } => {
        impl_op_meta! { unsigned integer: $($vector,)* }
        $(
            impl_op! { impl Neg for $vector }
        )*
    };
    { @index $vector:ty } => {
        impl<I> core::ops::Index<I> for $vector
        where
            I: core::slice::SliceIndex<[<$vector as crate::Vector>::Scalar]>,
        {
            type Output = I::Output;
            fn index(&self, index: I) -> &Self::Output {
                let slice: &[_] = self.as_ref();
                &slice[index]
            }
        }

        impl<I> core::ops::IndexMut<I> for $vector
        where
            I: core::slice::SliceIndex<[<$vector as crate::Vector>::Scalar]>,
        {
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                let slice: &mut [_] = self.as_mut();
                &mut slice[index]
            }
        }
    }
}

impl_op_meta! {
    unsigned integer:
        crate::u8x8,    crate::u8x16,   crate::u8x32,   crate::u8x64,
        crate::u16x4,   crate::u16x8,   crate::u16x16,  crate::u16x32,
        crate::u32x2,   crate::u32x4,   crate::u32x8,   crate::u32x16,
        crate::u64x2,   crate::u64x4,   crate::u64x8,
        crate::u128x2,  crate::u128x4,
        crate::usizex2, crate::usizex4, crate::usizex8,
}

impl_op_meta! {
    signed integer:
        crate::i8x8,    crate::i8x16,   crate::i8x32,   crate::i8x64,
        crate::i16x4,   crate::i16x8,   crate::i16x16,  crate::i16x32,
        crate::i32x2,   crate::i32x4,   crate::i32x8,   crate::i32x16,
        crate::i64x2,   crate::i64x4,   crate::i64x8,
        crate::i128x2,  crate::i128x4,
        crate::isizex2, crate::isizex4, crate::isizex8,
}

impl_op_meta! {
    float:
        crate::f32x2, crate::f32x4, crate::f32x8, crate::f32x16,
        crate::f64x2, crate::f64x4, crate::f64x8,
}

impl_op_meta! {
    mask:
        crate::mask8x8,    crate::mask8x16,   crate::mask8x32,   crate::mask8x64,
        crate::mask16x4,   crate::mask16x8,   crate::mask16x16,  crate::mask16x32,
        crate::mask32x2,   crate::mask32x4,   crate::mask32x8,   crate::mask32x16,
        crate::mask64x2,   crate::mask64x4,   crate::mask64x8,
        crate::mask128x2,  crate::mask128x4,
        crate::masksizex2, crate::masksizex4, crate::masksizex8,
}
