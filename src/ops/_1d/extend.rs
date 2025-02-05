use core::marker::Destruct;

use array_trait::Array;
use slice_ops::AsSlice;

use crate::private;

#[const_trait]
pub trait ArrayExtend<T, const N: usize>: Array + AsSlice<Item = T>
{
    fn extend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;
    fn rextend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;

    fn try_extend<const M: usize, F, E>(self, fill: F) -> Result<[T; M], E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct,
        [(); M - N]:;
    fn try_rextend<const M: usize, F, E>(self, fill: F) -> Result<[T; M], E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct,
        [(); M - N]:;
}

impl<T, const N: usize> ArrayExtend<T, N> for [T; N]
{
    fn extend<const M: usize, F>(self, mut fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T,
        [(); M - N]:
    {
        // TODO add initialize to slice_ops
        /*let mut extended = unsafe {
            private::uninit_extend_transmute(self)
        };
        unsafe {
            extended.assume_init_mut()[N..].initialize(fill)
        }*/
        let filled: [T; M - N] = crate::from_fn(|i| fill(i + N));
        unsafe {private::merge_transmute(self, filled)}
    }
    fn rextend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T,
        [(); M - N]:
    {
        let filled: [T; M - N] = crate::rfrom_fn(fill);
        unsafe {private::merge_transmute(filled, self)}
    }

    fn try_extend<const M: usize, F, E>(self, fill: F) -> Result<[T; M], E>
    where
        F: FnMut(usize) -> Result<T, E>,
        [(); M - N]:
    {
        let filled: [T; M - N] = crate::try_from_fn(fill)?;
        unsafe {private::merge_transmute(filled, self)}
    }
    fn try_rextend<const M: usize, F, E>(self, fill: F) -> Result<[T; M], E>
    where
        F: FnMut(usize) -> Result<T, E>,
        [(); M - N]:
    {
        let filled: [T; M - N] = crate::try_rfrom_fn(fill)?;
        unsafe {private::merge_transmute(filled, self)}
    }
}