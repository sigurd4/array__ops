use core::marker::Destruct;

use array_trait::Array;

use crate::private;

#[const_trait]
pub trait Extend<T, const N: usize>: Array<Item = T>
{
    fn extend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;
    fn rextend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;

    // TODO: Needs initialization from SliceOps
    /*fn try_extend<const M: usize, F>(self, fill: F) -> Option<[T; M]>
    where
        F: FnMut(usize) -> T + ~const Destruct;
    fn try_rextend<const M: usize, F>(self, fill: F) -> Option<[T; M]>
    where
        F: FnMut(usize) -> T + ~const Destruct;*/
}

impl<T, const N: usize> Extend<T, N> for [T; N]
{
    fn extend<const M: usize, F>(self, mut fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + Destruct,
        [(); M - N]:
    {
        let filled: [T; M - N] = crate::from_fn(|i| fill(i + N));
        unsafe {private::merge_transmute(self, filled)}
    }
    fn rextend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + Destruct,
        [(); M - N]:
    {
        let filled: [T; M - N] = crate::rfrom_fn(fill);
        unsafe {private::merge_transmute(filled, self)}
    }
}