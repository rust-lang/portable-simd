#![allow(non_snake_case)]
#![doc("This code is automatically generated, do not edit.")]
use super::StdLibm;

use super::StdFloat;

use super::simd::{LaneCount, Simd, SupportedLaneCount};

impl<const N: usize> StdLibm for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type IntType = Simd<i32, N>;
    type UintType = Simd<u32, N>;
    #[inline]
    fn asin(self) -> Self {
        let PI_BY_2 = Self::splat(1.57079632679489661923);
        let arg = self;
        let LIM = Self::splat(0.70710678118654752440);
        let c = ((arg).lanes_lt(Self::splat(0.0))).select(-PI_BY_2, PI_BY_2);
        let s = ((arg).lanes_lt(Self::splat(0.0))).select(-Self::splat(1.0), Self::splat(1.0));
        let x =
            ((arg * arg).lanes_lt(LIM * LIM)).select(arg, (Self::splat(1.0) - arg * arg).sqrt());
        let y = (Self::splat(0.12778643f32))
            .mul_add(x * x, -Self::splat(0.12145509f32))
            .mul_add(x * x, Self::splat(0.09684546f32))
            .mul_add(x * x, Self::splat(0.009571692f32))
            .mul_add(x * x, Self::splat(0.047712374f32))
            .mul_add(x * x, Self::splat(0.07478066f32))
            .mul_add(x * x, Self::splat(0.1666726f32))
            .mul_add(x * x, Self::splat(1f32))
            * x;
        ((arg * arg).lanes_lt(LIM * LIM)).select(y, c - y * s)
    }
    #[inline]
    fn acos(self) -> Self {
        let PI_BY_2 = Self::splat(1.57079632679489661923);
        let PI = Self::splat(3.14159265358979323846);
        let arg = self;
        let LIM = Self::splat(0.70710678118654752440);
        let c = ((arg).lanes_lt(Self::splat(0.0))).select(PI, Self::splat(0.0));
        let s = ((arg).lanes_lt(Self::splat(0.0))).select(Self::splat(1.0), -Self::splat(1.0));
        let x =
            ((arg * arg).lanes_lt(LIM * LIM)).select(arg, (Self::splat(1.0) - arg * arg).sqrt());
        let y = (Self::splat(0.12778643f32))
            .mul_add(x * x, -Self::splat(0.12145509f32))
            .mul_add(x * x, Self::splat(0.09684546f32))
            .mul_add(x * x, Self::splat(0.009571692f32))
            .mul_add(x * x, Self::splat(0.047712374f32))
            .mul_add(x * x, Self::splat(0.07478066f32))
            .mul_add(x * x, Self::splat(0.1666726f32))
            .mul_add(x * x, Self::splat(1f32))
            * x;
        ((arg * arg).lanes_lt(LIM * LIM)).select(PI_BY_2 - y, c - y * s)
    }
    #[inline]
    fn atan(self) -> Self {
        let PI_BY_2 = Self::splat(1.57079632679489661923);
        let arg = self;
        let LIM = Self::splat(1.0);
        let c = ((arg).lanes_lt(Self::splat(0.0))).select(-PI_BY_2, PI_BY_2);
        let x = ((arg.abs()).lanes_lt(LIM)).select(arg, arg.recip());
        let y = (-Self::splat(0.0039602574f32))
            .mul_add(x * x, Self::splat(0.021659138f32))
            .mul_add(x * x, -Self::splat(0.05587457f32))
            .mul_add(x * x, Self::splat(0.09664151f32))
            .mul_add(x * x, -Self::splat(0.13930209f32))
            .mul_add(x * x, Self::splat(0.19954468f32))
            .mul_add(x * x, -Self::splat(0.33331004f32))
            .mul_add(x * x, Self::splat(0.9999998f32))
            * x;
        ((arg.abs()).lanes_lt(LIM)).select(y, c - y)
    }
    #[inline]
    fn atan2(self, x: Self) -> Self {
        let PI_BY_2 = Self::splat(1.57079632679489661923);
        let PI = Self::splat(3.14159265358979323846);
        let y = self;
        let offset180 = ((y).lanes_lt(Self::splat(0.0))).select(-PI, PI);
        let x1 = ((x).lanes_lt(Self::splat(0.0))).select(-x, x);
        let y1 = ((x).lanes_lt(Self::splat(0.0))).select(-y, y);
        let offset1 = ((x).lanes_lt(Self::splat(0.0))).select(offset180, Self::splat(0.0));
        let offset90 = ((y).lanes_lt(Self::splat(0.0))).select(-PI_BY_2, PI_BY_2);
        let x2 = ((y1.abs()).lanes_gt(x1)).select(y1, x1);
        let y2 = ((y1.abs()).lanes_gt(x1)).select(-x1, y1);
        let offset2 = ((y1.abs()).lanes_gt(x1)).select(offset1 + offset90, offset1);
        let x3 = y2 / x2;
        let y3 = (-Self::splat(0.0039602574f32))
            .mul_add(x3 * x3, Self::splat(0.021659138f32))
            .mul_add(x3 * x3, -Self::splat(0.05587457f32))
            .mul_add(x3 * x3, Self::splat(0.09664151f32))
            .mul_add(x3 * x3, -Self::splat(0.13930209f32))
            .mul_add(x3 * x3, Self::splat(0.19954468f32))
            .mul_add(x3 * x3, -Self::splat(0.33331004f32))
            .mul_add(x3 * x3, Self::splat(0.9999998f32))
            * x3;
        y3 + offset2
    }
    #[inline]
    fn exp2(self) -> Self {
        let EXP2_SCALE = Self::splat(8388608.0f32);
        let EXP2_ONE = Self::splat(1065353216.0f32);
        let arg = self;
        let r = arg.round();
        let mul = Self::from_bits((r.mul_add(EXP2_SCALE, EXP2_ONE)).cast::<u32>());
        let x = arg - r;
        (Self::splat(0.000015310081f32))
            .mul_add(x, Self::splat(0.0001547802f32))
            .mul_add(x, Self::splat(0.0013333454f32))
            .mul_add(x, Self::splat(0.009617995f32))
            .mul_add(x, Self::splat(0.05550411f32))
            .mul_add(x, Self::splat(0.24022652f32))
            .mul_add(x, Self::splat(0.6931472f32))
            .mul_add(x, Self::splat(1f32))
            * mul
    }
    #[inline]
    fn exp(self) -> Self {
        let LOG2_E =Self ::splat (1.442695040888963407359769137464649992339735961996202908859290566914912486673985594186422766333708408);
        let arg = self;
        (arg * LOG2_E).exp2()
    }
    #[inline]
    fn log2(self) -> Self {
        let ONE_BITS = Self::UintType::splat(0x3f800000_u32);
        let ONE_MASK = Self::UintType::splat(0x007fffff_u32);
        let LOG2_OFFSET = Self::IntType::splat(127_i32);
        let LOG2_SHIFT = Self::IntType::splat(23_i32);
        let arg = self;
        let arg_bits = arg.to_bits();
        let exponent = (arg_bits.cast::<i32>() >> LOG2_SHIFT) - LOG2_OFFSET;
        let x = Self::from_bits((arg_bits & ONE_MASK) | ONE_BITS) - Self::splat(1.5);
        let y = (Self::splat(0.00033940058f32))
            .mul_add(x, -Self::splat(0.0005435155f32))
            .mul_add(x, Self::splat(0.00051382656f32))
            .mul_add(x, -Self::splat(0.0008369385f32))
            .mul_add(x, Self::splat(0.0015296092f32))
            .mul_add(x, -Self::splat(0.0025230509f32))
            .mul_add(x, Self::splat(0.0041680275f32))
            .mul_add(x, -Self::splat(0.007033716f32))
            .mul_add(x, Self::splat(0.012062632f32))
            .mul_add(x, -Self::splat(0.021109587f32))
            .mul_add(x, Self::splat(0.037996903f32))
            .mul_add(x, -Self::splat(0.071244195f32))
            .mul_add(x, Self::splat(0.1424884f32))
            .mul_add(x, -Self::splat(0.3205989f32))
            .mul_add(x, Self::splat(0.9617967f32))
            .mul_add(x, Self::splat(0.5849625f32));
        y + (exponent.cast::<f32>())
    }
    #[inline]
    fn sin(self) -> Self {
        let RECIP_2PI = Self::splat(0.15915494309189533577);
        let arg = self;
        let scaled = arg * RECIP_2PI;
        let x = scaled - scaled.round();
        (-Self::splat(12.26886f32))
            .mul_add(x * x, Self::splat(41.21624f32))
            .mul_add(x * x, -Self::splat(76.58672f32))
            .mul_add(x * x, Self::splat(81.59746f32))
            .mul_add(x * x, -Self::splat(41.34151f32))
            .mul_add(x * x, Self::splat(6.2831845f32))
            * x
    }
    #[inline]
    fn cos(self) -> Self {
        let RECIP_2PI = Self::splat(0.15915494309189533577);
        let arg = self;
        let scaled = arg * RECIP_2PI;
        let x = scaled - scaled.round();
        (Self::splat(6.5286584f32))
            .mul_add(x * x, -Self::splat(25.973276f32))
            .mul_add(x * x, Self::splat(60.17118f32))
            .mul_add(x * x, -Self::splat(85.45092f32))
            .mul_add(x * x, Self::splat(64.939186f32))
            .mul_add(x * x, -Self::splat(19.739206f32))
            .mul_add(x * x, Self::splat(1f32))
    }
    #[inline]
    fn tan(self) -> Self {
        let RECIP_PI = Self::splat(0.31830988618379067154);
        let arg = self;
        let scaled = arg * RECIP_PI;
        let x = scaled - scaled.round();
        let recip = Self::splat(1.0) / (x * x - Self::splat(0.25));
        let y = (Self::splat(0.014397301f32))
            .mul_add(x * x, Self::splat(0.021017345f32))
            .mul_add(x * x, Self::splat(0.05285888f32))
            .mul_add(x * x, Self::splat(0.13475448f32))
            .mul_add(x * x, Self::splat(0.55773664f32))
            .mul_add(x * x, -Self::splat(0.7853982f32))
            * x;
        y * recip
    }
}
