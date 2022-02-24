const NUM_ITER: usize = 0x10000;

macro_rules! test_vec {
    (
        vector_type: $vector_type: ty,
        scalar_fn: $scalar_fn: expr,
        vector_fn: $vector_fn: expr,
        limit: $limit: expr,
        x: $x: expr,
    ) => ({
        let sf = $scalar_fn;
        let vf = $vector_fn;
        let yref = <$vector_type>::from_array([sf($x[0]), sf($x[1]), sf($x[2]), sf($x[3])]);
        let y = vf($x);
        let e = (y - yref);
        let bit_match = y.to_bits().lanes_eq(yref.to_bits());
        let val_ok = bit_match | e.abs().lanes_le($limit);
        if !val_ok.all() || y.is_nan() != yref.is_nan() {
            panic!("\nx     ={:20.16?}\ne     ={:20.16?}\nlimit ={:20.16?}\nvector={:20.16?}\nscalar={:20.16?}\nvector={:020x?}\nscalar={:020x?}\nvector_fn={}",
                $x,
                e,
                $limit,
                y, yref,
                y.to_bits(), yref.to_bits(),
                stringify!($vector_fn)
            );
        }
    });
}

macro_rules! test_range {
    (
        min: $min: expr,
        max: $max: expr,
        limit: $limit: expr,
        scalar_fn: $scalar_fn: expr,
        vector_fn: $vector_fn: expr,
        scalar_type: $scalar_type: ty,
        vector_type: $vector_type: ty,
    ) => {{
        let limit = <$vector_type>::splat($limit);
        let b = (($max) - ($min)) * (1.0 / NUM_ITER as $scalar_type);
        let a = $min;
        for i in (0..NUM_ITER / 4) {
            let fi = (i * 4) as $scalar_type;
            let x = <$vector_type>::from_array([
                (fi + 0.0) * b + a,
                (fi + 1.0) * b + a,
                (fi + 2.0) * b + a,
                (fi + 3.0) * b + a,
            ]);
            test_vec!(
                vector_type: $vector_type,
                scalar_fn: $scalar_fn,
                vector_fn: $vector_fn,
                limit: limit,
                x: x,
            )
        }
    }};
    (
        value: $value: expr,
        limit: $limit: expr,
        scalar_fn: $scalar_fn: expr,
        vector_fn: $vector_fn: expr,
        scalar_type: $scalar_type: ty,
        vector_type: $vector_type: ty,
    ) => {{
        let limit = <$vector_type>::splat($value);
        let x = <$vector_type>::splat($value);
        test_vec!(
            vector_type: $vector_type,
            scalar_fn: $scalar_fn,
            vector_fn: $vector_fn,
            limit: limit,
            x: x,
        )
    }};
}

#[test]
fn sin_f32() {
    use crate::StdLibm;
    use core::f32::consts::PI;
    use core_simd::f32x4;

    test_range!(
        min: -PI/4.0,
        max: PI/4.0,
        limit: f32::EPSILON * 1.0,
        scalar_fn: |x : f32| x.sin(),
        vector_fn: |x : f32x4| x.sin(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: -PI/2.0,
        max: PI/2.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.sin(),
        vector_fn: |x : f32x4| x.sin(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: -PI,
        max: PI,
        limit: f32::EPSILON * 8.0,
        scalar_fn: |x : f32| x.sin(),
        vector_fn: |x : f32x4| x.sin(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn cos_f32() {
    use crate::StdLibm;
    use core::f32::consts::PI;
    use core_simd::f32x4;

    // In the range +/- pi/4 the input has 1 ulp of error.
    test_range!(
        min: -PI/4.0,
        max: PI/4.0,
        limit: f32::EPSILON * 1.0,
        scalar_fn: |x : f32| x.cos(),
        vector_fn: |x : f32x4| x.cos(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    // In the range +/- pi/2 the input and output has 2 ulp of error.
    test_range!(
        min: -PI/2.0,
        max: PI/2.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.cos(),
        vector_fn: |x : f32x4| x.cos(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    // In the range +/- pi the input has 4 ulp of error and the output has 5.
    // Note that the scalar cos also has this error but the implementation
    // is different.
    test_range!(
        min: -PI,
        max: PI,
        limit: f32::EPSILON * 8.0,
        scalar_fn: |x : f32| x.cos(),
        vector_fn: |x : f32x4| x.cos(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn tan_f32() {
    use crate::StdLibm;
    use core::f32::consts::PI;
    use core_simd::f32x4;

    // For the outsides, reciprocal accuracy is important.
    // Note that the vector function correctly gets -inf for -PI/2
    // but the scalar function does not.
    test_range!(
        min: -PI/2.0 + 0.00001,
        max: -PI/4.0,
        limit: f32::EPSILON * 3.0,
        scalar_fn: |x : f32| x.tan().recip(),
        vector_fn: |x : f32x4| x.tan().recip(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    // For the insides, absolute accuracy is important.
    test_range!(
        min: -PI/4.0,
        max: PI/4.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.tan(),
        vector_fn: |x : f32x4| x.tan(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: PI/4.0,
        max: PI/2.0 - 0.00001,
        limit: f32::EPSILON * 3.0,
        scalar_fn: |x : f32| x.tan().recip(),
        vector_fn: |x : f32x4| x.tan().recip(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn asin_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 8.0,
        scalar_fn: |x : f32| x.asin(),
        vector_fn: |x : f32x4| x.asin(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: -0.5,
        max: 0.5,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.asin(),
        vector_fn: |x : f32x4| x.asin(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn atan_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 8.0,
        scalar_fn: |x : f32| x.atan(),
        vector_fn: |x : f32x4| x.atan(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 8.0,
        scalar_fn: |x : f32| x.recip().atan(),
        vector_fn: |x : f32x4| x.recip().atan(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn acos_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 6.0,
        scalar_fn: |x : f32| x.acos(),
        vector_fn: |x : f32x4| x.acos(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn exp2_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        value: -126.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.exp2(),
        vector_fn: |x : f32x4| x.exp2(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    // Denormals not supported.
    //
    // test_range!(
    //     value: -127.0,
    //     limit: f32::EPSILON * 2.0,
    //     scalar_fn: |x : f32| x.exp2(),
    //     vector_fn: |x : f32x4| x.exp2(),
    //     scalar_type: f32,
    //     vector_type: f32x4,
    // );

    test_range!(
        value: -200.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.exp2(),
        vector_fn: |x : f32x4| x.exp2(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.exp2(),
        vector_fn: |x : f32x4| x.exp2(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: -126.0,
        max: 126.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.exp2().log2(),
        vector_fn: |x : f32x4| x.exp2().log2(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn exp_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: -2.0,
        max: 0.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.exp(),
        vector_fn: |x : f32x4| x.exp(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 0.0,
        max: 2.0,
        limit: f32::EPSILON * 8.0,
        scalar_fn: |x : f32| x.exp(),
        vector_fn: |x : f32x4| x.exp(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn log2_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        value: -1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log2(),
        vector_fn: |x : f32x4| x.log2(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        value: 0.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log2(),
        vector_fn: |x : f32x4| x.log2(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    // Note that the std library may accept denormals.
    test_range!(
        value: f32::MIN_POSITIVE,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log2(),
        vector_fn: |x : f32x4| x.log2(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log2(),
        vector_fn: |x : f32x4| x.log2(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 2.0,
        max: 4.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log2(),
        vector_fn: |x : f32x4| x.log2(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 4.0,
        max: 8.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log2(),
        vector_fn: |x : f32x4| x.log2(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn ln_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        value: -1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.ln(),
        vector_fn: |x : f32x4| x.ln(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        value: 0.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.ln(),
        vector_fn: |x : f32x4| x.ln(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        value: f32::MIN_POSITIVE,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.ln(),
        vector_fn: |x : f32x4| x.ln(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.ln(),
        vector_fn: |x : f32x4| x.ln(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 2.0,
        max: 4.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.ln(),
        vector_fn: |x : f32x4| x.ln(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 4.0,
        max: 8.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.ln(),
        vector_fn: |x : f32x4| x.ln(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn log10_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log10(),
        vector_fn: |x : f32x4| x.log10(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 2.0,
        max: 4.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log10(),
        vector_fn: |x : f32x4| x.log10(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 4.0,
        max: 8.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log10(),
        vector_fn: |x : f32x4| x.log10(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn ln_1p_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: 0.0,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.ln_1p(),
        vector_fn: |x : f32x4| x.ln_1p(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn log_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.log(2.0),
        vector_fn: |x : f32x4| x.log(f32x4::splat(2.0)),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn powf_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: 0.5,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.powf(2.0),
        vector_fn: |x : f32x4| x.powf(f32x4::splat(2.0)),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: f32::EPSILON * 5.0,
        scalar_fn: |x : f32| x.powf(2.0),
        vector_fn: |x : f32x4| x.powf(f32x4::splat(2.0)),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn powi_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;
    use core_simd::i32x4;

    test_range!(
        min: 0.5,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.powi(2),
        vector_fn: |x : f32x4| x.powi(i32x4::splat(2)),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: f32::EPSILON * 5.0,
        scalar_fn: |x : f32| x.powi(2),
        vector_fn: |x : f32x4| x.powi(i32x4::splat(2)),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn sinh_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.sinh(),
        vector_fn: |x : f32x4| x.sinh(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn cosh_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.cosh(),
        vector_fn: |x : f32x4| x.cosh(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn tanh_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.tanh(),
        vector_fn: |x : f32x4| x.tanh(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn asinh_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.asinh(),
        vector_fn: |x : f32x4| x.asinh(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn acosh_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    // Will be NAN in this range.
    test_range!(
        min: 0.0,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.acosh(),
        vector_fn: |x : f32x4| x.acosh(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.acosh(),
        vector_fn: |x : f32x4| x.acosh(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}

#[test]
fn atanh_f32() {
    use crate::StdLibm;
    use core_simd::f32x4;

    test_range!(
        value: -1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.atanh(),
        vector_fn: |x : f32x4| x.atanh(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        value: 1.0,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.atanh(),
        vector_fn: |x : f32x4| x.atanh(),
        scalar_type: f32,
        vector_type: f32x4,
    );

    test_range!(
        min: -0.75,
        max: 0.75,
        limit: f32::EPSILON * 2.0,
        scalar_fn: |x : f32| x.atanh(),
        vector_fn: |x : f32x4| x.atanh(),
        scalar_type: f32,
        vector_type: f32x4,
    );
}
