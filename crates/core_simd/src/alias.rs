macro_rules! number {
    { 1 } => { "one" };
    { 2 } => { "two" };
    { 4 } => { "four" };
    { 8 } => { "eight" };
    { $x:literal } => { stringify!($x) };
}

macro_rules! plural {
    { 1 } => { "" };
    { $x:literal } => { "s" };
}

macro_rules! alias {
    {
        $(
            $element_ty:ty = {
                $($alias:ident $num_elements:tt)*
            }
        )*
    } => {
        $(
            $(
            #[doc = concat!("A SIMD vector with ", number!($num_elements), " element", plural!($num_elements), " of type [`", stringify!($element_ty), "`].")]
            #[allow(non_camel_case_types)]
            pub type $alias = $crate::simd::Simd<$element_ty, $num_elements>;
            )*
        )*
    }
}

macro_rules! mask_alias {
    {
        $(
            $element_ty:ty : $size:literal = {
                $($alias:ident $num_elements:tt)*
            }
        )*
    } => {
        $(
            $(
            #[doc = concat!("A SIMD mask with ", number!($num_elements), " element", plural!($num_elements), " for vectors with ", $size, " element types.")]
            ///
            #[doc = concat!(
                "The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to `[",
                stringify!($element_ty), "; ", $num_elements, "]`."
            )]
            #[allow(non_camel_case_types)]
            pub type $alias = $crate::simd::Mask<$element_ty, $num_elements>;
            )*
        )*
    }
}

alias! {
    i8 = {
        i8x1 1
        i8x2 2
        i8x4 4
        i8x8 8
        i8x16 16
        i8x32 32
        i8x64 64
    }

    i16 = {
        i16x1 1
        i16x2 2
        i16x4 4
        i16x8 8
        i16x16 16
        i16x32 32
        i16x64 64
    }

    i32 = {
        i32x1 1
        i32x2 2
        i32x4 4
        i32x8 8
        i32x16 16
        i32x32 32
        i32x64 64
    }

    i64 = {
        i64x1 1
        i64x2 2
        i64x4 4
        i64x8 8
        i64x16 16
        i64x32 32
        i64x64 64
    }

    isize = {
        isizex1 1
        isizex2 2
        isizex4 4
        isizex8 8
        isizex16 16
        isizex32 32
        isizex64 64
    }

    u8 = {
        u8x1 1
        u8x2 2
        u8x4 4
        u8x8 8
        u8x16 16
        u8x32 32
        u8x64 64
    }

    u16 = {
        u16x1 1
        u16x2 2
        u16x4 4
        u16x8 8
        u16x16 16
        u16x32 32
        u16x64 64
    }

    u32 = {
        u32x1 1
        u32x2 2
        u32x4 4
        u32x8 8
        u32x16 16
        u32x32 32
        u32x64 64
    }

    u64 = {
        u64x1 1
        u64x2 2
        u64x4 4
        u64x8 8
        u64x16 16
        u64x32 32
        u64x64 64
    }

    usize = {
        usizex1 1
        usizex2 2
        usizex4 4
        usizex8 8
        usizex16 16
        usizex32 32
        usizex64 64
    }

    f32 = {
        f32x1 1
        f32x2 2
        f32x4 4
        f32x8 8
        f32x16 16
        f32x32 32
        f32x64 64
    }

    f64 = {
        f64x1 1
        f64x2 2
        f64x4 4
        f64x8 8
        f64x16 16
        f64x32 32
        f64x64 64
    }
}

mask_alias! {
    i8 : "8-bit" = {
        mask8x1 1
        mask8x2 2
        mask8x4 4
        mask8x8 8
        mask8x16 16
        mask8x32 32
        mask8x64 64
    }

    i16 : "16-bit" = {
        mask16x1 1
        mask16x2 2
        mask16x4 4
        mask16x8 8
        mask16x16 16
        mask16x32 32
        mask16x64 64
    }

    i32 : "32-bit" = {
        mask32x1 1
        mask32x2 2
        mask32x4 4
        mask32x8 8
        mask32x16 16
        mask32x32 32
        mask32x64 64
    }

    i64 : "64-bit" = {
        mask64x1 1
        mask64x2 2
        mask64x4 4
        mask64x8 8
        mask64x16 16
        mask64x32 32
        mask64x64 64
    }

    isize : "pointer-sized" = {
        masksizex1 1
        masksizex2 2
        masksizex4 4
        masksizex8 8
        masksizex16 16
        masksizex32 32
        masksizex64 64
    }
}

// Generic SIMD type aliases for writing code generic over lane count.
//
// Use these when writing functions that work with any lane count N (1-64).
// Example: fn dot_product<const N: usize>(a: f32xN<N>, b: f32xN<N>) -> f32

/// Generic `Simd<i8, N>` vector type.
#[allow(non_camel_case_types)]
pub type i8xN<const N: usize> = crate::simd::Simd<i8, N>;

/// Generic `Simd<i16, N>` vector type.
#[allow(non_camel_case_types)]
pub type i16xN<const N: usize> = crate::simd::Simd<i16, N>;

/// Generic `Simd<i32, N>` vector type.
#[allow(non_camel_case_types)]
pub type i32xN<const N: usize> = crate::simd::Simd<i32, N>;

/// Generic `Simd<i64, N>` vector type.
#[allow(non_camel_case_types)]
pub type i64xN<const N: usize> = crate::simd::Simd<i64, N>;

/// Generic `Simd<isize, N>` vector type.
#[allow(non_camel_case_types)]
pub type isizexN<const N: usize> = crate::simd::Simd<isize, N>;

/// Generic `Simd<u8, N>` vector type.
#[allow(non_camel_case_types)]
pub type u8xN<const N: usize> = crate::simd::Simd<u8, N>;

/// Generic `Simd<u16, N>` vector type.
#[allow(non_camel_case_types)]
pub type u16xN<const N: usize> = crate::simd::Simd<u16, N>;

/// Generic `Simd<u32, N>` vector type.
#[allow(non_camel_case_types)]
pub type u32xN<const N: usize> = crate::simd::Simd<u32, N>;

/// Generic `Simd<u64, N>` vector type.
#[allow(non_camel_case_types)]
pub type u64xN<const N: usize> = crate::simd::Simd<u64, N>;

/// Generic `Simd<usize, N>` vector type.
#[allow(non_camel_case_types)]
pub type usizexN<const N: usize> = crate::simd::Simd<usize, N>;

/// Generic `Simd<f32, N>` vector type.
#[allow(non_camel_case_types)]
pub type f32xN<const N: usize> = crate::simd::Simd<f32, N>;

/// Generic `Simd<f64, N>` vector type.
#[allow(non_camel_case_types)]
pub type f64xN<const N: usize> = crate::simd::Simd<f64, N>;

// Generic mask type aliases

/// Generic `Mask<i8, N>` mask type for 8-bit lanes.
#[allow(non_camel_case_types)]
pub type mask8xN<const N: usize> = crate::simd::Mask<i8, N>;

/// Generic `Mask<i16, N>` mask type for 16-bit lanes.
#[allow(non_camel_case_types)]
pub type mask16xN<const N: usize> = crate::simd::Mask<i16, N>;

/// Generic `Mask<i32, N>` mask type for 32-bit lanes.
#[allow(non_camel_case_types)]
pub type mask32xN<const N: usize> = crate::simd::Mask<i32, N>;

/// Generic `Mask<i64, N>` mask type for 64-bit lanes.
#[allow(non_camel_case_types)]
pub type mask64xN<const N: usize> = crate::simd::Mask<i64, N>;

/// Generic `Mask<isize, N>` mask type for pointer-sized lanes.
#[allow(non_camel_case_types)]
pub type masksizexN<const N: usize> = crate::simd::Mask<isize, N>;
