use crate::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};
use core::fmt;

impl<T, const LANES: usize> fmt::Debug for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    T: SimdElement + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[repr(transparent)]
        struct Wrapper<'a, T: fmt::Debug>(&'a T);

        impl<T: fmt::Debug> fmt::Debug for Wrapper<'_, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        f.debug_list()
            .entries(self.as_array().iter().map(|x| Wrapper(x)))
            .finish()
    }
}
