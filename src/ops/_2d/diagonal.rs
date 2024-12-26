use core::marker::Destruct;

use array_trait::Array;

use crate::private::guard::PartialEmptyGuard;

#[const_trait]
pub trait ArrayDiagonal<T, const M: usize, const N: usize>: Array<Item = [T; N]>
{
    fn diagonal(self) -> [T; crate::min_len(M, N)]
    where
        T: ~const Destruct;
    fn diagonal_ref(&self) -> [&T; crate::min_len(M, N)];
    fn diagonal_mut(&mut self) -> [&mut T; crate::min_len(M, N)];
}

impl<T, const M: usize, const N: usize> ArrayDiagonal<T, M, N> for [[T; N]; M]
{
    fn diagonal(self) -> [T; crate::min_len(M, N)]
    {
        let mut guard = PartialEmptyGuard::new(self);
        crate::from_fn(|n| guard.pop()[n])
    }
    fn diagonal_ref(&self) -> [&T; crate::min_len(M, N)]
    {
        crate::from_fn(|n| &self[n][n])
    }
    fn diagonal_mut(&mut self) -> [&mut T; crate::min_len(M, N)]
    {
        crate::from_fn(|n| unsafe {
            (&mut self[n][n] as *mut T).as_mut_unchecked()
        })
    }
}