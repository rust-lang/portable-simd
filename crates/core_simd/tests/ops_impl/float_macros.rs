macro_rules! float_tests {
    { $vector:ident, $scalar:ident } => {
        #[cfg(test)]
        mod $vector {
            use super::*;
            use helpers::lanewise::*;

            #[cfg(target_arch = "wasm32")]
            use wasm_bindgen_test::*;

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_test_configure!(run_in_browser);

            // TODO impl this as an associated fn on vectors
            fn from_slice(slice: &[$scalar]) -> core_simd::$vector {
                let mut value = core_simd::$vector::default();
                let value_slice: &mut [_] = value.as_mut();
                value_slice.copy_from_slice(&slice[0..value_slice.len()]);
                value
            }

            const A: [$scalar; 16] = [0.,   1.,  2.,  3.,  4.,  5.,  6.,  7.,  8.,  9., 10., 11., 12., 13., 14., 15.];
            const B: [$scalar; 16] = [16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29., 30., 31.];

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn add() {
                let a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Add::add);
                assert_biteq!(a + b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn add_assign() {
                let mut a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Add::add);
                a += b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn add_scalar_rhs() {
                let a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Add::add);
                assert_biteq!(a + b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn add_scalar_lhs() {
                let a = 5.;
                let b = from_slice(&B);
                let expected = apply_binary_scalar_lhs_lanewise(a, b, core::ops::Add::add);
                assert_biteq!(a + b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn add_assign_scalar() {
                let mut a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Add::add);
                a += b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn sub() {
                let a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Sub::sub);
                assert_biteq!(a - b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn sub_assign() {
                let mut a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Sub::sub);
                a -= b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn sub_scalar_rhs() {
                let a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Sub::sub);
                assert_biteq!(a - b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn sub_scalar_lhs() {
                let a = 5.;
                let b = from_slice(&B);
                let expected = apply_binary_scalar_lhs_lanewise(a, b, core::ops::Sub::sub);
                assert_biteq!(a - b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn sub_assign_scalar() {
                let mut a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Sub::sub);
                a -= b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn mul() {
                let a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Mul::mul);
                assert_biteq!(a * b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn mul_assign() {
                let mut a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Mul::mul);
                a *= b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn mul_scalar_rhs() {
                let a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Mul::mul);
                assert_biteq!(a * b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn mul_scalar_lhs() {
                let a = 5.;
                let b = from_slice(&B);
                let expected = apply_binary_scalar_lhs_lanewise(a, b, core::ops::Mul::mul);
                assert_biteq!(a * b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn mul_assign_scalar() {
                let mut a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Mul::mul);
                a *= b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn div() {
                let a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Div::div);
                assert_biteq!(a / b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn div_assign() {
                let mut a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Div::div);
                a /= b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn div_scalar_rhs() {
                let a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Div::div);
                assert_biteq!(a / b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn div_scalar_lhs() {
                let a = 5.;
                let b = from_slice(&B);
                let expected = apply_binary_scalar_lhs_lanewise(a, b, core::ops::Div::div);
                assert_biteq!(a / b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn div_assign_scalar() {
                let mut a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Div::div);
                a /= b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn rem() {
                let a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Rem::rem);
                assert_biteq!(a % b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn rem_assign() {
                let mut a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::Rem::rem);
                a %= b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn rem_scalar_rhs() {
                let a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Rem::rem);
                assert_biteq!(a % b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn rem_scalar_lhs() {
                let a = 5.;
                let b = from_slice(&B);
                let expected = apply_binary_scalar_lhs_lanewise(a, b, core::ops::Rem::rem);
                assert_biteq!(a % b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn rem_assign_scalar() {
                let mut a = from_slice(&A);
                let b = 5.;
                let expected = apply_binary_scalar_rhs_lanewise(a, b, core::ops::Rem::rem);
                a %= b;
                assert_biteq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn neg() {
                let v = from_slice(&A);
                let expected = apply_unary_lanewise(v, core::ops::Neg::neg);
                assert_biteq!(-v, expected);
            }
        }
    }
}
