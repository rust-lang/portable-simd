use crate::simd::{LaneCount, Simd, SupportedLaneCount};
trait Vectorize {
    type Vector<const N: usize>
    where
        LaneCount<N>: SupportedLaneCount;

    fn splat<const N: usize>(self) -> Self::Vector<N>
    where
        LaneCount<N>: SupportedLaneCount;
}

macro_rules! vectorize_types {
    (type T = ($($scalar:ident),*);) => {
        $(
        impl Vectorize for $scalar {
            type Vector<const N: usize> where LaneCount<N>: SupportedLaneCount = Simd<$scalar, N>;
            fn splat<const N: usize>(self) -> Self::Vector<N>
            where LaneCount<N>: SupportedLaneCount,
            {
                Self::Vector::from_array([self; N])
            }
        }
        )*}
}

vectorize_types! {
    type T = (f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
}
