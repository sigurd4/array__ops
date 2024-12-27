use core::ops::Add;

use crate::form::ArrayForm;

use super::{Map, ZipWith};

#[const_trait]
pub trait ArrayAdd<T, const N: usize>: Map<T, N>
{
    fn add_all<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>,
        Rhs: Copy;
        
    async fn add_all_async<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>,
        Rhs: Copy;
        
    fn add_each<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs::Elem>>::Output; N]
    where
        T: Add<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn add_each_async<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs::Elem>>::Output; N]
    where
        T: Add<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayAdd<T, N> for [T; N]
{
    fn add_all<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x + rhs)
    }
        
    async fn add_all_async<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x + rhs).await
    }
        
    fn add_each<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs::Elem>>::Output; N]
    where
        T: Add<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x + y)
    }
        
    async fn add_each_async<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs::Elem>>::Output; N]
    where
        T: Add<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, async |x, y| x + y).await
    }
}