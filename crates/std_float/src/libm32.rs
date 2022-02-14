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
        let LIM: Self = Self::splat(0.70710678118654752440);
        let c: Self = ((arg).lanes_lt(Self::splat(0.0))).select(-PI_BY_2, PI_BY_2);
        let s: Self =
            ((arg).lanes_lt(Self::splat(0.0))).select(-Self::splat(1.0), Self::splat(1.0));
        let x: Self =
            ((arg * arg).lanes_lt(LIM * LIM)).select(arg, (Self::splat(1.0) - arg * arg).sqrt());
        let y: Self = (Self::splat(0.11644821f32))
            .mul_add(x * x, Self::splat(0.04343228f32))
            .mul_add(x * x, Self::splat(0.17078044f32))
            .mul_add(x * x, Self::splat(0.99991643f32))
            * x;
        ((arg * arg).lanes_lt(LIM * LIM)).select(y, c - y * s)
    }
    #[inline]
    fn acos(self) -> Self {
        let PI_BY_2 = Self::splat(1.57079632679489661923);
        let PI = Self::splat(3.14159265358979323846);
        let arg = self;
        let LIM: Self = Self::splat(0.9);
        let c: Self = ((arg).lanes_lt(Self::splat(0.0))).select(PI, Self::splat(0.0));
        let s: Self =
            ((arg).lanes_lt(Self::splat(0.0))).select(Self::splat(1.0), -Self::splat(1.0));
        let x: Self =
            ((arg * arg).lanes_lt(LIM * LIM)).select(arg, (Self::splat(1.0) - arg * arg).sqrt());
        let y: Self = (Self::splat(1.3740137f32))
            .mul_add(x * x, -Self::splat(3.1993167f32))
            .mul_add(x * x, Self::splat(3.103398f32))
            .mul_add(x * x, -Self::splat(1.4533828f32))
            .mul_add(x * x, Self::splat(0.41395915f32))
            .mul_add(x * x, Self::splat(0.03113007f32))
            .mul_add(x * x, Self::splat(0.16861732f32))
            .mul_add(x * x, Self::splat(0.99998593f32))
            * x;
        ((arg * arg).lanes_lt(LIM * LIM)).select(PI_BY_2 - y, c - y * s)
    }
    #[inline]
    fn atan(self) -> Self {
        let PI_BY_2 = Self::splat(1.57079632679489661923);
        let arg = self;
        let LIM: Self = Self::splat(1.0);
        let c: Self = ((arg).lanes_lt(Self::splat(0.0))).select(-PI_BY_2, PI_BY_2);
        let x: Self = ((arg.abs()).lanes_lt(LIM)).select(arg, arg.recip());
        let y: Self = (-Self::splat(0.0039602574f32))
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
        let offset180: Self = ((y).lanes_lt(Self::splat(0.0))).select(-PI, PI);
        let x1: Self = ((x).lanes_lt(Self::splat(0.0))).select(-x, x);
        let y1: Self = ((x).lanes_lt(Self::splat(0.0))).select(-y, y);
        let offset1: Self = ((x).lanes_lt(Self::splat(0.0))).select(offset180, Self::splat(0.0));
        let offset90: Self = ((y).lanes_lt(Self::splat(0.0))).select(-PI_BY_2, PI_BY_2);
        let x2: Self = ((y1.abs()).lanes_gt(x1)).select(y1, x1);
        let y2: Self = ((y1.abs()).lanes_gt(x1)).select(-x1, y1);
        let offset2: Self = ((y1.abs()).lanes_gt(x1)).select(offset1 + offset90, offset1);
        let x3: Self = y2 / x2;
        let y3: Self = (-Self::splat(0.0039602574f32))
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
        let arg = self;
        let r: Self = arg.round();
        let mul: Self = Self::from_bits(unsafe {
            (r.mul_add(Self::splat(8388608.0f32), Self::splat(1065353216.0f32))).to_uint_unchecked()
        });
        let x: Self = arg - r;
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
    fn sin(self) -> Self {
        let RECIP_2PI = Self::splat(0.15915494309189533577);
        let arg = self;
        let scaled: Self = arg * RECIP_2PI;
        let x: Self = scaled - scaled.round();
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
        let scaled: Self = arg * RECIP_2PI;
        let x: Self = scaled - scaled.round();
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
        let scaled: Self = arg * RECIP_PI;
        let x: Self = scaled - scaled.round();
        let recip: Self = Self::splat(1.0) / (x * x - Self::splat(0.25));
        let y: Self = (Self::splat(0.014397301f32))
            .mul_add(x * x, Self::splat(0.021017345f32))
            .mul_add(x * x, Self::splat(0.05285888f32))
            .mul_add(x * x, Self::splat(0.13475448f32))
            .mul_add(x * x, Self::splat(0.55773664f32))
            .mul_add(x * x, -Self::splat(0.7853982f32))
            * x;
        y * recip
    }
}
