use crate::simd::{LaneCount, Simd, SupportedLaneCount};
pub trait Vectorize
{
    type Vector<const VL: usize> where LaneCount<VL>: SupportedLaneCount;

    fn splat<const N: usize>(self) -> Self::Vector<{ N }>
    where LaneCount<N>: SupportedLaneCount;
}

macro_rules! vectorize_types {
    (type T = ($($scalar:ident),*);) => {
        $(
        impl Vectorize for $scalar {
            type Vector<const VL: usize> where LaneCount<VL>: SupportedLaneCount = Simd<$scalar, VL>;
            fn splat<const N: usize>(self) -> Self::Vector<{ N }>
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

pub trait Scalarize<const N: usize> 
where LaneCount<N>: SupportedLaneCount
{
    type Scalar: Vectorize<Vector<N> = Self>;
}

impl<const N: usize> Scalarize<N> for <f32 as Vectorize>::Vector::<N> where LaneCount<N>: SupportedLaneCount, {
    type Scalar = f32;
}