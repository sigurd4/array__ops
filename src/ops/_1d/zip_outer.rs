use core::pin::Pin;

use array_trait::Array;

use crate::form::ArrayForm;

use super::ArrayZipOuterWith;

#[const_trait]
pub trait ArrayZipOuter<T, const N: usize>: Array<Item = T>
{
    fn zip_outer<Z, const M: usize>(&self, other: &Z) -> [[(T, Z::Elem); M]; N]
    where
        T: Copy,
        Z: ArrayForm<M, Elem: Copy>;
    fn zip_outer_ref<Z, const M: usize>(&self, other: &Z) -> [[(&T, Z::Elem); M]; N]
    where
        Z: ArrayForm<M, Elem: Copy>;
    fn zip_outer_pin_ref<Z, const M: usize>(self: Pin<&Self>, other: &Z) -> [[(Pin<&T>, Z::Elem); M]; N]
    where
        Z: ArrayForm<M, Elem: Copy>;
}

impl<T, const N: usize> ArrayZipOuter<T, N> for [T; N]
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
    fn zip_outer_pin_ref<Z, const M: usize>(self: Pin<&Self>, other: &Z) -> [[(Pin<&T>, Z::Elem); M]; N]
    where
        Z: ArrayForm<M, Elem: Copy>
    {
        self.zip_outer_pin_ref_with(other, const |x, y| (x, y))
    }
}