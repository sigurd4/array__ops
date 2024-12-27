use core::{pin::Pin, simd::{LaneCount, Simd, SimdElement, SupportedLaneCount}};

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
    fn array_simd_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[Simd<T, M>; N / M]>, Pin<&[T; N % M]>)
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    fn array_simd_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [Simd<T, M>; N / M]>, Pin<&mut [T; N % M]>)
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
    fn array_rsimd_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[T; N % M]>, Pin<&[Simd<T, M>; N / M]>)
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    fn array_rsimd_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [T; N % M]>, Pin<&mut [Simd<T, M>; N / M]>)
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
    fn array_simd_exact_pin_ref<const M: usize>(self: Pin<&Self>) -> Pin<&[Simd<T, M>; N / M]>
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:;
    fn array_simd_exact_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Pin<&mut [Simd<T, M>; N / M]>
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
            left.cast::<[Simd<T, M>; N / M]>().read()
        };
        let rest = unsafe {
            right.cast::<[T; N % M]>().read()
        };
        #[allow(forgetting_copy_types)]
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
            left.cast::<[Simd<T, M>; N / M]>().as_ref_unchecked()
        };
        let rest = unsafe {
            right.cast::<[T; N % M]>().as_ref_unchecked()
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
            left.cast::<[Simd<T, M>; N / M]>().as_mut_unchecked()
        };
        let rest = unsafe {
            right.cast::<[T; N % M]>().as_mut_unchecked()
        };
        (simd, rest)
    }
    fn array_simd_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[Simd<T, M>; N / M]>, Pin<&[T; N % M]>)
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        let (left, right) = self.get_ref().array_simd_ref();
        unsafe {(
            Pin::new_unchecked(left),
            Pin::new_unchecked(right)
        )}
    }
    fn array_simd_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [Simd<T, M>; N / M]>, Pin<&mut [T; N % M]>)
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        let (left, right) = unsafe {
            self.get_unchecked_mut().array_simd_mut()
        };
        unsafe {(
            Pin::new_unchecked(left),
            Pin::new_unchecked(right)
        )}
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
            left.cast::<[T; N % M]>().read()
        };
        let simd = unsafe {
            right.cast::<[Simd<T, M>; N / M]>().read()
        };
        #[allow(forgetting_copy_types)]
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
            left.cast::<[T; N % M]>().as_ref_unchecked()
        };
        let simd = unsafe {
            right.cast::<[Simd<T, M>; N / M]>().as_ref_unchecked()
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
            left.cast::<[T; N % M]>().as_mut_unchecked()
        };
        let simd = unsafe {
            right.cast::<[Simd<T, M>; N / M]>().as_mut_unchecked()
        };
        (rest, simd)
    }
    fn array_rsimd_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[T; N % M]>, Pin<&[Simd<T, M>; N / M]>)
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        let (left, right) = self.get_ref().array_rsimd_ref();
        unsafe {(
            Pin::new_unchecked(left),
            Pin::new_unchecked(right)
        )}
    }
    fn array_rsimd_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [T; N % M]>, Pin<&mut [Simd<T, M>; N / M]>)
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        let (left, right) = unsafe {
            self.get_unchecked_mut().array_rsimd_mut()
        };
        unsafe {(
            Pin::new_unchecked(left),
            Pin::new_unchecked(right)
        )}
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
            self.as_ptr().cast::<[Simd<T, M>; N / M]>().read()
        };
        #[allow(forgetting_copy_types)]
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
            self.as_ptr().cast::<[Simd<T, M>; N / M]>().as_ref_unchecked()
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
            self.as_mut_ptr().cast::<[Simd<T, M>; N / M]>().as_mut_unchecked()
        }
    }
    fn array_simd_exact_pin_ref<const M: usize>(self: Pin<&Self>) -> Pin<&[Simd<T, M>; N / M]>
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            Pin::new_unchecked(self.get_ref().array_simd_exact_ref())
        }
    }
    fn array_simd_exact_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Pin<&mut [Simd<T, M>; N / M]>
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            Pin::new_unchecked(self.get_unchecked_mut().array_simd_exact_mut())
        }
    }
}