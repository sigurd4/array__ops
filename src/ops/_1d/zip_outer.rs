use core::{ops::AsyncFn, marker::Destruct};

use array_trait::Array;

use crate::{ArrayForm, Runs2D, TryRuns2D};

#[const_trait]
pub trait ZipOuter<T, const N: usize>: Array<Item = T>
{
    fn zip_outer<Z, const M: usize>(&self, other: &Z) -> [[(T, Z::Elem); M]; N]
    where
        T: Copy,
        Z: ArrayForm<M, Elem: Copy>;
    fn zip_outer_ref<Z, const M: usize>(&self, other: &Z) -> [[(&T, Z::Elem); M]; N]
    where
        Z: ArrayForm<M, Elem: Copy>;
}

impl<T, const N: usize> ZipOuter<T, N> for [T; N]
{
    fn zip_outer<Z, const M: usize>(&self, other: &Z) -> [[(T, Z::Elem); M]; N]
    where
        T: Copy,
        Z: ArrayForm<M, Elem: Copy>
    {
        self.zip_outer_with(other, const |x, y| (x, y))
    }
    fn zip_outer_ref<Z, const M: usize>(&self, other: &Z) -> [[(&T, Z::Elem); M]; N]
    where
        Z: ArrayForm<M, Elem: Copy>
    {
        self.zip_outer_ref_with(other, const |x, y| (x, y))
    }
}