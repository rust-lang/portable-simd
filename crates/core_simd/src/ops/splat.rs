use super::*;

impl<T, const N: usize> Add<T> for Simd<T, N>
where
    Self: Add,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        self.add(Self::splat(rhs))
    }
}

macro_rules! splat_ops {
    (
        type Lhs = Simd<T, N>;
        type Rhs = T;
        $(impl $op:ident::$call:ident;$)*) => {$(
            impl<T, const N: usize> $op<T> for Simd<T, N>
            where
                Self: $op,
                T: SimdElement,
                LaneCount<N>: SupportedLaneCount,
            {
                type Output = Simd<T, N>;

                #[inline]
                #[must_use = "operator returns a new vector without mutating the inputs"]
                fn $call(&mut self, rhs: T) -> Self::Output {
                    self.$call(Self::splat(rhs))
                }
            })*
    }
}

splat_ops!{
    type Lhs = Simd<T, N>;
    type Rhs = T;

    impl Add::add;
    impl Mul::mul;
    impl Sub::sub;
    impl BitAnd::bitand;
    impl BitOr::bitor;
    impl BitXor::bitxor;
    impl Div::div;
    impl Rem::rem;
    impl Shl::shl;
    impl Shr::shr;
}

// // Integers can always accept add, mul, sub, bitand, bitor, and bitxor.
// // For all of these operations, simd_* intrinsics apply wrapping logic.
// for_base_ops! {
//     T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
//     type Lhs = Simd<T, N>;
//     type Rhs = T;
//     type Output = Self;

//     impl Add::add {
//         unsafe_base { simd_add }
//     }

//     impl Mul::mul {
//         unsafe_base { simd_mul }
//     }

//     impl Sub::sub {
//         unsafe_base { simd_sub }
//     }

//     impl BitAnd::bitand {
//         unsafe_base { simd_and }
//     }

//     impl BitOr::bitor {
//         unsafe_base { simd_or }
//     }

//     impl BitXor::bitxor {
//         unsafe_base { simd_xor }
//     }

//     impl Div::div {
//         int_divrem_guard {
//             const PANIC_ZERO: &'static str = "attempt to divide by zero";
//             simd_div
//         }
//     }

//     impl Rem::rem {
//         int_divrem_guard {
//             const PANIC_ZERO: &'static str = "attempt to calculate the remainder with a divisor of zero";
//             simd_rem
//         }
//     }

//     // The only question is how to handle shifts >= <Int>::BITS?
//     // Our current solution uses wrapping logic.
//     impl Shl::shl {
//         wrap_bitshift { simd_shl }
//     }

//     impl Shr::shr {
//         wrap_bitshift {
//             // This automatically monomorphizes to lshr or ashr, depending,
//             // so it's fine to use it for both UInts and SInts.
//             simd_shr
//         }
//     }
// }

// // We don't need any special precautions here:
// // Floats always accept arithmetic ops, but may become NaN.
// for_base_ops! {
//     T = (f32, f64);
//     type Lhs = Simd<T, N>;
//     type Rhs = Simd<T, N>;
//     type Output = Self;

//     impl Add::add {
//         unsafe_base { simd_add }
//     }

//     impl Mul::mul {
//         unsafe_base { simd_mul }
//     }

//     impl Sub::sub {
//         unsafe_base { simd_sub }
//     }

//     impl Div::div {
//         unsafe_base { simd_div }
//     }

//     impl Rem::rem {
//         unsafe_base { simd_rem }
//     }
// }
