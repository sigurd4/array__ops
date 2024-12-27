use core::ops::Bound;

use array_trait::Array;

#[const_trait]
pub trait Isolate<T, const N: usize>: Array<Item = T>
{
    fn isolate(self, i: usize) -> T;
}

impl<T, const N: usize> Isolate<T, N> for [T; N]
{
    fn isolate(mut self, i: usize) -> T
    {
        assert!(i < N, "Index is out of bounds");
        let value = unsafe {
            core::ptr::drop_in_place(&mut self[..i]);
            core::ptr::drop_in_place(&mut self[(Bound::Excluded(i), Bound::Unbounded)]);
            core::ptr::read(&self[i])
        };
        core::mem::forget(self);
        value
    }
}