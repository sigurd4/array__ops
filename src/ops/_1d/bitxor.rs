use core::ops::BitXor;

use crate::form::ArrayForm;

use super::{ArrayMap, ArrayZipWith};

#[const_trait]
pub trait ArrayBitXor<T, const N: usize>: ArrayMap<T, N>
{
    fn bitxor_all<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>,
        Rhs: Copy;
        
    async fn bitxor_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>,
        Rhs: Copy;
        
    fn bitxor_each<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs::Elem>>::Output; N]
    where
        T: BitXor<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn bitxor_each_async<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs::Elem>>::Output; N]
    where
        T: BitXor<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayBitXor<T, N> for [T; N]
{
    fn bitxor_all<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x ^ rhs)
    }
        
    async fn bitxor_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x ^ rhs).await
    }
        
    fn bitxor_each<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs::Elem>>::Output; N]
    where
        T: BitXor<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x ^ y)
    }
        
    async fn bitxor_each_async<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs::Elem>>::Output; N]
    where
        T: BitXor<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, async |x, y| x ^ y).await
    }
}