macro_rules! impl_op {
    // trait with no rhs arg
    {
        impl $trait:ident for $($type:ty),* $(,)? {
            type Output = $output:ty;
        }
    } => {
        $(
            impl_op! { @impl $trait for $type, $output }
        )*
    };

    // trait with rhs arg
    {
        impl $trait:ident<$rhs:ty> for $($type:ty),* $(,)? {
            type Output = $output:ty;
        }
    } => {
        $(
            impl_op! { @impl $trait<$rhs> for $type, $output }
        )*
    };

    // fill rhs arg for these traits
    { @impl Add for $type:ty, $output:ty } => { impl_op! { @impl Add<$type> for $type, $output } };
    { @impl Sub for $type:ty, $output:ty } => { impl_op! { @impl Sub<$type> for $type, $output } };
    { @impl Mul for $type:ty, $output:ty } => { impl_op! { @impl Mul<$type> for $type, $output } };
    { @impl Div for $type:ty, $output:ty } => { impl_op! { @impl Div<$type> for $type, $output } };
    { @impl Rem for $type:ty, $output:ty } => { impl_op! { @impl Rem<$type> for $type, $output } };
    { @impl Shl for $type:ty, $output:ty } => { impl_op! { @impl Shl<$type> for $type, $output } };
    { @impl Shr for $type:ty, $output:ty } => { impl_op! { @impl Shr<$type> for $type, $output } };
    { @impl BitAnd for $type:ty, $output:ty } => { impl_op! { @impl BitAnd<$type> for $type, $output } };
    { @impl BitOr for $type:ty, $output:ty } => { impl_op! { @impl BitOr<$type> for $type, $output } };
    { @impl BitXor for $type:ty, $output:ty } => { impl_op! { @impl BitXor<$type> for $type, $output } };

    // how to implement these traits
    { @impl Add<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, Add::add, AddAssign::add_assign, simd_add, $output }
    };
    { @impl Sub<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, Sub::sub, SubAssign::sub_assign, simd_sub, $output }
    };
    { @impl Mul<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, Mul::mul, MulAssign::mul_assign, simd_mul, $output }
    };
    { @impl Div<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, Div::div, DivAssign::div_assign, simd_div, $output }
    };
    { @impl Rem<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, Rem::rem, RemAssign::rem_assign, simd_rem, $output }
    };
    { @impl Shl<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, Shl::shl, ShlAssign::shl_assign, simd_shl, $output }
    };
    { @impl Shr<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, Shr::shr, ShrAssign::shr_assign, simd_shr, $output }
    };
    { @impl BitAnd<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, BitAnd::bitand, BitAndAssign::bitand_assign, simd_and, $output }
    };
    { @impl BitOr<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, BitOr::bitor, BitOrAssign::bitor_assign, simd_or, $output }
    };
    { @impl BitXor<$rhs:ty> for $type:ty, $output:ty } => {
        impl_op! { @binary $type, $rhs, BitXor::bitxor, BitXorAssign::bitxor_assign, simd_xor, $output }
    };

    // generic binary op with assignment when output is `Self`
    { @binary $type:ty, $rhs:ty, $trait:ident :: $trait_fn:ident, $assign_trait:ident :: $assign_trait_fn:ident, $intrinsic:ident, Self } => {
        impl core::ops::$trait<$rhs> for $type {
            type Output = Self;
            fn $trait_fn(self, rhs: $rhs) -> Self::Output {
                unsafe {
                    crate::intrinsics::$intrinsic(self, rhs.into())
                }
            }
        }

        impl core::ops::$assign_trait<$rhs> for $type {
            fn $assign_trait_fn(&mut self, rhs: $rhs) {
                unsafe {
                    self = crate::intrinsics::$intrinsic(self, rhs.into());
                }
            }
        }
    };

    // generic binary op with output other than `Self` (meaning no assignment)
    { @binary $type:ty, $rhs:ty, $trait:ident :: $trait_fn:ident, $assign_trait:ident :: $assign_trait_fn:ident, $intrinsic:ident, $output:ty } => {
        impl core::ops::$trait<$rhs> for $type {
            type Output = $output;
            fn $trait_fn(self, rhs: $rhs) -> Self::Output {
                unsafe {
                    crate::intrinsics::$intrinsic(self, rhs.into())
                }
            }
        }
    };
}

impl_op! {
    impl Add for
        crate::u8x2,    crate::u8x4,    crate::u8x8,    crate::u8x16,    crate::u8x32,   crate::u8x64,
        crate::u16x2,   crate::u16x4,   crate::u16x8,   crate::u16x16,   crate::u16x32,
        crate::u32x2,   crate::u32x4,   crate::u32x8,   crate::u32x16,
        crate::u64x2,   crate::u64x4,   crate::u64x8,
        crate::u128x2,  crate::u128x4,
        crate::usizex2, crate::usizex4, crate::usizex8,
        crate::i8x2,    crate::i8x4,    crate::i8x8,    crate::i8x16,    crate::i8x32,   crate::i8x64,
        crate::i16x2,   crate::i16x4,   crate::i16x8,   crate::i16x16,   crate::i16x32,
        crate::i32x2,   crate::i32x4,   crate::i32x8,   crate::i32x16,
        crate::i64x2,   crate::i64x4,   crate::i64x8,
        crate::i128x2,  crate::i128x4,
        crate::isizex2, crate::isizex4, crate::isizex8,
        crate::f32x2,   crate::f32x4,   crate::f32x8,   crate::f32x16,
        crate::f64x2,   crate::f64x4,   crate::f64x8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl Sub for
        crate::u8x2,    crate::u8x4,    crate::u8x8,    crate::u8x16,    crate::u8x32,   crate::u8x64,
        crate::u16x2,   crate::u16x4,   crate::u16x8,   crate::u16x16,   crate::u16x32,
        crate::u32x2,   crate::u32x4,   crate::u32x8,   crate::u32x16,
        crate::u64x2,   crate::u64x4,   crate::u64x8,
        crate::u128x2,  crate::u128x4,
        crate::usizex2, crate::usizex4, crate::usizex8,
        crate::i8x2,    crate::i8x4,    crate::i8x8,    crate::i8x16,    crate::i8x32,   crate::i8x64,
        crate::i16x2,   crate::i16x4,   crate::i16x8,   crate::i16x16,   crate::i16x32,
        crate::i32x2,   crate::i32x4,   crate::i32x8,   crate::i32x16,
        crate::i64x2,   crate::i64x4,   crate::i64x8,
        crate::i128x2,  crate::i128x4,
        crate::isizex2, crate::isizex4, crate::isizex8,
        crate::f32x2,   crate::f32x4,   crate::f32x8,   crate::f32x16,
        crate::f64x2,   crate::f64x4,   crate::f64x8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl Mul for
        crate::u8x2,    crate::u8x4,    crate::u8x8,    crate::u8x16,    crate::u8x32,   crate::u8x64,
        crate::u16x2,   crate::u16x4,   crate::u16x8,   crate::u16x16,   crate::u16x32,
        crate::u32x2,   crate::u32x4,   crate::u32x8,   crate::u32x16,
        crate::u64x2,   crate::u64x4,   crate::u64x8,
        crate::u128x2,  crate::u128x4,
        crate::usizex2, crate::usizex4, crate::usizex8,
        crate::i8x2,    crate::i8x4,    crate::i8x8,    crate::i8x16,    crate::i8x32,   crate::i8x64,
        crate::i16x2,   crate::i16x4,   crate::i16x8,   crate::i16x16,   crate::i16x32,
        crate::i32x2,   crate::i32x4,   crate::i32x8,   crate::i32x16,
        crate::i64x2,   crate::i64x4,   crate::i64x8,
        crate::i128x2,  crate::i128x4,
        crate::isizex2, crate::isizex4, crate::isizex8,
        crate::f32x2,   crate::f32x4,   crate::f32x8,   crate::f32x16,
        crate::f64x2,   crate::f64x4,   crate::f64x8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl Div for
        crate::u8x2,    crate::u8x4,    crate::u8x8,    crate::u8x16,    crate::u8x32,   crate::u8x64,
        crate::u16x2,   crate::u16x4,   crate::u16x8,   crate::u16x16,   crate::u16x32,
        crate::u32x2,   crate::u32x4,   crate::u32x8,   crate::u32x16,
        crate::u64x2,   crate::u64x4,   crate::u64x8,
        crate::u128x2,  crate::u128x4,
        crate::usizex2, crate::usizex4, crate::usizex8,
        crate::i8x2,    crate::i8x4,    crate::i8x8,    crate::i8x16,    crate::i8x32,   crate::i8x64,
        crate::i16x2,   crate::i16x4,   crate::i16x8,   crate::i16x16,   crate::i16x32,
        crate::i32x2,   crate::i32x4,   crate::i32x8,   crate::i32x16,
        crate::i64x2,   crate::i64x4,   crate::i64x8,
        crate::i128x2,  crate::i128x4,
        crate::isizex2, crate::isizex4, crate::isizex8,
        crate::f32x2,   crate::f32x4,   crate::f32x8,   crate::f32x16,
        crate::f64x2,   crate::f64x4,   crate::f64x8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl Rem for
        crate::u8x2,    crate::u8x4,    crate::u8x8,    crate::u8x16,    crate::u8x32,   crate::u8x64,
        crate::u16x2,   crate::u16x4,   crate::u16x8,   crate::u16x16,   crate::u16x32,
        crate::u32x2,   crate::u32x4,   crate::u32x8,   crate::u32x16,
        crate::u64x2,   crate::u64x4,   crate::u64x8,
        crate::u128x2,  crate::u128x4,
        crate::usizex2, crate::usizex4, crate::usizex8,
        crate::i8x2,    crate::i8x4,    crate::i8x8,    crate::i8x16,    crate::i8x32,   crate::i8x64,
        crate::i16x2,   crate::i16x4,   crate::i16x8,   crate::i16x16,   crate::i16x32,
        crate::i32x2,   crate::i32x4,   crate::i32x8,   crate::i32x16,
        crate::i64x2,   crate::i64x4,   crate::i64x8,
        crate::i128x2,  crate::i128x4,
        crate::isizex2, crate::isizex4, crate::isizex8,
        crate::f32x2,   crate::f32x4,   crate::f32x8,   crate::f32x16,
        crate::f64x2,   crate::f64x4,   crate::f64x8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl Shl for
        crate::u8x2,    crate::u8x4,    crate::u8x8,    crate::u8x16,    crate::u8x32,   crate::u8x64,
        crate::u16x2,   crate::u16x4,   crate::u16x8,   crate::u16x16,   crate::u16x32,
        crate::u32x2,   crate::u32x4,   crate::u32x8,   crate::u32x16,
        crate::u64x2,   crate::u64x4,   crate::u64x8,
        crate::u128x2,  crate::u128x4,
        crate::usizex2, crate::usizex4, crate::usizex8,
        crate::i8x2,    crate::i8x4,    crate::i8x8,    crate::i8x16,    crate::i8x32,   crate::i8x64,
        crate::i16x2,   crate::i16x4,   crate::i16x8,   crate::i16x16,   crate::i16x32,
        crate::i32x2,   crate::i32x4,   crate::i32x8,   crate::i32x16,
        crate::i64x2,   crate::i64x4,   crate::i64x8,
        crate::i128x2,  crate::i128x4,
        crate::isizex2, crate::isizex4, crate::isizex8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl Shr for
        crate::u8x2,    crate::u8x4,    crate::u8x8,    crate::u8x16,    crate::u8x32,   crate::u8x64,
        crate::u16x2,   crate::u16x4,   crate::u16x8,   crate::u16x16,   crate::u16x32,
        crate::u32x2,   crate::u32x4,   crate::u32x8,   crate::u32x16,
        crate::u64x2,   crate::u64x4,   crate::u64x8,
        crate::u128x2,  crate::u128x4,
        crate::usizex2, crate::usizex4, crate::usizex8,
        crate::i8x2,    crate::i8x4,    crate::i8x8,    crate::i8x16,    crate::i8x32,   crate::i8x64,
        crate::i16x2,   crate::i16x4,   crate::i16x8,   crate::i16x16,   crate::i16x32,
        crate::i32x2,   crate::i32x4,   crate::i32x8,   crate::i32x16,
        crate::i64x2,   crate::i64x4,   crate::i64x8,
        crate::i128x2,  crate::i128x4,
        crate::isizex2, crate::isizex4, crate::isizex8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl BitAnd for
        crate::u8x2,       crate::u8x4,       crate::u8x8,       crate::u8x16,       crate::u8x32,      crate::u8x64,
        crate::u16x2,      crate::u16x4,      crate::u16x8,      crate::u16x16,      crate::u16x32,
        crate::u32x2,      crate::u32x4,      crate::u32x8,      crate::u32x16,
        crate::u64x2,      crate::u64x4,      crate::u64x8,
        crate::u128x2,     crate::u128x4,
        crate::usizex2,    crate::usizex4,    crate::usizex8,
        crate::i8x2,       crate::i8x4,       crate::i8x8,       crate::i8x16,       crate::i8x32,      crate::i8x64,
        crate::i16x2,      crate::i16x4,      crate::i16x8,      crate::i16x16,      crate::i16x32,
        crate::i32x2,      crate::i32x4,      crate::i32x8,      crate::i32x16,
        crate::i64x2,      crate::i64x4,      crate::i64x8,
        crate::i128x2,     crate::i128x4,
        crate::isizex2,    crate::isizex4,    crate::isizex8,
        crate::mask8x2,    crate::mask8x4,    crate::mask8x8,    crate::mask8x16,    crate::mask8x32,   crate::mask8x64,
        crate::mask16x2,   crate::mask16x4,   crate::mask16x8,   crate::mask16x16,   crate::mask16x32,
        crate::mask32x2,   crate::mask32x4,   crate::mask32x8,   crate::mask32x16,
        crate::mask64x2,   crate::mask64x4,   crate::mask64x8,
        crate::mask128x2,  crate::mask128x4,
        crate::masksizex2, crate::masksizex4, crate::masksizex8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl BitOr for
        crate::u8x2,       crate::u8x4,       crate::u8x8,       crate::u8x16,       crate::u8x32,      crate::u8x64,
        crate::u16x2,      crate::u16x4,      crate::u16x8,      crate::u16x16,      crate::u16x32,
        crate::u32x2,      crate::u32x4,      crate::u32x8,      crate::u32x16,
        crate::u64x2,      crate::u64x4,      crate::u64x8,
        crate::u128x2,     crate::u128x4,
        crate::usizex2,    crate::usizex4,    crate::usizex8,
        crate::i8x2,       crate::i8x4,       crate::i8x8,       crate::i8x16,       crate::i8x32,      crate::i8x64,
        crate::i16x2,      crate::i16x4,      crate::i16x8,      crate::i16x16,      crate::i16x32,
        crate::i32x2,      crate::i32x4,      crate::i32x8,      crate::i32x16,
        crate::i64x2,      crate::i64x4,      crate::i64x8,
        crate::i128x2,     crate::i128x4,
        crate::isizex2,    crate::isizex4,    crate::isizex8,
        crate::mask8x2,    crate::mask8x4,    crate::mask8x8,    crate::mask8x16,    crate::mask8x32,   crate::mask8x64,
        crate::mask16x2,   crate::mask16x4,   crate::mask16x8,   crate::mask16x16,   crate::mask16x32,
        crate::mask32x2,   crate::mask32x4,   crate::mask32x8,   crate::mask32x16,
        crate::mask64x2,   crate::mask64x4,   crate::mask64x8,
        crate::mask128x2,  crate::mask128x4,
        crate::masksizex2, crate::masksizex4, crate::masksizex8,
    {
        type Output = Self;
    }
}

impl_op! {
    impl BitXor for
        crate::u8x2,       crate::u8x4,       crate::u8x8,       crate::u8x16,       crate::u8x32,      crate::u8x64,
        crate::u16x2,      crate::u16x4,      crate::u16x8,      crate::u16x16,      crate::u16x32,
        crate::u32x2,      crate::u32x4,      crate::u32x8,      crate::u32x16,
        crate::u64x2,      crate::u64x4,      crate::u64x8,
        crate::u128x2,     crate::u128x4,
        crate::usizex2,    crate::usizex4,    crate::usizex8,
        crate::i8x2,       crate::i8x4,       crate::i8x8,       crate::i8x16,       crate::i8x32,      crate::i8x64,
        crate::i16x2,      crate::i16x4,      crate::i16x8,      crate::i16x16,      crate::i16x32,
        crate::i32x2,      crate::i32x4,      crate::i32x8,      crate::i32x16,
        crate::i64x2,      crate::i64x4,      crate::i64x8,
        crate::i128x2,     crate::i128x4,
        crate::isizex2,    crate::isizex4,    crate::isizex8,
        crate::mask8x2,    crate::mask8x4,    crate::mask8x8,    crate::mask8x16,    crate::mask8x32,   crate::mask8x64,
        crate::mask16x2,   crate::mask16x4,   crate::mask16x8,   crate::mask16x16,   crate::mask16x32,
        crate::mask32x2,   crate::mask32x4,   crate::mask32x8,   crate::mask32x16,
        crate::mask64x2,   crate::mask64x4,   crate::mask64x8,
        crate::mask128x2,  crate::mask128x4,
        crate::masksizex2, crate::masksizex4, crate::masksizex8,
    {
        type Output = Self;
    }
}
