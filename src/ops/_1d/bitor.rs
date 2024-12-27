use core::ops::BitOr;

use crate::form::ArrayForm;

use super::{Map, ZipWith};

#[const_trait]
pub trait ArrayBitOr<T, const N: usize>: Map<T, N>
{
    fn bitor_all<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>,
        Rhs: Copy;
        
    async fn bitor_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>,
        Rhs: Copy;
        
    fn bitor_each<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs::Elem>>::Output; N]
    where
        T: BitOr<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn bitor_each_async<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs::Elem>>::Output; N]
    where
        T: BitOr<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayBitOr<T, N> for [T; N]
{
    fn bitor_all<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x | rhs)
    }
        
    async fn bitor_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x | rhs).await
    }
        
    fn bitor_each<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs::Elem>>::Output; N]
    where
        T: BitOr<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_with(rhs, |x, y| x | y)
    }
        
    async fn bitor_each_async<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs::Elem>>::Output; N]
    where
        T: BitOr<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.zip_async_with(rhs, async |x, y| x | y).await
    }
}