use core::mem::MaybeUninit;

use array_trait::Array;

use super::Split;

#[const_trait]
pub trait ArrayRotate<T, const N: usize>: Array<Item = T>
{
    fn into_rotate_left(self, n: usize) -> [T; N];
    fn into_rotate_right(self, n: usize) -> [T; N];

    fn rotate_left(&mut self, n: usize);
    fn rotate_right(&mut self, n: usize);
}

impl<T, const N: usize> const ArrayRotate<T, N> for [T; N]
{
    fn into_rotate_left(self, n: usize) -> [T; N]
    {
        let n = n % N;
        let mut rotated = MaybeUninit::<[T; N]>::uninit();
    
        let (left, right) = slice_ops::split_len(N, n);
        let (src_left, src_right) = self.split_ptr(n);
    
        unsafe {
            let (dst_left, dst_right) = rotated.assume_init_mut().rsplit_mut_ptr(n);
    
            core::ptr::copy_nonoverlapping(src_right, dst_left, right);
            core::ptr::copy_nonoverlapping(src_left, dst_right, left);
        }
    
        core::mem::forget(self);
    
        unsafe {
            MaybeUninit::assume_init(rotated)
        }
    }
    fn into_rotate_right(self, n: usize) -> [T; N]
    {
        let n = n % N;
        let mut rotated = MaybeUninit::<[T; N]>::uninit();
    
        let (left, right) = slice_ops::rsplit_len(N, n);
        let (src_left, src_right) = self.rsplit_ptr(n);
    
        unsafe {
            let (dst_left, dst_right) = rotated.assume_init_mut().split_mut_ptr(n);
    
            core::ptr::copy_nonoverlapping(src_right, dst_left, right);
            core::ptr::copy_nonoverlapping(src_left, dst_right, left);
        }
    
        core::mem::forget(self);
    
        unsafe {
            MaybeUninit::assume_init(rotated)
        }
    }

    fn rotate_left(&mut self, n: usize)
    {
        let n = n % N;
        unsafe {
            let mut buffer: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();
    
            let (left, right) = slice_ops::split_len(N, n);
            let (src_left, src_right) = buffer.split_mut_ptr(n);
            let (dst_left, dst_right) = self.rsplit_mut_ptr(n);
    
            core::ptr::copy_nonoverlapping(
                dst_left,
                src_left.cast(),
                N
            );
            core::ptr::copy_nonoverlapping(
                src_right,
                dst_left.cast(),
                right
            );
            core::ptr::copy_nonoverlapping(
                src_left,
                dst_right.cast(),
                left
            );
            core::mem::forget(buffer);
        }
    }
    fn rotate_right(&mut self, n: usize)
    {
        let n = n % N;
        unsafe {
            let mut buffer: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();
    
            let (left, right) = slice_ops::rsplit_len(N, n);
            let (src_left, src_right) = buffer.rsplit_mut_ptr(n);
            let (dst_left, dst_right) = self.split_mut_ptr(n);
    
            core::ptr::copy_nonoverlapping(
                dst_left,
                src_left.cast(),
                N
            );
            core::ptr::copy_nonoverlapping(
                src_right,
                dst_left.cast(),
                right
            );
            core::ptr::copy_nonoverlapping(
                src_left,
                dst_right.cast(),
                left
            );
            core::mem::forget(buffer);
        }
    }
}