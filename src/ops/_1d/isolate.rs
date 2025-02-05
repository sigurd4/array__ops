use core::ops::Bound;

use array_trait::Array;
use slice_ops::AsSlice;

#[const_trait]
pub trait ArrayIsolate<T, const N: usize>: Array + AsSlice<Item = T>
{
    fn isolate(self, i: usize) -> Option<T>;
}

impl<T, const N: usize> ArrayIsolate<T, N> for [T; N]
{
    fn isolate(mut self, i: usize) -> Option<T>
    {
        if i >= N
        {
            return None
        }
        
        let value = unsafe {
            if core::mem::needs_drop::<T>()
            {
                core::ptr::drop_in_place(&mut self[0..i]);
                core::ptr::drop_in_place(&mut self[(Bound::Excluded(i), Bound::Excluded(N))]);
            }
            core::ptr::read(&self[i])
        };
        core::mem::forget(self);
        Some(value)
    }
}