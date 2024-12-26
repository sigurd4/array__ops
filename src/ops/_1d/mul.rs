use core::ops::Mul;

use crate::form::ArrayForm;

use super::{Map, ZipWith};

#[const_trait]
pub trait ArrayMul<T, const N: usize>: Map<T, N>
{
    fn mul_all<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy;
    fn mul_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Mul<Rhs>>::Output; N]
    where
        &'a T: Mul<Rhs>,
        Rhs: Copy;
        
    async fn mul_all_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy;
    async fn mul_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Mul<Rhs>>::Output; N]
    where
        &'a T: Mul<Rhs>,
        Rhs: Copy;
        
    fn mul_each<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs::Elem>>::Output; N]
    where
        T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    fn mul_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Mul<Rhs::Elem>>::Output; N]
    where
        &'a T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn mul_each_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs::Elem>>::Output; N]
    where
        T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    async fn mul_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Mul<Rhs::Elem>>::Output; N]
    where
        &'a T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayMul<T, N> for [T; N]
{
    fn mul_all<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x * rhs)
    }
    fn mul_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Mul<Rhs>>::Output; N]
    where
        &'a T: Mul<Rhs>,
        Rhs: Copy
    {
        self.map_ref(|x| x * rhs)
    }
        
    async fn mul_all_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy
    {
        self.map_async(|x| x * rhs)
    }
    async fn mul_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Mul<Rhs>>::Output; N]
    where
        &'a T: Mul<Rhs>,
        Rhs: Copy
    {
        self.map_ref_async(|x| x * rhs)
    }
        
    fn mul_each<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs::Elem>>::Output; N]
    where
        T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x * y)
    }
    fn mul_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Mul<Rhs::Elem>>::Output; N]
    where
        &'a T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_with(rhs, |x, y| x * y)
    }
        
    async fn mul_each_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs::Elem>>::Output; N]
    where
        T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, |x, y| x * y)
    }
    async fn mul_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Mul<Rhs::Elem>>::Output; N]
    where
        &'a T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_async_with(rhs, |x, y| x * y)
    }
}