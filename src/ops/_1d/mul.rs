use core::ops::Mul;

use crate::form::ArrayForm;

use super::{ArrayMap, ArrayZipWith};

#[const_trait]
pub trait ArrayMul<T, const N: usize>: ArrayMap<T, N>
{
    fn mul_all<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy;
        
    async fn mul_all_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy;
        
    fn mul_each<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs::Elem>>::Output; N]
    where
        T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn mul_each_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs::Elem>>::Output; N]
    where
        T: Mul<Rhs::Elem>,
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
        
    async fn mul_all_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x * rhs).await
    }
        
    fn mul_each<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs::Elem>>::Output; N]
    where
        T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x * y)
    }
        
    async fn mul_each_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs::Elem>>::Output; N]
    where
        T: Mul<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, async |x, y| x * y).await
    }
}