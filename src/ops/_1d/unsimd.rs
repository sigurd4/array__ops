use core::{pin::Pin, simd::{LaneCount, Simd, SimdElement, SupportedLaneCount}};

use array_trait::Array;

pub trait ArrayUnsimd<T, const N: usize, const M: usize>: Array<Item = Simd<T, M>>
where
    T: SimdElement,
    LaneCount<M>: SupportedLaneCount
{
    fn unsimd(self) -> [T; N*M];
    fn unsimd_ref(&self) -> &[T; N*M];
    fn unsimd_mut(&mut self) -> &mut [T; N*M];
    fn unsimd_pin_ref(self: Pin<&Self>) -> Pin<&[T; N*M]>;
    fn unsimd_pin_mut(self: Pin<&mut Self>) -> Pin<&mut [T; N*M]>;
}

impl<T, const N: usize, const M: usize> ArrayUnsimd<T, N, M> for [Simd<T, M>; N]
where
    T: SimdElement,
    LaneCount<M>: SupportedLaneCount
{
    fn unsimd(self) -> [T; N*M]
    {
        let unsimd = unsafe {
            self.as_ptr().cast::<[T; N*M]>().read()
        };
        core::mem::forget(self);
        unsimd
    }
    fn unsimd_ref(&self) -> &[T; N*M]
    {
        unsafe {
            self.as_ptr().cast::<[T; N*M]>().as_ref_unchecked()
        }
    }
    fn unsimd_mut(&mut self) -> &mut [T; N*M]
    {
        unsafe {
            self.as_mut_ptr().cast::<[T; N*M]>().as_mut_unchecked()
        }
    }
    fn unsimd_pin_ref(self: Pin<&Self>) -> Pin<&[T; N*M]>
    {
        unsafe {
            self.map_unchecked(|pin| pin.unsimd_ref())
        }
    }
    fn unsimd_pin_mut(self: Pin<&mut Self>) -> Pin<&mut [T; N*M]>
    {
        unsafe {
            self.map_unchecked_mut(|pin| pin.unsimd_mut())
        }
    }
}