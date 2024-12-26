use core::{marker::Destruct, mem::MaybeUninit};

use array_trait::Array;

#[const_trait]
pub trait Truncate<T, const N: usize>: Array<Item = T>
{
    fn truncate<const M: usize>(self) -> [T; M]
    where
        T: ~const Destruct,
        [(); N - M]:;
    fn rtruncate<const M: usize>(self) -> [T; M]
    where
        T: ~const Destruct,
        [(); N - M]:;
    fn try_truncate<const M: usize>(self) -> Option<[T; M]>
    where
        T: ~const Destruct;
    fn try_rtruncate<const M: usize>(self) -> Option<[T; M]>
    where
        T: ~const Destruct;
        
    fn truncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:;
    fn rtruncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:;
    fn try_truncate_ref<const M: usize>(&self) -> Option<&[T; M]>
    where
        T: ~const Destruct;
    fn try_rtruncate_ref<const M: usize>(&self) -> Option<&[T; M]>
    where
        T: ~const Destruct;
        
    fn truncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:;
    fn rtruncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:;
    fn try_truncate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>
    where
        T: ~const Destruct;
    fn try_rtruncate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>
    where
        T: ~const Destruct;
}

impl<T, const N: usize> /*const*/ Truncate<T, N> for [T; N]
{
    fn truncate<const M: usize>(self) -> [T; M]
    where
        T: Destruct,
        [(); N - M]:
    {
        self.try_truncate().unwrap()
    }
    fn rtruncate<const M: usize>(self) -> [T; M]
    where
        T: Destruct,
        [(); N - M]:
    {
        self.try_rtruncate().unwrap()
    }
    fn try_truncate<const M: usize>(mut self) -> Option<[T; M]>
    {
        if M > N
        {
            return None
        }
        if M < N
        {
            unsafe {
                core::ptr::drop_in_place(&mut self[M..N]);
            }
        }
        let mut trunc = MaybeUninit::uninit_array();
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.as_ptr(),
                trunc.as_mut_ptr().cast(),
                M
            );
        }
        core::mem::forget(self);
        unsafe {
            Some(MaybeUninit::array_assume_init(trunc))
        }
    }
    fn try_rtruncate<const M: usize>(mut self) -> Option<[T; M]>
    {
        if M > N
        {
            return None
        }
        if M < N
        {
            unsafe {
                core::ptr::drop_in_place(&mut self[0..N - M]);
            }
        }
        let mut trunc = MaybeUninit::uninit_array();
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.as_ptr().add(N - M),
                trunc.as_mut_ptr().cast(),
                M
            );
        }
        core::mem::forget(self);
        unsafe {
            Some(MaybeUninit::array_assume_init(trunc))
        }
    }
    
    fn truncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:
    {
        self.try_truncate_ref().unwrap()
    }
    fn rtruncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:
    {
        self.try_rtruncate_ref().unwrap()
    }
    fn try_truncate_ref<const M: usize>(&self) -> Option<&[T; M]>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(self.as_ptr().cast::<[T; M]>().as_ref_unchecked())
        }
    }
    fn try_rtruncate_ref<const M: usize>(&self) -> Option<&[T; M]>
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
        self.try_truncate_mut().unwrap()
    }
    fn rtruncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:
    {
        self.try_rtruncate_mut().unwrap()
    }
    fn try_truncate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(self.as_mut_ptr().cast::<[T; M]>().as_mut_unchecked())
        }
    }
    fn try_rtruncate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>
    {
        if M > N
        {
            return None
        }
        unsafe {
            Some(self.as_mut_ptr().add(N - M).cast::<[T; M]>().as_mut_unchecked())
        }
    }
}