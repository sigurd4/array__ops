use core::ops::Not;

use super::Map;

#[const_trait]
pub trait ArrayNot<T, const N: usize>: Map<T, N>
{
    fn not_all(self) -> [<T as Not>::Output; N]
    where
        T: Not;
    
    async fn not_all_async(self) -> [<T as Not>::Output; N]
    where
        T: Not;
}

impl<T, const N: usize> ArrayNot<T, N> for [T; N]
{
    fn not_all(self) -> [<T as Not>::Output; N]
    where
        T: Not
    {
        self.map(|x| !x)
    }
    
    async fn not_all_async(self) -> [<T as Not>::Output; N]
    where
        T: Not
    {
        self.map_async(async |x| !(x as T)).await
    }
}