const NUM_ITER: usize = 0x10000;

macro_rules! test_vec {
    (
        scalar_type: $scalar_type: ty,
        vector_type: $vector_type: ty,
        int_vector_type: $int_vector_type: ty,
        scalar_fn: $scalar_fn: expr,
        vector_fn: $vector_fn: expr,
        limit: $limit: expr,
        x: $x: expr,
    ) => ({
        {
            #![allow(non_camel_case_types)]
            use crate::StdLibm;
            type scalar_type = $scalar_type;
            type vector_type = $vector_type;
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
    ) => {{
        #![allow(non_camel_case_types)]
        #![allow(dead_code)]
        #![allow(clippy::approx_constant)]
        type scalar_type = f32;
        type vector_type = core_simd::f32x4;
        type int_vector_type = core_simd::i32x4;
        const PI: scalar_type = 3.1415926535897932384626433832795028841972;

        let limit = vector_type::splat($limit);
        let b = (($max) - ($min)) * (1.0 / NUM_ITER as scalar_type);
        let a = $min;
        for i in (0..NUM_ITER / 4) {
            let fi = (i * 4) as scalar_type;
            let x = vector_type::from_array([
                (fi + 0.0) * b + a,
                (fi + 1.0) * b + a,
                (fi + 2.0) * b + a,
                (fi + 3.0) * b + a,
            ]);
            test_vec!(
                scalar_type: f32,
                vector_type: core_simd::f32x4,
                int_vector_type: core_simd::i32x4,
                scalar_fn: $scalar_fn,
                vector_fn: $vector_fn,
                limit: limit,
                x: x,
            )
        }
    }
    {
        #![allow(non_camel_case_types)]
        #![allow(dead_code)]
        #![allow(unused)]
        #![allow(clippy::approx_constant)]
        type scalar_type = f64;
        type vector_type = core_simd::f64x4;
        type int_vector_type = core_simd::i64x4;
        const PI: scalar_type = 3.1415926535897932384626433832795028841972;

        let limit = vector_type::splat($limit);
        let b = (($max) - ($min)) * (1.0 / NUM_ITER as scalar_type);
        let a = $min;
        for i in (0..NUM_ITER / 4) {
            let fi = (i * 4) as scalar_type;
            let x = vector_type::from_array([
                (fi + 0.0) * b + a,
                (fi + 1.0) * b + a,
                (fi + 2.0) * b + a,
                (fi + 3.0) * b + a,
            ]);
            test_vec!(
                scalar_type: f64,
                vector_type: core_simd::f64x4,
                int_vector_type: core_simd::i64x4,
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
    ) => {{
        #![allow(non_camel_case_types)]
        #![allow(dead_code)]
        {
            type scalar_type = f32;
            type vector_type = core_simd::f32x4;
            let limit = <core_simd::f32x4>::splat($limit);
            let x = <core_simd::f32x4>::splat($value);
            test_vec!(
                scalar_type: f32,
                vector_type: core_simd::f32x4,
                int_vector_type: core_simd::i32x4,
                scalar_fn: $scalar_fn,
                vector_fn: $vector_fn,
                limit: limit,
                x: x,
            )
        }
        {
            type scalar_type = f64;
            type vector_type = core_simd::f64x4;
            let limit = <core_simd::f64x4>::splat($limit);
            let x = <core_simd::f64x4>::splat($value);
            test_vec!(
                scalar_type: f64,
                vector_type: core_simd::f64x4,
                int_vector_type: core_simd::i64x4,
                scalar_fn: $scalar_fn,
                vector_fn: $vector_fn,
                limit: limit,
                x: x,
            )
        }
    }};
}

#[test]
fn sin() {
    test_range!(
        min: -PI/4.0,
        max: PI/4.0,
        limit: scalar_type::EPSILON * 1.0,
        scalar_fn: |x : scalar_type| x.sin(),
        vector_fn: |x : vector_type| x.sin(),
    );

    test_range!(
        min: -PI/2.0,
        max: PI/2.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.sin(),
        vector_fn: |x : vector_type| x.sin(),
    );

    test_range!(
        min: -PI,
        max: PI,
        limit: scalar_type::EPSILON * 8.0,
        scalar_fn: |x : scalar_type| x.sin(),
        vector_fn: |x : vector_type| x.sin(),
    );
}

#[test]
fn cos() {
    // In the range +/- pi/4 the input has 1 ulp of error.
    test_range!(
        min: -PI/4.0,
        max: PI/4.0,
        limit: scalar_type::EPSILON * 1.0,
        scalar_fn: |x : scalar_type| x.cos(),
        vector_fn: |x : vector_type| x.cos(),
    );

    // In the range +/- pi/2 the input and output has 2 ulp of error.
    test_range!(
        min: -PI/2.0,
        max: PI/2.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.cos(),
        vector_fn: |x : vector_type| x.cos(),
    );

    // In the range +/- pi the input has 4 ulp of error and the output has 5.
    // Note that the scalar cos also has this error but the implementation
    // is different.
    test_range!(
        min: -PI,
        max: PI,
        limit: scalar_type::EPSILON * 8.0,
        scalar_fn: |x : scalar_type| x.cos(),
        vector_fn: |x : vector_type| x.cos(),
    );
}

#[test]
fn tan() {
    // For the outsides, reciprocal accuracy is important.
    // Note that the vector function correctly gets -inf for -PI/2
    // but the scalar function does not.
    test_range!(
        min: -PI/2.0 + 0.00001,
        max: -PI/4.0,
        limit: scalar_type::EPSILON * 3.0,
        scalar_fn: |x : scalar_type| x.tan().recip(),
        vector_fn: |x : vector_type| x.tan().recip(),
    );

    // For the insides, absolute accuracy is important.
    test_range!(
        min: -PI/4.0,
        max: PI/4.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.tan(),
        vector_fn: |x : vector_type| x.tan(),
    );

    test_range!(
        min: PI/4.0,
        max: PI/2.0 - 0.00001,
        limit: scalar_type::EPSILON * 3.0,
        scalar_fn: |x : scalar_type| x.tan().recip(),
        vector_fn: |x : vector_type| x.tan().recip(),
    );
}

#[test]
fn asin() {
    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 8.0,
        scalar_fn: |x : scalar_type| x.asin(),
        vector_fn: |x : vector_type| x.asin(),
    );

    test_range!(
        min: -0.5,
        max: 0.5,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.asin(),
        vector_fn: |x : vector_type| x.asin(),
    );
}

#[test]
fn atan() {
    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 8.0,
        scalar_fn: |x : scalar_type| x.atan(),
        vector_fn: |x : vector_type| x.atan(),
    );

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 8.0,
        scalar_fn: |x : scalar_type| x.recip().atan(),
        vector_fn: |x : vector_type| x.recip().atan(),
    );
}

#[test]
fn acos() {
    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 6.0,
        scalar_fn: |x : scalar_type| x.acos(),
        vector_fn: |x : vector_type| x.acos(),
    );
}

#[test]
fn exp2() {
    test_range!(
        value: -126.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.exp2(),
        vector_fn: |x : vector_type| x.exp2(),
    );

    test_range!(
        value: 127.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.exp2(),
        vector_fn: |x : vector_type| x.exp2(),
    );

    // Denormals not supported.
    //
    // test_range!(
    //     value: -127.0,
    //     limit: scalar_type::EPSILON * 2.0,
    //     scalar_fn: |x : scalar_type| x.exp2(),
    //     vector_fn: |x : vector_type| x.exp2(),
    // );

    // Large negatives give zero
    test_range!(
        value: -200.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.exp2(),
        vector_fn: |x : vector_type| x.exp2(),
    );

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.exp2(),
        vector_fn: |x : vector_type| x.exp2(),
    );

    // Accuracy is good over the entire range.
    // (Range expanded because windows exp->log is less accurate)
    test_range!(
        min: -126.0,
        max: 126.0,
        limit: scalar_type::EPSILON * 4.0,
        scalar_fn: |x : scalar_type| x.exp2().log2(),
        vector_fn: |x : vector_type| x.exp2().log2(),
    );
}

#[test]
fn exp() {
    test_range!(
        min: -2.0,
        max: 0.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.exp(),
        vector_fn: |x : vector_type| x.exp(),
    );

    test_range!(
        min: 0.0,
        max: 2.0,
        limit: scalar_type::EPSILON * 8.0,
        scalar_fn: |x : scalar_type| x.exp(),
        vector_fn: |x : vector_type| x.exp(),
    );
}

#[test]
fn log2() {
    // Unix gives -NaN, windows gives +NaN.
    #[cfg(not(target_os = "windows"))]
    test_range!(
        value: -1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log2(),
        vector_fn: |x : vector_type| x.log2(),
    );

    // Both should give Inf.
    test_range!(
        value: 0.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log2(),
        vector_fn: |x : vector_type| x.log2(),
    );

    // Note that the std library may accept denormals.
    test_range!(
        value: scalar_type::MIN_POSITIVE,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log2(),
        vector_fn: |x : vector_type| x.log2(),
    );

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log2(),
        vector_fn: |x : vector_type| x.log2(),
    );

    test_range!(
        min: 2.0,
        max: 4.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log2(),
        vector_fn: |x : vector_type| x.log2(),
    );

    test_range!(
        min: 4.0,
        max: 8.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log2(),
        vector_fn: |x : vector_type| x.log2(),
    );
}

#[test]
fn ln() {
    test_range!(
        value: -1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.ln(),
        vector_fn: |x : vector_type| x.ln(),
    );

    test_range!(
        value: 0.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.ln(),
        vector_fn: |x : vector_type| x.ln(),
    );

    test_range!(
        value: scalar_type::MIN_POSITIVE,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.ln(),
        vector_fn: |x : vector_type| x.ln(),
    );

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.ln(),
        vector_fn: |x : vector_type| x.ln(),
    );

    test_range!(
        min: 2.0,
        max: 4.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.ln(),
        vector_fn: |x : vector_type| x.ln(),
    );

    test_range!(
        min: 4.0,
        max: 8.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.ln(),
        vector_fn: |x : vector_type| x.ln(),
    );
}

#[test]
fn log10() {
    test_range!(
        min: 1.0,
        max: 2.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log10(),
        vector_fn: |x : vector_type| x.log10(),
    );

    test_range!(
        min: 2.0,
        max: 4.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log10(),
        vector_fn: |x : vector_type| x.log10(),
    );

    test_range!(
        min: 4.0,
        max: 8.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log10(),
        vector_fn: |x : vector_type| x.log10(),
    );
}

#[test]
fn ln_1p() {
    test_range!(
        min: 0.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.ln_1p(),
        vector_fn: |x : vector_type| x.ln_1p(),
    );
}

#[test]
fn log() {
    test_range!(
        min: 1.0,
        max: 2.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.log(2.0),
        vector_fn: |x : vector_type| x.log(vector_type::splat(2.0)),
    );
}

#[test]
fn powf() {
    test_range!(
        min: 0.5,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.powf(2.0),
        vector_fn: |x : vector_type| x.powf(vector_type::splat(2.0)),
    );

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: scalar_type::EPSILON * 5.0,
        scalar_fn: |x : scalar_type| x.powf(2.0),
        vector_fn: |x : vector_type| x.powf(vector_type::splat(2.0)),
    );
}

#[test]
fn powi() {
    test_range!(
        min: 0.5,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.powi(2),
        vector_fn: |x : vector_type| x.powi(int_vector_type::splat(2)),
    );

    test_range!(
        min: 1.0,
        max: 2.0,
        limit: scalar_type::EPSILON * 5.0,
        scalar_fn: |x : scalar_type| x.powi(2),
        vector_fn: |x : vector_type| x.powi(int_vector_type::splat(2)),
    );
}

#[test]
fn sinh() {
    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.sinh(),
        vector_fn: |x : vector_type| x.sinh(),
    );
}

#[test]
fn cosh() {
    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.cosh(),
        vector_fn: |x : vector_type| x.cosh(),
    );
}

#[test]
fn tanh() {
    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.tanh(),
        vector_fn: |x : vector_type| x.tanh(),
    );
}

#[test]
fn asinh() {
    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 3.0,
        scalar_fn: |x : scalar_type| x.asinh(),
        vector_fn: |x : vector_type| x.asinh(),
    );
}

#[test]
fn acosh() {
    // Will be NAN in this range.
    test_range!(
        min: 0.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.acosh(),
        vector_fn: |x : vector_type| x.acosh(),
    );

    test_range!(
        min: -1.0,
        max: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.acosh(),
        vector_fn: |x : vector_type| x.acosh(),
    );
}

#[test]
fn atanh() {
    test_range!(
        value: -1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.atanh(),
        vector_fn: |x : vector_type| x.atanh(),
    );

    test_range!(
        value: 1.0,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.atanh(),
        vector_fn: |x : vector_type| x.atanh(),
    );

    test_range!(
        min: -0.75,
        max: 0.75,
        limit: scalar_type::EPSILON * 2.0,
        scalar_fn: |x : scalar_type| x.atanh(),
        vector_fn: |x : vector_type| x.atanh(),
    );
}

#[test]
fn cbrt() {
    test_range!(
        min: -8.0,
        max: 8.0,
        limit: scalar_type::EPSILON * 3.0,
        scalar_fn: |x : scalar_type| x.cbrt(),
        vector_fn: |x : vector_type| x.cbrt(),
    );
}

#[test]
fn hypot() {
    test_range!(
        min: 0.0,
        max: 8.0,
        limit: scalar_type::EPSILON * 6.0,
        scalar_fn: |x : scalar_type| x.cos().hypot(x.sin()),
        vector_fn: |x : vector_type| x.cos().hypot(x.sin()),
    );

    // Large values will mostly not overflow.
    test_range!(
        value: scalar_type::MAX/2.0,
        limit: scalar_type::MAX * scalar_type::EPSILON * 6.0,
        scalar_fn: |x : scalar_type| x.hypot(x),
        vector_fn: |x : vector_type| x.hypot(x),
    );

    // Except for MAX.
    test_range!(
        value: scalar_type::MAX,
        limit: scalar_type::EPSILON * 6.0,
        scalar_fn: |x : scalar_type| x.hypot(x),
        vector_fn: |x : vector_type| x.hypot(x),
    );
}

#[test]
fn atan2() {
    // Studiously ignore -PI and PI where signs change erraticly.
    test_range!(
        min: -3.141,
        max: 3.141,
        limit: scalar_type::EPSILON * 4.0,
        scalar_fn: |x : scalar_type| x.sin().atan2(x.cos()),
        vector_fn: |x : vector_type| x.sin().atan2(x.cos()),
    );

    // East
    test_range!(
        value: 0.0,
        limit: scalar_type::EPSILON * 6.0,
        scalar_fn: |x : scalar_type| x.atan2(scalar_type::from(1.0)),
        vector_fn: |x : vector_type| x.atan2(vector_type::splat(1.0)),
    );

    // West
    test_range!(
        value: 0.0,
        limit: scalar_type::EPSILON * 6.0,
        scalar_fn: |x : scalar_type| x.atan2(scalar_type::from(-1.0)),
        vector_fn: |x : vector_type| x.atan2(vector_type::splat(-1.0)),
    );

    // North
    test_range!(
        value: 1.0,
        limit: scalar_type::EPSILON * 6.0,
        scalar_fn: |x : scalar_type| x.atan2(scalar_type::from(0.0)),
        vector_fn: |x : vector_type| x.atan2(vector_type::splat(0.0)),
    );

    // South
    test_range!(
        value: -1.0,
        limit: scalar_type::EPSILON * 6.0,
        scalar_fn: |x : scalar_type| x.atan2(scalar_type::from(0.0)),
        vector_fn: |x : vector_type| x.atan2(vector_type::splat(0.0)),
    );
}
