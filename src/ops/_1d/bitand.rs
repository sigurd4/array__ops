use core::ops::BitAnd;

use crate::form::ArrayForm;

use super::{Map, ZipWith};

#[const_trait]
pub trait ArrayBitAnd<T, const N: usize>: Map<T, N>
{
    fn bitand_all<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>,
        Rhs: Copy;
    fn bitand_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as BitAnd<Rhs>>::Output; N]
    where
        &'a T: BitAnd<Rhs>,
        Rhs: Copy;
        
    async fn bitand_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>,
        Rhs: Copy;
    async fn bitand_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as BitAnd<Rhs>>::Output; N]
    where
        &'a T: BitAnd<Rhs>,
        Rhs: Copy;
        
    fn bitand_each<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs::Elem>>::Output; N]
    where
        T: BitAnd<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    fn bitand_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as BitAnd<Rhs::Elem>>::Output; N]
    where
        &'a T: BitAnd<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn bitand_each_async<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs::Elem>>::Output; N]
    where
        T: BitAnd<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    async fn bitand_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as BitAnd<Rhs::Elem>>::Output; N]
    where
        &'a T: BitAnd<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayBitAnd<T, N> for [T; N]
{
    fn bitand_all<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x & rhs)
    }
    fn bitand_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as BitAnd<Rhs>>::Output; N]
    where
        &'a T: BitAnd<Rhs>,
        Rhs: Copy
    {
        self.map_ref(|x| x & rhs)
    }
        
    async fn bitand_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x & rhs).await
    }
    async fn bitand_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as BitAnd<Rhs>>::Output; N]
    where
        &'a T: BitAnd<Rhs>,
        Rhs: Copy
    {
        self.map_ref_async(async |x| x & rhs).await
    }
        
    fn bitand_each<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs::Elem>>::Output; N]
    where
        T: BitAnd<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x & y)
    }
    fn bitand_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as BitAnd<Rhs::Elem>>::Output; N]
    where
        &'a T: BitAnd<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_with(rhs, |x, y| x & y)
    }
        
    async fn bitand_each_async<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs::Elem>>::Output; N]
    where
        T: BitAnd<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, async |x, y| x & y).await
    }
    async fn bitand_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as BitAnd<Rhs::Elem>>::Output; N]
    where
        &'a T: BitAnd<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_async_with(rhs, async |x, y| x & y).await
    }
}