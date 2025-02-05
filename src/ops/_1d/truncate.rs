use core::{marker::Destruct, pin::Pin};

use array_trait::Array;
use slice_ops::AsSlice;

#[const_trait]
pub trait Truncate<T, const N: usize>: Array + AsSlice<Item = T>
{
    fn truncate<const M: usize>(self) -> [T; M]
    where
        T: ~const Destruct,
        [(); N - M]:;
    fn rtruncate<const M: usize>(self) -> [T; M]
    where
        T: ~const Destruct,
        [(); N - M]:;
    fn partial_truncate<const M: usize>(self) -> Option<[T; M]>
    where
        T: ~const Destruct;
    fn partial_rtruncate<const M: usize>(self) -> Option<[T; M]>
    where
        T: ~const Destruct;
        
    fn truncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:;
    fn rtruncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:;
    fn partial_truncate_ref<const M: usize>(&self) -> Option<&[T; M]>;
    fn partial_rtruncate_ref<const M: usize>(&self) -> Option<&[T; M]>;
        
    fn truncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:;
    fn rtruncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:;
    fn partial_truncate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>;
    fn partial_rtruncate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>;

    fn truncate_pin_ref<const M: usize>(self: Pin<&Self>) -> Pin<&[T; M]>
    where
        [(); N - M]:;
    fn rtruncate_pin_ref<const M: usize>(self: Pin<&Self>) -> Pin<&[T; M]>
    where
        [(); N - M]:;
    fn partial_truncate_pin_ref<const M: usize>(self: Pin<&Self>) -> Option<Pin<&[T; M]>>;
    fn partial_rtruncate_pin_ref<const M: usize>(self: Pin<&Self>) -> Option<Pin<&[T; M]>>;

    fn truncate_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Pin<&mut [T; M]>
    where
        [(); N - M]:;
    fn rtruncate_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Pin<&mut [T; M]>
    where
        [(); N - M]:;
    fn partial_truncate_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Option<Pin<&mut [T; M]>>;
    fn partial_rtruncate_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Option<Pin<&mut [T; M]>>;
}

impl<T, const N: usize> /*const*/ Truncate<T, N> for [T; N]
{
    fn truncate<const M: usize>(self) -> [T; M]
    where
        T: Destruct,
        [(); N - M]:
    {
        unsafe {
            self.partial_truncate().unwrap_unchecked()
        }
    }
    fn rtruncate<const M: usize>(self) -> [T; M]
    where
        T: Destruct,
        [(); N - M]:
    {
        unsafe {
            self.partial_rtruncate().unwrap_unchecked()
        }
    }
    fn partial_truncate<const M: usize>(mut self) -> Option<[T; M]>
    {
        if M > N
        {
            return None
        }
        if M < N && const {core::mem::needs_drop::<T>()}
        {
            unsafe {
                core::ptr::drop_in_place(&mut self[M..N]);
            }
        }
        let trunc = unsafe {
            self.as_ptr().cast::<[T; M]>().read()
        };
        core::mem::forget(self);
        Some(trunc)
    }
    fn partial_rtruncate<const M: usize>(mut self) -> Option<[T; M]>
    {
        if M > N
        {
            return None
        }
        if M < N && const {core::mem::needs_drop::<T>()}
        {
            unsafe {
                core::ptr::drop_in_place(&mut self[0..N - M]);
            }
        }
        let trunc = unsafe {
            self.as_ptr().add(N - M).cast::<[T; M]>().read()
        };
        core::mem::forget(self);
        Some(trunc)
    }
    
    fn truncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:
    {
        unsafe {
            self.partial_truncate_ref().unwrap_unchecked()
        }
    }
    fn rtruncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:
    {
        unsafe {
            self.partial_rtruncate_ref().unwrap_unchecked()
        }
    }
    fn partial_truncate_ref<const M: usize>(&self) -> Option<&[T; M]>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(self.as_ptr().cast::<[T; M]>().as_ref_unchecked())
        }
    }
    fn partial_rtruncate_ref<const M: usize>(&self) -> Option<&[T; M]>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(self.as_ptr().add(N - M).cast::<[T; M]>().as_ref_unchecked())
        }
    }
        
    fn truncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:
    {
        unsafe {
            self.partial_truncate_mut().unwrap_unchecked()
        }
    }
    fn rtruncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:
    {
        unsafe {
            self.partial_rtruncate_mut().unwrap_unchecked()
        }
    }
    fn partial_truncate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(self.as_mut_ptr().cast::<[T; M]>().as_mut_unchecked())
        }
    }
    fn partial_rtruncate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(self.as_mut_ptr().add(N - M).cast::<[T; M]>().as_mut_unchecked())
        }
    }

    fn truncate_pin_ref<const M: usize>(self: Pin<&Self>) -> Pin<&[T; M]>
    where
        [(); N - M]:
    {
        unsafe {
            Pin::new_unchecked(self.get_ref().truncate_ref())
        }
    }
    fn rtruncate_pin_ref<const M: usize>(self: Pin<&Self>) -> Pin<&[T; M]>
    where
        [(); N - M]:
    {
        unsafe {
            Pin::new_unchecked(self.get_ref().rtruncate_ref())
        }
    }
    fn partial_truncate_pin_ref<const M: usize>(self: Pin<&Self>) -> Option<Pin<&[T; M]>>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(Pin::new_unchecked(self.get_ref().partial_truncate_ref().unwrap_unchecked()))
        }
    }
    fn partial_rtruncate_pin_ref<const M: usize>(self: Pin<&Self>) -> Option<Pin<&[T; M]>>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(Pin::new_unchecked(self.get_ref().partial_rtruncate_ref().unwrap_unchecked()))
        }
    }

    fn truncate_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Pin<&mut [T; M]>
    where
        [(); N - M]:
    {
        unsafe {
            Pin::new_unchecked(self.get_unchecked_mut().truncate_mut())
        }
    }
    fn rtruncate_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Pin<&mut [T; M]>
    where
        [(); N - M]:
    {
        unsafe {
            Pin::new_unchecked(self.get_unchecked_mut().rtruncate_mut())
        }
    }
    fn partial_truncate_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Option<Pin<&mut [T; M]>>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(Pin::new_unchecked(self.get_unchecked_mut().partial_truncate_mut().unwrap_unchecked()))
        }
    }
    fn partial_rtruncate_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Option<Pin<&mut [T; M]>>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(Pin::new_unchecked(self.get_unchecked_mut().partial_rtruncate_mut().unwrap_unchecked()))
        }
    }
}