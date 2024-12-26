use core::ops::Shl;

use crate::form::ArrayForm;

use super::{Map, ZipWith};

#[const_trait]
pub trait ArrayShl<T, const N: usize>: Map<T, N>
{
    fn shl_all<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>,
        Rhs: Copy;
    fn shl_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shl<Rhs>>::Output; N]
    where
        &'a T: Shl<Rhs>,
        Rhs: Copy;
        
    async fn shl_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>,
        Rhs: Copy;
    async fn shl_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shl<Rhs>>::Output; N]
    where
        &'a T: Shl<Rhs>,
        Rhs: Copy;
        
    fn shl_each<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs::Elem>>::Output; N]
    where
        T: Shl<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    fn shl_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shl<Rhs::Elem>>::Output; N]
    where
        &'a T: Shl<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn shl_each_async<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs::Elem>>::Output; N]
    where
        T: Shl<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    async fn shl_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shl<Rhs::Elem>>::Output; N]
    where
        &'a T: Shl<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayShl<T, N> for [T; N]
{
    fn shl_all<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x << rhs)
    }
    fn shl_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shl<Rhs>>::Output; N]
    where
        &'a T: Shl<Rhs>,
        Rhs: Copy
    {
        self.map_ref(|x| x << rhs)
    }
        
    async fn shl_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>,
        Rhs: Copy
    {
        self.map_async(|x| x << rhs)
    }
    async fn shl_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shl<Rhs>>::Output; N]
    where
        &'a T: Shl<Rhs>,
        Rhs: Copy
    {
        self.map_ref_async(|x| x << rhs)
    }
        
    fn shl_each<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs::Elem>>::Output; N]
    where
        T: Shl<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x << y)
    }
    fn shl_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shl<Rhs::Elem>>::Output; N]
    where
        &'a T: Shl<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_with(rhs, |x, y| x << y)
    }
        
    async fn shl_each_async<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs::Elem>>::Output; N]
    where
        T: Shl<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, |x, y| x << y)
    }
    async fn shl_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shl<Rhs::Elem>>::Output; N]
    where
        &'a T: Shl<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_async_with(rhs, |x, y| x << y)
    }
}