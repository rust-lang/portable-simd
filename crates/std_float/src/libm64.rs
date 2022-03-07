#![allow(non_snake_case)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::approx_constant)]
use super::StdLibm;

use super::StdFloat;

use super::simd::{LaneCount, Simd, SupportedLaneCount};

impl<const N: usize> StdLibm for Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type IntType = Simd<i64, N>;
    type UintType = Simd<u64, N>;
    #[inline]
    fn sinh(self) -> Self {
        let LOG2_E =Self ::splat (1.442695040888963407359924681001892137426660660756662389692043961734752676409787833023288579071980359);
        let x = self;
        let a = x.mul_add(LOG2_E, -Self::splat(1.0));
        let b = x.mul_add(-LOG2_E, -Self::splat(1.0));
        (a).exp2() - (b).exp2()
    }
    #[inline]
    fn cosh(self) -> Self {
        let LOG2_E =Self ::splat (1.442695040888963407359924681001892137426660660756662389692043961734752676409787833023288579071980359);
        let x = self;
        let a = x.mul_add(LOG2_E, -Self::splat(1.0));
        let b = x.mul_add(-LOG2_E, -Self::splat(1.0));
        (a).exp2() + (b).exp2()
    }
    #[inline]
    fn tanh(self) -> Self {
        let LOG2_E =Self ::splat (1.442695040888963407359924681001892137426660660756662389692043961734752676409787833023288579071980359);
        let x = self;
        let exp2x = (x * (LOG2_E * Self::splat(2.0))).exp2();
        (exp2x - Self::splat(1.0)) / (exp2x + Self::splat(1.0))
    }
    #[inline]
    fn asinh(self) -> Self {
        let x = self;
        (x + (x * x + Self::splat(1.0)).sqrt()).ln()
    }
    #[inline]
    fn acosh(self) -> Self {
        let NAN = Self::splat(f64::NAN);
        let x = self;
        ((x).lanes_lt(Self::splat(1.0))).select(NAN, (x + (x * x - Self::splat(1.0)).sqrt()).ln())
    }
    #[inline]
    fn atanh(self) -> Self {
        let x = self;
        ((Self::splat(1.0) + x).ln() - (Self::splat(1.0) - x).ln()) * Self::splat(0.5)
    }
    #[inline]
    fn asin(self) -> Self {
        let PI_BY_2 = Self::splat(1.5707963267948966192313216916397514420986);
        let arg = self;
        let LIM = Self::splat(0.70710678118654752440);
        let c = ((arg).lanes_lt(Self::splat(0.0))).select(-PI_BY_2, PI_BY_2);
        let s = ((arg).lanes_lt(Self::splat(0.0))).select(-Self::splat(1.0), Self::splat(1.0));
        let x =
            ((arg * arg).lanes_lt(LIM * LIM)).select(arg, (Self::splat(1.0) - arg * arg).sqrt());
        let y = (Self::splat(0.8373648093412319f64))
            .mul_add(x * x, -Self::splat(2.980106295592163f64))
            .mul_add(x * x, Self::splat(5.042442367613399f64))
            .mul_add(x * x, -Self::splat(5.227353021050702f64))
            .mul_add(x * x, Self::splat(3.7146677455757984f64))
            .mul_add(x * x, -Self::splat(1.8827672802928515f64))
            .mul_add(x * x, Self::splat(0.7180951142924303f64))
            .mul_add(x * x, -Self::splat(0.19178725657932066f64))
            .mul_add(x * x, Self::splat(0.05210781979238637f64))
            .mul_add(x * x, Self::splat(0.00485554931570699f64))
            .mul_add(x * x, Self::splat(0.014746118856810628f64))
            .mul_add(x * x, Self::splat(0.017287003548468568f64))
            .mul_add(x * x, Self::splat(0.022376015418082405f64))
            .mul_add(x * x, Self::splat(0.030381795054318782f64))
            .mul_add(x * x, Self::splat(0.04464286065908419f64))
            .mul_add(x * x, Self::splat(0.07499999995639162f64))
            .mul_add(x * x, Self::splat(0.1666666666668809f64))
            .mul_add(x * x, Self::splat(0.9999999999999997f64))
            * x;
        ((arg * arg).lanes_lt(LIM * LIM)).select(y, c - y * s)
    }
    #[inline]
    fn acos(self) -> Self {
        let PI_BY_2 = Self::splat(1.5707963267948966192313216916397514420986);
        let PI = Self::splat(3.1415926535897932384626433832795028841972);
        let arg = self;
        let LIM = Self::splat(0.70710678118654752440);
        let c = ((arg).lanes_lt(Self::splat(0.0))).select(PI, Self::splat(0.0));
        let s = ((arg).lanes_lt(Self::splat(0.0))).select(Self::splat(1.0), -Self::splat(1.0));
        let x =
            ((arg * arg).lanes_lt(LIM * LIM)).select(arg, (Self::splat(1.0) - arg * arg).sqrt());
        let y = (Self::splat(0.6668533325236312f64))
            .mul_add(x * x, -Self::splat(2.203633342583737f64))
            .mul_add(x * x, Self::splat(3.4682293590554205f64))
            .mul_add(x * x, -Self::splat(3.31825365991194f64))
            .mul_add(x * x, Self::splat(2.1679686827931266f64))
            .mul_add(x * x, -Self::splat(0.9934711561764131f64))
            .mul_add(x * x, Self::splat(0.34673516466685284f64))
            .mul_add(x * x, -Self::splat(0.07465114063751678f64))
            .mul_add(x * x, Self::splat(0.02708987879711642f64))
            .mul_add(x * x, Self::splat(0.011875258490214528f64))
            .mul_add(x * x, Self::splat(0.01755397524017199f64))
            .mul_add(x * x, Self::splat(0.022358737646075745f64))
            .mul_add(x * x, Self::splat(0.03038253331569182f64))
            .mul_add(x * x, Self::splat(0.04464284149373235f64))
            .mul_add(x * x, Self::splat(0.07500000021866425f64))
            .mul_add(x * x, Self::splat(0.16666666666545776f64))
            .mul_add(x * x, Self::splat(1.000000000000001f64))
            * x;
        ((arg * arg).lanes_lt(LIM * LIM)).select(PI_BY_2 - y, c - y * s)
    }
    #[inline]
    fn atan(self) -> Self {
        let PI_BY_2 = Self::splat(1.5707963267948966192313216916397514420986);
        let arg = self;
        let LIM = Self::splat(1.0);
        let c = ((arg).lanes_lt(Self::splat(0.0))).select(-PI_BY_2, PI_BY_2);
        let x = ((arg.abs()).lanes_lt(LIM)).select(arg, arg.recip());
        let y = (-Self::splat(0.000039339860635465445f64))
            .mul_add(x * x, Self::splat(0.0004066164434836197f64))
            .mul_add(x * x, -Self::splat(0.001986001768572495f64))
            .mul_add(x * x, Self::splat(0.006143174006145858f64))
            .mul_add(x * x, -Self::splat(0.013667536945096575f64))
            .mul_add(x * x, Self::splat(0.023696745325204483f64))
            .mul_add(x * x, -Self::splat(0.03413639435272701f64))
            .mul_add(x * x, Self::splat(0.043317460873511335f64))
            .mul_add(x * x, -Self::splat(0.05106904370972279f64))
            .mul_add(x * x, Self::splat(0.058384099848191776f64))
            .mul_add(x * x, -Self::splat(0.0665730796562759f64))
            .mul_add(x * x, Self::splat(0.07690840682218662f64))
            .mul_add(x * x, -Self::splat(0.09090746301914292f64))
            .mul_add(x * x, Self::splat(0.11111099019519444f64))
            .mul_add(x * x, -Self::splat(0.1428571373381894f64))
            .mul_add(x * x, Self::splat(0.19999999986592576f64))
            .mul_add(x * x, -Self::splat(0.3333333333320309f64))
            .mul_add(x * x, Self::splat(0.9999999999999978f64))
            * x;
        ((arg.abs()).lanes_lt(LIM)).select(y, c - y)
    }
    #[inline]
    fn atan2(self, x: Self) -> Self {
        let PI_BY_2 = Self::splat(1.5707963267948966192313216916397514420986);
        let PI = Self::splat(3.1415926535897932384626433832795028841972);
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
        let y3 = (-Self::splat(0.00009430222207292313f64))
            .mul_add(x3 * x3, Self::splat(0.0008815268490338067f64))
            .mul_add(x3 * x3, -Self::splat(0.003880445752738139f64))
            .mul_add(x3 * x3, Self::splat(0.010809286571701573f64))
            .mul_add(x3 * x3, -Self::splat(0.021742691352755614f64))
            .mul_add(x3 * x3, Self::splat(0.03447094444008241f64))
            .mul_add(x3 * x3, -Self::splat(0.04635546886266202f64))
            .mul_add(x3 * x3, Self::splat(0.05651493240009292f64))
            .mul_add(x3 * x3, -Self::splat(0.06602767750343502f64))
            .mul_add(x3 * x3, Self::splat(0.07679373778921496f64))
            .mul_add(x3 * x3, -Self::splat(0.09089066294110684f64))
            .mul_add(x3 * x3, Self::splat(0.11110936152693832f64))
            .mul_add(x3 * x3, -Self::splat(0.14285704110594352f64))
            .mul_add(x3 * x3, Self::splat(0.19999999685566291f64))
            .mul_add(x3 * x3, -Self::splat(0.3333333332944839f64))
            .mul_add(x3 * x3, Self::splat(0.9999999999999193f64))
            * x3;
        y3 + offset2
    }
    #[inline]
    fn exp2(self) -> Self {
        let EXP2_MAX = Self::splat(1023.0f64);
        let EXP2_MIN = -Self::splat(1023.0f64);
        let EXP2_SCALE = Self::splat(4503599627370496.0f64);
        let EXP2_ONE = Self::splat(4607182418800017408.0f64);
        let INFINITY = Self::splat(f64::INFINITY);
        let arg = self;
        let r = arg.round();
        let mul = Self::from_bits((r.mul_add(EXP2_SCALE, EXP2_ONE)).cast::<u64>());
        let x = arg - r;
        let y = (Self::splat(0.00000000044566754636398603f64))
            .mul_add(x, Self::splat(0.000000007075803175956986f64))
            .mul_add(x, Self::splat(0.00000010178051728089911f64))
            .mul_add(x, Self::splat(0.0000013215422480586546f64))
            .mul_add(x, Self::splat(0.000015252733853280778f64))
            .mul_add(x, Self::splat(0.00015403530485719912f64))
            .mul_add(x, Self::splat(0.0013333558146396002f64))
            .mul_add(x, Self::splat(0.009618129107567618f64))
            .mul_add(x, Self::splat(0.05550410866482166f64))
            .mul_add(x, Self::splat(0.2402265069591022f64))
            .mul_add(x, Self::splat(0.6931471805599452f64))
            .mul_add(x, Self::splat(1f64))
            * mul;
        let y1 = ((arg).lanes_gt(EXP2_MAX)).select(INFINITY, y);
        ((r).lanes_lt(EXP2_MIN)).select(Self::splat(0.0), y1)
    }
    #[inline]
    fn exp(self) -> Self {
        let LOG2_E =Self ::splat (1.442695040888963407359924681001892137426660660756662389692043961734752676409787833023288579071980359);
        let arg = self;
        (arg * LOG2_E).exp2()
    }
    #[inline]
    fn exp_m1(self) -> Self {
        let LOG2_E =Self ::splat (1.442695040888963407359924681001892137426660660756662389692043961734752676409787833023288579071980359);
        let EXP2_SCALE = Self::splat(4503599627370496.0f64);
        let EXP2_ONE = Self::splat(4607182418800017408.0f64);
        let arg = self;
        let scaled = arg * LOG2_E;
        let r = scaled.round();
        let mul = Self::from_bits((r.mul_add(EXP2_SCALE, EXP2_ONE)).cast::<u64>());
        let x = scaled - r;
        (Self::splat(0.00000000044566754636398603f64))
            .mul_add(x, Self::splat(0.000000007075803175956986f64))
            .mul_add(x, Self::splat(0.00000010178051728089911f64))
            .mul_add(x, Self::splat(0.0000013215422480586546f64))
            .mul_add(x, Self::splat(0.000015252733853280778f64))
            .mul_add(x, Self::splat(0.00015403530485719912f64))
            .mul_add(x, Self::splat(0.0013333558146396002f64))
            .mul_add(x, Self::splat(0.009618129107567618f64))
            .mul_add(x, Self::splat(0.05550410866482166f64))
            .mul_add(x, Self::splat(0.2402265069591022f64))
            .mul_add(x, Self::splat(0.6931471805599452f64))
            .mul_add(x, -Self::splat(0.0000000000000000061353609179806035f64))
            * mul
            + (mul - Self::splat(1.0))
    }
    #[inline]
    fn log2(self) -> Self {
        let ONE_BITS = Self::UintType::splat(0x3ff0000000000000_u64);
        let ONE_MASK = Self::UintType::splat(0x000fffffffffffff_u64);
        let MIN_POSITIVE = Self::splat(f64::MIN_POSITIVE);
        let INFINITY = Self::splat(f64::INFINITY);
        let NAN = Self::splat(f64::NAN);
        let LOG2_OFFSET = Self::IntType::splat(1023_i64);
        let LOG2_SHIFT = Self::IntType::splat(52_i64);
        let arg = self;
        let arg_bits = arg.to_bits();
        let exponent = (arg_bits.cast::<i64>() >> LOG2_SHIFT) - LOG2_OFFSET;
        let x = Self::from_bits((arg_bits & ONE_MASK) | ONE_BITS) - Self::splat(1.5);
        let y = (Self::splat(0.000059440811569894275f64))
            .mul_add(x, -Self::splat(0.00009384549305785918f64))
            .mul_add(x, Self::splat(0.00007056268243091807f64))
            .mul_add(x, -Self::splat(0.00011279762643562555f64))
            .mul_add(x, Self::splat(0.00022472329897976745f64))
            .mul_add(x, -Self::splat(0.00036098242513245754f64))
            .mul_add(x, Self::splat(0.0005692370613966115f64))
            .mul_add(x, -Self::splat(0.0009250629378630191f64))
            .mul_add(x, Self::splat(0.0015163928320102102f64))
            .mul_add(x, -Self::splat(0.002502038922613527f64))
            .mul_add(x, Self::splat(0.004169747986750192f64))
            .mul_add(x, -Self::splat(0.007036450720529529f64))
            .mul_add(x, Self::splat(0.01206251033391092f64))
            .mul_add(x, -Self::splat(0.021109393020516138f64))
            .mul_add(x, Self::splat(0.03799690641909651f64))
            .mul_add(x, -Self::splat(0.07124419953811696f64))
            .mul_add(x, Self::splat(0.14248839910020777f64))
            .mul_add(x, -Self::splat(0.3205988979754245f64))
            .mul_add(x, Self::splat(0.9617966939259754f64))
            .mul_add(x, Self::splat(0.5849625007211563f64));
        ((arg).lanes_lt(Self::splat(0.0))).select(
            -NAN,
            ((arg).lanes_lt(MIN_POSITIVE)).select(-INFINITY, y + (exponent.cast::<f64>())),
        )
    }
    #[inline]
    fn ln_1p(self) -> Self {
        let arg = self;
        (Self::splat(1.0) + arg).ln()
    }
    #[inline]
    fn ln(self) -> Self {
        let RECIP_LOG2_E = Self::splat(0.6931471805599453094172321214581765680755);
        let arg = self;
        (arg).log2() * RECIP_LOG2_E
    }
    #[inline]
    fn log10(self) -> Self {
        let RECIP_LOG2_10 = Self::splat(0.3010299956639811952137388947244930267682);
        let arg = self;
        (arg).log2() * RECIP_LOG2_10
    }
    #[inline]
    fn log(self, base: Self) -> Self {
        let arg = self;
        (arg).log2() / (base).log2()
    }
    #[inline]
    fn powf(self, y: Self) -> Self {
        let arg = self;
        ((arg).log2() * y).exp2()
    }
    #[inline]
    fn powi(self, y: Self::IntType) -> Self {
        let x = self;
        (x).powf(y.cast::<f64>())
    }
    #[inline]
    fn cbrt(self) -> Self {
        let TWO_THIRDS = Self::splat(0.666666666666666667f64);
        let ONE_THIRD = Self::splat(0.333333333333333333f64);
        let EXP2_ONE = Self::splat(4607182418800017408.0f64);
        let x = self;
        let r = Self::from_bits(
            ((x.abs().to_bits().cast::<f64>()).mul_add(ONE_THIRD, EXP2_ONE * TWO_THIRDS))
                .cast::<u64>(),
        );
        let r = r + (x.abs() - r * r * r) / (Self::splat(3.0) * r * r);
        let r = r + (x.abs() - r * r * r) / (Self::splat(3.0) * r * r);
        let r = r + (x.abs() - r * r * r) / (Self::splat(3.0) * r * r);
        let r = r + (x.abs() - r * r * r) / (Self::splat(3.0) * r * r);
        r.copysign(x)
    }
    #[inline]
    fn hypot(self, y: Self) -> Self {
        let MIN_POSITIVE = Self::splat(f64::MIN_POSITIVE);
        let x = self;
        let xgty = (x.abs()).lanes_gt(y.abs());
        let x2 = (xgty).select(x, y);
        let y2 = (xgty).select(y, x);
        ((x2.abs()).lanes_le(MIN_POSITIVE)).select(
            x2,
            x2.abs() * (Self::splat(1.0) + (y2 / x2) * (y2 / x2)).sqrt(),
        )
    }
    #[inline]
    fn sin(self) -> Self {
        let RECIP_2PI = Self::splat(0.1591549430918953357688837633725143620345);
        let arg = self;
        let scaled = arg * RECIP_2PI;
        let x = scaled - scaled.round();
        (-Self::splat(0.00007959781355646816f64))
            .mul_add(x * x, Self::splat(0.0011251039233483632f64))
            .mul_add(x * x, -Self::splat(0.012029309381583758f64))
            .mul_add(x * x, Self::splat(0.10422859417031961f64))
            .mul_add(x * x, -Self::splat(0.718122207748485f64))
            .mul_add(x * x, Self::splat(3.8199525744232106f64))
            .mul_add(x * x, -Self::splat(15.094642576059076f64))
            .mul_add(x * x, Self::splat(42.058693944862014f64))
            .mul_add(x * x, -Self::splat(76.7058597530604f64))
            .mul_add(x * x, Self::splat(81.60524927607504f64))
            .mul_add(x * x, -Self::splat(41.34170224039976f64))
            .mul_add(x * x, Self::splat(6.283185307179586f64))
            * x
    }
    #[inline]
    fn cos(self) -> Self {
        let RECIP_2PI = Self::splat(0.1591549430918953357688837633725143620345);
        let arg = self;
        let scaled = arg * RECIP_2PI;
        let x = scaled - scaled.round();
        (Self::splat(0.00002092503869007534f64))
            .mul_add(x * x, -Self::splat(0.0003214576104012376f64))
            .mul_add(x * x, Self::splat(0.003779202401314546f64))
            .mul_add(x * x, -Self::splat(0.03638267368288368f64))
            .mul_add(x * x, Self::splat(0.28200593868080975f64))
            .mul_add(x * x, -Self::splat(1.7143907074899654f64))
            .mul_add(x * x, Self::splat(7.903536371025055f64))
            .mul_add(x * x, -Self::splat(26.426256783358706f64))
            .mul_add(x * x, Self::splat(60.244641371876135f64))
            .mul_add(x * x, -Self::splat(85.45681720669371f64))
            .mul_add(x * x, Self::splat(64.9393940226683f64))
            .mul_add(x * x, -Self::splat(19.739208802178716f64))
            .mul_add(x * x, Self::splat(1f64))
    }
    #[inline]
    fn tan(self) -> Self {
        let RECIP_PI = Self::splat(0.3183098861837906715377675267450287240689);
        let arg = self;
        let scaled = arg * RECIP_PI;
        let x = scaled - scaled.round();
        let recip = Self::splat(1.0) / (x * x - Self::splat(0.25));
        let y = (Self::splat(0.00015634929503112444f64))
            .mul_add(x * x, Self::splat(0.00010749666907629025f64))
            .mul_add(x * x, Self::splat(0.00040923484089717195f64))
            .mul_add(x * x, Self::splat(0.0008549505315816931f64))
            .mul_add(x * x, Self::splat(0.0019412482440671268f64))
            .mul_add(x * x, Self::splat(0.004371782765072613f64))
            .mul_add(x * x, Self::splat(0.009879869124007078f64))
            .mul_add(x * x, Self::splat(0.02251293831770456f64))
            .mul_add(x * x, Self::splat(0.05263664423645279f64))
            .mul_add(x * x, Self::splat(0.13476940059037382f64))
            .mul_add(x * x, Self::splat(0.5577362635648092f64))
            .mul_add(x * x, -Self::splat(0.7853981633974483f64))
            * x;
        y * recip
    }
}
