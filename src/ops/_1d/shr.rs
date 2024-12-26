use core::ops::Shr;

use crate::form::ArrayForm;

use super::{Map, ZipWith};

#[const_trait]
pub trait ArrayShr<T, const N: usize>: Map<T, N>
{
    fn shr_all<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy;
    fn shr_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shr<Rhs>>::Output; N]
    where
        &'a T: Shr<Rhs>,
        Rhs: Copy;
        
    async fn shr_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy;
    async fn shr_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shr<Rhs>>::Output; N]
    where
        &'a T: Shr<Rhs>,
        Rhs: Copy;
        
    fn shr_each<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs::Elem>>::Output; N]
    where
        T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    fn shr_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shr<Rhs::Elem>>::Output; N]
    where
        &'a T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn shr_each_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs::Elem>>::Output; N]
    where
        T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    async fn shr_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shr<Rhs::Elem>>::Output; N]
    where
        &'a T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayShr<T, N> for [T; N]
{
    fn shr_all<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x >> rhs)
    }
    fn shr_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shr<Rhs>>::Output; N]
    where
        &'a T: Shr<Rhs>,
        Rhs: Copy
    {
        self.map_ref(|x| x >> rhs)
    }
        
    async fn shr_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy
    {
        self.map_async(|x| x >> rhs)
    }
    async fn shr_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shr<Rhs>>::Output; N]
    where
        &'a T: Shr<Rhs>,
        Rhs: Copy
    {
        self.map_ref_async(|x| x >> rhs)
    }
        
    fn shr_each<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs::Elem>>::Output; N]
    where
        T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x >> y)
    }
    fn shr_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shr<Rhs::Elem>>::Output; N]
    where
        &'a T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_with(rhs, |x, y| x >> y)
    }
        
    async fn shr_each_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs::Elem>>::Output; N]
    where
        T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, |x, y| x >> y)
    }
    async fn shr_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Shr<Rhs::Elem>>::Output; N]
    where
        &'a T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_async_with(rhs, |x, y| x >> y)
    }
}