extern "platform-intrinsic" {
    pub(crate) fn simd_add<T>(x: T, y: T) -> T;
    pub(crate) fn simd_sub<T>(x: T, y: T) -> T;
    pub(crate) fn simd_mul<T>(x: T, y: T) -> T;
    pub(crate) fn simd_div<T>(x: T, y: T) -> T;
    pub(crate) fn simd_rem<T>(x: T, y: T) -> T;
    pub(crate) fn simd_shl<T>(x: T, y: T) -> T;
    pub(crate) fn simd_shr<T>(x: T, y: T) -> T;
    pub(crate) fn simd_and<T>(x: T, y: T) -> T;
    pub(crate) fn simd_or<T>(x: T, y: T) -> T;
    pub(crate) fn simd_xor<T>(x: T, y: T) -> T;
}
