use core::ops::Mul;

use array_trait::Array;
use slice_ops::AsSlice;

use crate::form::ArrayForm;

use super::ArrayZipKroneckerWith;

#[const_trait]
pub trait ArrayMulKronecker<T, const M: usize, const N: usize>: Array + AsSlice<Item = [T; N]>
{
    fn mul_kronecker<Rhs, U, const H: usize, const W: usize>(&self, rhs: &Rhs) -> [[<T as Mul<U>>::Output; N*W]; M*H]
    where
        T: Mul<U> + Copy,
        U: Copy,
        Rhs: ArrayForm<H, Elem: ArrayForm<W, Elem = U>>;
}

impl<T, const M: usize, const N: usize> ArrayMulKronecker<T, M, N> for [[T; N]; M]
{
    fn mul_kronecker<Rhs, U, const H: usize, const W: usize>(&self, rhs: &Rhs) -> [[<T as Mul<U>>::Output; N*W]; M*H]
    where
        T: Mul<U> + Copy,
        U: Copy,
        Rhs: ArrayForm<H, Elem: ArrayForm<W, Elem = U>>
    {
        self.zip_kronecker_with(rhs, |x, y| x * y)
    }
}