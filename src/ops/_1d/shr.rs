use core::ops::Shr;

use crate::form::ArrayForm;

use super::{ArrayMap, ArrayZipWith};

#[const_trait]
pub trait ArrayShr<T, const N: usize>: ArrayMap<T, N>
{
    fn shr_all<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy;
        
    async fn shr_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy;
        
    fn shr_each<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs::Elem>>::Output; N]
    where
        T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn shr_each_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs::Elem>>::Output; N]
    where
        T: Shr<Rhs::Elem>,
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
        
    async fn shr_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x >> rhs).await
    }
        
    fn shr_each<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs::Elem>>::Output; N]
    where
        T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x >> y)
    }
        
    async fn shr_each_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs::Elem>>::Output; N]
    where
        T: Shr<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, async |x, y| x >> y).await
    }
}