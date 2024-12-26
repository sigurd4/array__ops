use core::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

use array_trait::Array;

pub trait ArrayUnsimd<T, const N: usize, const M: usize>: Array<Item = Simd<T, M>>
where
    T: SimdElement,
    LaneCount<M>: SupportedLaneCount
{
    fn array_unsimd(self) -> [T; N*M];
    fn array_unsimd_ref(&self) -> &[T; N*M];
    fn array_unsimd_mut(&mut self) -> &mut [T; N*M];
}

impl<T, const N: usize, const M: usize> ArrayUnsimd<T, N, M> for [Simd<T, M>; N]
where
    T: SimdElement,
    LaneCount<M>: SupportedLaneCount
{
    fn array_unsimd(self) -> [T; N*M]
    {
        let unsimd = unsafe {
            self.as_ptr().cast().read()
        };
        core::mem::forget(self);
        unsimd
    }

    fn array_unsimd_ref(&self) -> &[T; N*M]
    {
        unsafe {
            self.as_ptr().cast().as_ref_unchecked()
        }
    }

    fn array_unsimd_mut(&mut self) -> &mut [T; N*M]
    {
        unsafe {
            self.as_mut_ptr().cast().as_mut_unchecked()
        }
    }
}