use core::{marker::Destruct, mem::MaybeUninit};

use super::{ArrayExtend, Truncate};

#[const_trait]
pub trait ArrayResize<T, const N: usize>: Truncate<T, N> + ArrayExtend<T, N>
{
    fn resize<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct;
    fn rresize<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct;
}

impl<T, const N: usize> ArrayResize<T, N> for [T; N]
{
    fn resize<const M: usize, F>(mut self, mut fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + Destruct,
        T: Destruct
    {
        let overlap = N.min(M);

        if M < N
        {
            // Drop truncated elements
            unsafe {
                core::ptr::drop_in_place(&mut self[M..N]);
            }
        }

        let src = self.as_ptr();

        if M <= N
        {
            // If not larger than original, dont make a new uninit, instead read directly from original
            let array = unsafe {
                core::ptr::read(src.cast())
            };
            core::mem::forget(self);
            return array;
        }
    
        // Make new uninit array
        let mut array = MaybeUninit::uninit_array();
        let mut dst = (&mut array as *mut MaybeUninit<T>).cast::<T>();
    
        // Copy over
        unsafe {core::ptr::copy_nonoverlapping(src, dst, overlap)};
        core::mem::forget(self);
    
        // Extend with fill
        let mut i = N;
        dst = unsafe {dst.add(N)};
        while i < M
        {
            unsafe {core::ptr::write(dst, fill(i))};
            i += 1;
            dst = unsafe {dst.add(1)};
        }
        unsafe {
            MaybeUninit::array_assume_init(array)
        }
    }
    fn rresize<const M: usize, F>(mut self, mut fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + Destruct,
        T: Destruct
    {
        let trunc = N.saturating_sub(M);
        let offset = M.saturating_sub(N);
        let overlap = N.min(M);

        if M < N
        {
            // Drop truncated elements
            unsafe {
                core::ptr::drop_in_place(&mut self[0..trunc]);
            }
        }

        let src = unsafe {
            self.as_ptr().add(trunc)
        };

        if M <= N
        {
            // If not larger than original, dont make a new uninit, instead read directly from original
            let array = unsafe {
                core::ptr::read(src.cast())
            };
            core::mem::forget(self);
            return array;
        }
        
        // Make new uninit array
        let mut array = MaybeUninit::uninit_array();
        let mut dst = unsafe {
            (&mut array as *mut MaybeUninit<T>).cast::<T>().add(offset)
        };
    
        // Copy over
        unsafe {core::ptr::copy_nonoverlapping(src, dst, overlap)};
        core::mem::forget(self);
    
        // Extend with fill
        let mut i = offset;
        while i > 0
        {
            i -= 1;
            dst = unsafe {dst.sub(1)};
            unsafe {core::ptr::write(dst, fill(i))};
        }
    
        unsafe {
            MaybeUninit::array_assume_init(array)
        }
    }
}