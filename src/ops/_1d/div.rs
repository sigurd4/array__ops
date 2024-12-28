use core::ops::Div;

use crate::form::ArrayForm;

use super::{ArrayMap, ArrayZipWith};

#[const_trait]
pub trait ArrayDiv<T, const N: usize>: ArrayMap<T, N>
{
    fn div_all<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>,
        Rhs: Copy;
        
    async fn div_all_async<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>,
        Rhs: Copy;
        
    fn div_each<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs::Elem>>::Output; N]
    where
        T: Div<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn div_each_async<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs::Elem>>::Output; N]
    where
        T: Div<Rhs::Elem>,
        Rhs: ArrayForm<N>;

    fn rdiv_all<Lhs>(self, lhs: Lhs) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Div<T> + Copy;
        
    async fn rdiv_all_async<Lhs>(self, lhs: Lhs) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Div<T> + Copy;
        
    fn rdiv_each<Lhs>(self, lhs: Lhs) -> [<Lhs::Elem as Div<T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Div<T>>;
        
    async fn rdiv_each_async<Lhs>(self, lhs: Lhs) -> [<Lhs::Elem as Div<T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Div<T>>;
}

impl<T, const N: usize> ArrayDiv<T, N> for [T; N]
{
    fn div_all<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x / rhs)
    }
        
    async fn div_all_async<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x / rhs).await
    }
        
    fn div_each<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs::Elem>>::Output; N]
    where
        T: Div<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x / y)
    }
        
    async fn div_each_async<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs::Elem>>::Output; N]
    where
        T: Div<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, async |x, y| x / y).await
    }

    fn rdiv_all<Lhs>(self, lhs: Lhs) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Div<T> + Copy
    {
        self.map(|x| lhs / x)
    }
        
    async fn rdiv_all_async<Lhs>(self, lhs: Lhs) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Div<T> + Copy
    {
        self.map_async(async |x| lhs / x).await
    }
        
    fn rdiv_each<Lhs>(self, lhs: Lhs) -> [<Lhs::Elem as Div<T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Div<T>>
    {
        self.zip_with(lhs, |x, y| y / x)
    }
        
    async fn rdiv_each_async<Lhs>(self, lhs: Lhs) -> [<Lhs::Elem as Div<T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Div<T>>
    {
        self.zip_async_with(lhs, async |x, y| y / x).await
    }
}