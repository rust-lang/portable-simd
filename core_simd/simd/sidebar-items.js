window.SIDEBAR_ITEMS = {"enum":[["Which","Specifies a lane index into one of two SIMD vectors."]],"macro":[["simd_swizzle","Constructs a new SIMD vector by copying elements from selected lanes in other vectors."]],"struct":[["LaneCount","Specifies the number of lanes in a SIMD vector as a type."],["Mask","A SIMD vector mask for `LANES` elements of width specified by `Element`."],["Simd","A SIMD vector of `LANES` elements of type `T`. `Simd<T, N>` has the same shape as `[T; N]`, but operates like `T`."]],"trait":[["MaskElement","Marker trait for types that may be used as SIMD mask elements."],["SimdCast","Supporting trait for `Simd::cast`.  Typically doesn’t need to be used directly."],["SimdCastPtr","Supporting trait for `Simd::cast_ptr`.  Typically doesn’t need to be used directly."],["SimdConstPtr","Operations on SIMD vectors of constant pointers."],["SimdElement","Marker trait for types that may be used as SIMD vector elements."],["SimdFloat","Operations on SIMD vectors of floats."],["SimdInt","Operations on SIMD vectors of signed integers."],["SimdMutPtr","Operations on SIMD vectors of mutable pointers."],["SimdOrd","Parallel `Ord`."],["SimdPartialEq","Parallel `PartialEq`."],["SimdPartialOrd","Parallel `PartialOrd`."],["SimdUint","Operations on SIMD vectors of unsigned integers."],["SupportedLaneCount","Statically guarantees that a lane count is marked as supported."],["Swizzle","Create a vector from the elements of another vector."],["Swizzle2","Create a vector from the elements of two other vectors."],["ToBitMask","Converts masks to and from integer bitmasks."]],"type":[["f32x1","A SIMD vector with one element of type [`f32`]."],["f32x16","A SIMD vector with 16 elements of type [`f32`]."],["f32x2","A SIMD vector with two elements of type [`f32`]."],["f32x32","A SIMD vector with 32 elements of type [`f32`]."],["f32x4","A SIMD vector with four elements of type [`f32`]."],["f32x64","A SIMD vector with 64 elements of type [`f32`]."],["f32x8","A SIMD vector with eight elements of type [`f32`]."],["f64x1","A SIMD vector with one element of type [`f64`]."],["f64x16","A SIMD vector with 16 elements of type [`f64`]."],["f64x2","A SIMD vector with two elements of type [`f64`]."],["f64x32","A SIMD vector with 32 elements of type [`f64`]."],["f64x4","A SIMD vector with four elements of type [`f64`]."],["f64x64","A SIMD vector with 64 elements of type [`f64`]."],["f64x8","A SIMD vector with eight elements of type [`f64`]."],["i16x1","A SIMD vector with one element of type [`i16`]."],["i16x16","A SIMD vector with 16 elements of type [`i16`]."],["i16x2","A SIMD vector with two elements of type [`i16`]."],["i16x32","A SIMD vector with 32 elements of type [`i16`]."],["i16x4","A SIMD vector with four elements of type [`i16`]."],["i16x64","A SIMD vector with 64 elements of type [`i16`]."],["i16x8","A SIMD vector with eight elements of type [`i16`]."],["i32x1","A SIMD vector with one element of type [`i32`]."],["i32x16","A SIMD vector with 16 elements of type [`i32`]."],["i32x2","A SIMD vector with two elements of type [`i32`]."],["i32x32","A SIMD vector with 32 elements of type [`i32`]."],["i32x4","A SIMD vector with four elements of type [`i32`]."],["i32x64","A SIMD vector with 64 elements of type [`i32`]."],["i32x8","A SIMD vector with eight elements of type [`i32`]."],["i64x1","A SIMD vector with one element of type [`i64`]."],["i64x16","A SIMD vector with 16 elements of type [`i64`]."],["i64x2","A SIMD vector with two elements of type [`i64`]."],["i64x32","A SIMD vector with 32 elements of type [`i64`]."],["i64x4","A SIMD vector with four elements of type [`i64`]."],["i64x64","A SIMD vector with 64 elements of type [`i64`]."],["i64x8","A SIMD vector with eight elements of type [`i64`]."],["i8x1","A SIMD vector with one element of type [`i8`]."],["i8x16","A SIMD vector with 16 elements of type [`i8`]."],["i8x2","A SIMD vector with two elements of type [`i8`]."],["i8x32","A SIMD vector with 32 elements of type [`i8`]."],["i8x4","A SIMD vector with four elements of type [`i8`]."],["i8x64","A SIMD vector with 64 elements of type [`i8`]."],["i8x8","A SIMD vector with eight elements of type [`i8`]."],["isizex1","A SIMD vector with one element of type [`isize`]."],["isizex16","A SIMD vector with 16 elements of type [`isize`]."],["isizex2","A SIMD vector with two elements of type [`isize`]."],["isizex32","A SIMD vector with 32 elements of type [`isize`]."],["isizex4","A SIMD vector with four elements of type [`isize`]."],["isizex64","A SIMD vector with 64 elements of type [`isize`]."],["isizex8","A SIMD vector with eight elements of type [`isize`]."],["mask16x1","A SIMD mask with one element for vectors with 16-bit element types."],["mask16x16","A SIMD mask with 16 elements for vectors with 16-bit element types."],["mask16x2","A SIMD mask with two elements for vectors with 16-bit element types."],["mask16x32","A SIMD mask with 32 elements for vectors with 16-bit element types."],["mask16x4","A SIMD mask with four elements for vectors with 16-bit element types."],["mask16x64","A SIMD mask with 64 elements for vectors with 16-bit element types."],["mask16x8","A SIMD mask with eight elements for vectors with 16-bit element types."],["mask32x1","A SIMD mask with one element for vectors with 32-bit element types."],["mask32x16","A SIMD mask with 16 elements for vectors with 32-bit element types."],["mask32x2","A SIMD mask with two elements for vectors with 32-bit element types."],["mask32x32","A SIMD mask with 32 elements for vectors with 32-bit element types."],["mask32x4","A SIMD mask with four elements for vectors with 32-bit element types."],["mask32x64","A SIMD mask with 64 elements for vectors with 32-bit element types."],["mask32x8","A SIMD mask with eight elements for vectors with 32-bit element types."],["mask64x1","A SIMD mask with one element for vectors with 64-bit element types."],["mask64x16","A SIMD mask with 16 elements for vectors with 64-bit element types."],["mask64x2","A SIMD mask with two elements for vectors with 64-bit element types."],["mask64x32","A SIMD mask with 32 elements for vectors with 64-bit element types."],["mask64x4","A SIMD mask with four elements for vectors with 64-bit element types."],["mask64x64","A SIMD mask with 64 elements for vectors with 64-bit element types."],["mask64x8","A SIMD mask with eight elements for vectors with 64-bit element types."],["mask8x1","A SIMD mask with one element for vectors with 8-bit element types."],["mask8x16","A SIMD mask with 16 elements for vectors with 8-bit element types."],["mask8x2","A SIMD mask with two elements for vectors with 8-bit element types."],["mask8x32","A SIMD mask with 32 elements for vectors with 8-bit element types."],["mask8x4","A SIMD mask with four elements for vectors with 8-bit element types."],["mask8x64","A SIMD mask with 64 elements for vectors with 8-bit element types."],["mask8x8","A SIMD mask with eight elements for vectors with 8-bit element types."],["masksizex1","A SIMD mask with one element for vectors with pointer-sized element types."],["masksizex16","A SIMD mask with 16 elements for vectors with pointer-sized element types."],["masksizex2","A SIMD mask with two elements for vectors with pointer-sized element types."],["masksizex32","A SIMD mask with 32 elements for vectors with pointer-sized element types."],["masksizex4","A SIMD mask with four elements for vectors with pointer-sized element types."],["masksizex64","A SIMD mask with 64 elements for vectors with pointer-sized element types."],["masksizex8","A SIMD mask with eight elements for vectors with pointer-sized element types."],["u16x1","A SIMD vector with one element of type [`u16`]."],["u16x16","A SIMD vector with 16 elements of type [`u16`]."],["u16x2","A SIMD vector with two elements of type [`u16`]."],["u16x32","A SIMD vector with 32 elements of type [`u16`]."],["u16x4","A SIMD vector with four elements of type [`u16`]."],["u16x64","A SIMD vector with 64 elements of type [`u16`]."],["u16x8","A SIMD vector with eight elements of type [`u16`]."],["u32x1","A SIMD vector with one element of type [`u32`]."],["u32x16","A SIMD vector with 16 elements of type [`u32`]."],["u32x2","A SIMD vector with two elements of type [`u32`]."],["u32x32","A SIMD vector with 32 elements of type [`u32`]."],["u32x4","A SIMD vector with four elements of type [`u32`]."],["u32x64","A SIMD vector with 64 elements of type [`u32`]."],["u32x8","A SIMD vector with eight elements of type [`u32`]."],["u64x1","A SIMD vector with one element of type [`u64`]."],["u64x16","A SIMD vector with 16 elements of type [`u64`]."],["u64x2","A SIMD vector with two elements of type [`u64`]."],["u64x32","A SIMD vector with 32 elements of type [`u64`]."],["u64x4","A SIMD vector with four elements of type [`u64`]."],["u64x64","A SIMD vector with 64 elements of type [`u64`]."],["u64x8","A SIMD vector with eight elements of type [`u64`]."],["u8x1","A SIMD vector with one element of type [`u8`]."],["u8x16","A SIMD vector with 16 elements of type [`u8`]."],["u8x2","A SIMD vector with two elements of type [`u8`]."],["u8x32","A SIMD vector with 32 elements of type [`u8`]."],["u8x4","A SIMD vector with four elements of type [`u8`]."],["u8x64","A SIMD vector with 64 elements of type [`u8`]."],["u8x8","A SIMD vector with eight elements of type [`u8`]."],["usizex1","A SIMD vector with one element of type [`usize`]."],["usizex16","A SIMD vector with 16 elements of type [`usize`]."],["usizex2","A SIMD vector with two elements of type [`usize`]."],["usizex32","A SIMD vector with 32 elements of type [`usize`]."],["usizex4","A SIMD vector with four elements of type [`usize`]."],["usizex64","A SIMD vector with 64 elements of type [`usize`]."],["usizex8","A SIMD vector with eight elements of type [`usize`]."]]};