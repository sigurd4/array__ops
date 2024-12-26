use core::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

use array_trait::Array;

use super::Split;

#[const_trait]
pub trait ArraySimd<T, const N: usize>: Array<Item = T>
{
    fn array_simd<const M: usize>(self) -> ([Simd<T, M>; N / M], [T; N % M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    fn array_simd_ref<const M: usize>(&self) -> (&[Simd<T, M>; N / M], &[T; N % M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    fn array_simd_mut<const M: usize>(&mut self) -> (&mut [Simd<T, M>; N / M], &mut [T; N % M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    
    fn array_rsimd<const M: usize>(self) -> ([T; N % M], [Simd<T, M>; N / M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    fn array_rsimd_ref<const M: usize>(&self) -> (&[T; N % M], &[Simd<T, M>; N / M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    fn array_rsimd_mut<const M: usize>(&mut self) -> (&mut [T; N % M], &mut [Simd<T, M>; N / M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    
    fn array_simd_exact<const M: usize>(self) -> [Simd<T, M>; N / M]
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:;
    fn array_simd_exact_ref<const M: usize>(&self) -> &[Simd<T, M>; N / M]
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:;
    fn array_simd_exact_mut<const M: usize>(&mut self) -> &mut [Simd<T, M>; N / M]
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:;
}

impl<T, const N: usize> const ArraySimd<T, N> for [T; N]
{
    fn array_simd<const M: usize>(self) -> ([Simd<T, M>; N / M], [T; N % M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        // transmute?

        let (left, right) = self.rsplit_ptr(N % M);
        let simd = unsafe {
            left.cast().read()
        };
        let rest = unsafe {
            right.cast().read()
        };
        core::mem::forget(self);
        (simd, rest)
    }
    fn array_simd_ref<const M: usize>(&self) -> (&[Simd<T, M>; N / M], &[T; N % M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        let (left, right) = self.rsplit_ptr(N % M);
        let simd = unsafe {
            left.cast().as_ref_unchecked()
        };
        let rest = unsafe {
            right.cast().as_ref_unchecked()
        };
        (simd, rest)
    }
    fn array_simd_mut<const M: usize>(&mut self) -> (&mut [Simd<T, M>; N / M], &mut [T; N % M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        let (left, right) = self.rsplit_mut_ptr(N % M);
        let simd = unsafe {
            left.cast().as_mut_unchecked()
        };
        let rest = unsafe {
            right.cast().as_mut_unchecked()
        };
        (simd, rest)
    }
    
    fn array_rsimd<const M: usize>(self) -> ([T; N % M], [Simd<T, M>; N / M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        // transmute?

        let (left, right) = self.split_ptr(N % M);
        let rest = unsafe {
            left.cast().read()
        };
        let simd = unsafe {
            right.cast().read()
        };
        core::mem::forget(self);
        (rest, simd)
    }
    fn array_rsimd_ref<const M: usize>(&self) -> (&[T; N % M], &[Simd<T, M>; N / M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        let (left, right) = self.split_ptr(N % M);
        let rest = unsafe {
            left.cast().as_ref_unchecked()
        };
        let simd = unsafe {
            right.cast().as_ref_unchecked()
        };
        (rest, simd)
    }
    fn array_rsimd_mut<const M: usize>(&mut self) -> (&mut [T; N % M], &mut [Simd<T, M>; N / M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        let (left, right) = self.split_mut_ptr(N % M);
        let rest = unsafe {
            left.cast().as_mut_unchecked()
        };
        let simd = unsafe {
            right.cast().as_mut_unchecked()
        };
        (rest, simd)
    }
    
    fn array_simd_exact<const M: usize>(self) -> [Simd<T, M>; N / M]
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        // transmute?

        let simd = unsafe {
            self.as_ptr().cast().read()
        };
        core::mem::forget(self);
        simd
    }
    fn array_simd_exact_ref<const M: usize>(&self) -> &[Simd<T, M>; N / M]
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            self.as_ptr().cast().as_ref_unchecked()
        }
    }
    fn array_simd_exact_mut<const M: usize>(&mut self) -> &mut [Simd<T, M>; N / M]
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            self.as_mut_ptr().cast().as_mut_unchecked()
        }
    }
}