use core::marker::Destruct;

use array_trait::Array;
use slice_ops::AsSlice;

use crate::form::ArrayForm;

use super::ArrayEnumerateZipKroneckerWith;

#[const_trait]
pub trait ArrayZipKroneckerWith<T, const M: usize, const N: usize>: Array + AsSlice<Item = [T; N]>
{
    fn zip_kronecker_with<Rhs, const H: usize, const W: usize, F>(&self, rhs: &Rhs, zipper: F) -> [[F::Output; N*W]; M*H]
    where
        T: Copy,
        Rhs: ArrayForm<H, Elem: ArrayForm<W, Elem: Copy>>,
        F: FnMut<(T, <Rhs::Elem as ArrayForm<W>>::Elem)> + ~const Destruct;
}

impl<T, const M: usize, const N: usize> ArrayZipKroneckerWith<T, M, N> for [[T; N]; M]
{
    fn zip_kronecker_with<Rhs, const H: usize, const W: usize, F>(&self, rhs: &Rhs, mut zipper: F) -> [[F::Output; N*W]; M*H]
    where
        T: Copy,
        Rhs: ArrayForm<H, Elem: ArrayForm<W, Elem: Copy>>,
        F: FnMut<(T, <Rhs::Elem as ArrayForm<W>>::Elem)>
    {
        self.enumerate_zip_kronecker_with(rhs, |_, _, _, _, x, y| zipper(x, y))
    }
}