use core::ops::Sub;

use crate::form::ArrayForm;

use super::{Map, ZipWith};

#[const_trait]
pub trait ArraySub<T, const N: usize>: Map<T, N>
{
    fn sub_all<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>,
        Rhs: Copy;
    fn sub_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Sub<Rhs>>::Output; N]
    where
        &'a T: Sub<Rhs>,
        Rhs: Copy;
        
    async fn sub_all_async<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>,
        Rhs: Copy;
    async fn sub_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Sub<Rhs>>::Output; N]
    where
        &'a T: Sub<Rhs>,
        Rhs: Copy;
        
    fn sub_each<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs::Elem>>::Output; N]
    where
        T: Sub<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    fn sub_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Sub<Rhs::Elem>>::Output; N]
    where
        &'a T: Sub<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn sub_each_async<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs::Elem>>::Output; N]
    where
        T: Sub<Rhs::Elem>,
        Rhs: ArrayForm<N>;
    async fn sub_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Sub<Rhs::Elem>>::Output; N]
    where
        &'a T: Sub<Rhs::Elem>,
        Rhs: ArrayForm<N>;

    fn rsub_all<Lhs>(self, lhs: Lhs) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Sub<T> + Copy;
    fn rsub_all_ref<'a, Lhs>(&'a self, lhs: Lhs) -> [<Lhs as Sub<&'a T>>::Output; N]
    where
        Lhs: Sub<&'a T> + Copy;
        
    async fn rsub_all_async<Lhs>(self, lhs: Lhs) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Sub<T> + Copy;
    async fn rsub_all_ref_async<'a, Lhs>(&'a self, lhs: Lhs) -> [<Lhs as Sub<&'a T>>::Output; N]
    where
        Lhs: Sub<&'a T> + Copy;
        
    fn rsub_each<Lhs>(self, lhs: Lhs) -> [<Lhs::Elem as Sub<T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Sub<T>>;
    fn rsub_each_ref<'a, Lhs>(&'a self, lhs: Lhs) -> [<Lhs::Elem as Sub<&'a T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Sub<&'a T>>;
        
    async fn rsub_each_async<Lhs>(self, lhs: Lhs) -> [<Lhs::Elem as Sub<T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Sub<T>>;
    async fn rsub_each_ref_async<'a, Lhs>(&'a self, lhs: Lhs) -> [<Lhs::Elem as Sub<&'a T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Sub<&'a T>>;
}

impl<T, const N: usize> ArraySub<T, N> for [T; N]
{
    fn sub_all<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x - rhs)
    }
    fn sub_all_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Sub<Rhs>>::Output; N]
    where
        &'a T: Sub<Rhs>,
        Rhs: Copy
    {
        self.map_ref(|x| x - rhs)
    }
        
    async fn sub_all_async<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>,
        Rhs: Copy
    {
        self.map_async(|x| x - rhs).await
    }
    async fn sub_all_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Sub<Rhs>>::Output; N]
    where
        &'a T: Sub<Rhs>,
        Rhs: Copy
    {
        self.map_ref_async(|x| x - rhs).await
    }
        
    fn sub_each<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs::Elem>>::Output; N]
    where
        T: Sub<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x - y)
    }
    fn sub_each_ref<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Sub<Rhs::Elem>>::Output; N]
    where
        &'a T: Sub<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_with(rhs, |x, y| x - y)
    }
        
    async fn sub_each_async<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs::Elem>>::Output; N]
    where
        T: Sub<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, |x, y| x - y).await
    }
    async fn sub_each_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> [<&'a T as Sub<Rhs::Elem>>::Output; N]
    where
        &'a T: Sub<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_ref_async_with(rhs, |x, y| x - y).await
    }

    fn rsub_all<Lhs>(self, lhs: Lhs) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Sub<T> + Copy
    {
        self.map(|x| lhs - x)
    }
    fn rsub_all_ref<'a, Lhs>(&'a self, lhs: Lhs) -> [<Lhs as Sub<&'a T>>::Output; N]
    where
        Lhs: Sub<&'a T> + Copy
    {
        self.map_ref(|x| lhs - x)
    }
        
    async fn rsub_all_async<Lhs>(self, lhs: Lhs) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Sub<T> + Copy
    {
        self.map_async(|x| lhs - x).await
    }
    async fn rsub_all_ref_async<'a, Lhs>(&'a self, lhs: Lhs) -> [<Lhs as Sub<&'a T>>::Output; N]
    where
        Lhs: Sub<&'a T> + Copy
    {
        self.map_ref_async(|x| lhs - x).await
    }
        
    fn rsub_each<Lhs>(self, lhs: Lhs) -> [<Lhs::Elem as Sub<T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Sub<T>>
    {
        self.zip_with(lhs, |x, y| y - x)
    }
    fn rsub_each_ref<'a, Lhs>(&'a self, lhs: Lhs) -> [<Lhs::Elem as Sub<&'a T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Sub<&'a T>>
    {
        self.zip_ref_with(lhs, |x, y| y - x)
    }
        
    async fn rsub_each_async<Lhs>(self, lhs: Lhs) -> [<Lhs::Elem as Sub<T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Sub<T>>
    {
        self.zip_async_with(lhs, |x, y| y - x).await
    }
    async fn rsub_each_ref_async<'a, Lhs>(&'a self, lhs: Lhs) -> [<Lhs::Elem as Sub<&'a T>>::Output; N]
    where
        Lhs: ArrayForm<N, Elem: Sub<&'a T>>
    {
        self.zip_ref_async_with(lhs, |x, y| y - x).await
    }
}