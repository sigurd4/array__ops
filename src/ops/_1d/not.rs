use core::ops::Not;

use super::Map;

#[const_trait]
pub trait ArrayNot<T, const N: usize>: Map<T, N>
{
    fn not_all(self) -> [<T as Not>::Output; N]
    where
        T: Not
    {
        self.map(|x| !x)
    }
    fn not_ref_all<'a>(&'a self) -> [<&'a T as Not>::Output; N]
    where
        &'a T: Not,
        T: 'a
    {
        self.map_ref(|x| !x)
    }
    
    async fn not_all_async(self) -> [<T as Not>::Output; N]
    where
        T: Not
    {
        self.map_async(async |x| !(x as T)).await
    }
    async fn not_ref_all_async<'a>(&'a self) -> [<&'a T as Not>::Output; N]
    where
        &'a T: Not,
        T: 'a
    {
        self.map_ref_async(async |x| !(x as &T)).await
    }
}

impl<T, const N: usize> ArrayNot<T, N> for [T; N]
{
    
}