use core::ops::Rem;

use crate::form::ArrayForm;

use super::{Map, ZipWith};

#[const_trait]
pub trait ArrayRem<T, const N: usize>: Map<T, N>
{
    fn rem_all<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy;
    fn rem_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Rem<Rhs>>::Output; N]
    where
        &'a T: Rem<Rhs>,
        Rhs: Copy;
        
    async fn rem_all_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy;
    async fn rem_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Rem<Rhs>>::Output; N]
    where
        &'a T: Rem<Rhs>,
        Rhs: Copy;
        
    fn rem_each<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs::Elem>>::Output; N]
    where
        T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    fn rem_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Rem<Rhs::Elem>>::Output; N]
    where
        &'a T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn rem_each_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs::Elem>>::Output; N]
    where
        T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    async fn rem_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Rem<Rhs::Elem>>::Output; N]
    where
        &'a T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayRem<T, N> for [T; N]
{
    fn rem_all<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x % rhs)
    }
    fn rem_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Rem<Rhs>>::Output; N]
    where
        &'a T: Rem<Rhs>,
        Rhs: Copy
    {
        self.map_ref(|x| x % rhs)
    }
        
    async fn rem_all_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy
    {
        self.map_async(|x| x % rhs)
    }
    async fn rem_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Rem<Rhs>>::Output; N]
    where
        &'a T: Rem<Rhs>,
        Rhs: Copy
    {
        self.map_ref_async(|x| x % rhs)
    }
        
    fn rem_each<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs::Elem>>::Output; N]
    where
        T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x % y)
    }
    fn rem_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Rem<Rhs::Elem>>::Output; N]
    where
        &'a T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_with(rhs, |x, y| x % y)
    }
        
    async fn rem_each_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs::Elem>>::Output; N]
    where
        T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, |x, y| x % y)
    }
    async fn rem_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Rem<Rhs::Elem>>::Output; N]
    where
        &'a T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_async_with(rhs, |x, y| x % y)
    }
}