use array_trait::Array;

use crate::form::ArrayForm;

use super::ArrayZipKroneckerWith;

#[const_trait]
pub trait ArrayZipKronecker<T, const M: usize, const N: usize>: Array<Item = [T; N]>
{
    fn zip_kronecker<Rhs, const H: usize, const W: usize>(&self, rhs: &Rhs) -> [[(T, <Rhs::Elem as ArrayForm<W>>::Elem); N*W]; M*H]
    where
        T: Copy,
        Rhs: ArrayForm<H, Elem: ArrayForm<W, Elem: Copy>>;
}

impl<T, const M: usize, const N: usize> ArrayZipKronecker<T, M, N> for [[T; N]; M]
{
    fn zip_kronecker<Rhs, const H: usize, const W: usize>(&self, rhs: &Rhs) -> [[(T, <Rhs::Elem as ArrayForm<W>>::Elem); N*W]; M*H]
    where
        T: Copy,
        Rhs: ArrayForm<H, Elem: ArrayForm<W, Elem: Copy>>
    {
        self.zip_kronecker_with(rhs, |x, y| (x, y))
    }
}