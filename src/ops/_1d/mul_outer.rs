use core::ops::Mul;

use array_trait::Array;

use crate::form::ArrayForm;

use super::ZipOuterWith;

#[const_trait]
pub trait ArrayMulOuter<T, const N: usize>: Array<Item = T>
{
    fn mul_outer<Rhs, const M: usize>(&self, rhs: &Rhs) -> [[<T as Mul<Rhs::Elem>>::Output; M]; N]
    where
        T: Mul<Rhs::Elem> + Copy,
        Rhs: ArrayForm<M, Elem: Copy>;

    async fn mul_outer_async<Rhs, const M: usize>(&self, rhs: &Rhs) -> [[<T as Mul<Rhs::Elem>>::Output; M]; N]
    where
        T: Mul<Rhs::Elem> + Copy,
        Rhs: ArrayForm<M, Elem: Copy>;
}

impl<T, const N: usize> ArrayMulOuter<T, N> for [T; N]
{
    fn mul_outer<Rhs, const M: usize>(&self, rhs: &Rhs) -> [[<T as Mul<Rhs::Elem>>::Output; M]; N]
    where
        T: Mul<Rhs::Elem> + Copy,
        Rhs: ArrayForm<M, Elem: Copy>
    {
        self.zip_outer_with(rhs, Mul::mul)
    }
    
    async fn mul_outer_async<Rhs, const M: usize>(&self, rhs: &Rhs) -> [[<T as Mul<Rhs::Elem>>::Output; M]; N]
    where
        T: Mul<Rhs::Elem> + Copy,
        Rhs: ArrayForm<M, Elem: Copy>
    {
        self.zip_outer_async_with(rhs, async |x, y| x*y).await
    }
}