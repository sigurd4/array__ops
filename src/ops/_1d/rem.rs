use core::ops::Rem;

use crate::form::ArrayForm;

use super::{ArrayMap, ArrayZipWith};

#[const_trait]
pub trait ArrayRem<T, const N: usize>: ArrayMap<T, N>
{
    fn rem_all<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy;
        
    async fn rem_all_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy;
        
    fn rem_each<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs::Elem>>::Output; N]
    where
        T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn rem_each_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs::Elem>>::Output; N]
    where
        T: Rem<Rhs::Elem>,
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
        
    async fn rem_all_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x % rhs).await
    }
        
    fn rem_each<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs::Elem>>::Output; N]
    where
        T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x % y)
    }
        
    async fn rem_each_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs::Elem>>::Output; N]
    where
        T: Rem<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, async |x, y| x % y).await
    }
}