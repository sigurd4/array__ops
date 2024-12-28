use core::ops::Neg;

use super::ArrayMap;

#[const_trait]
pub trait ArrayNeg<T, const N: usize>: ArrayMap<T, N>
{
    fn neg_all(self) -> [<T as Neg>::Output; N]
    where
        T: Neg;
    
    async fn neg_all_async(self) -> [<T as Neg>::Output; N]
    where
        T: Neg;
}

impl<T, const N: usize> ArrayNeg<T, N> for [T; N]
{
    fn neg_all(self) -> [<T as Neg>::Output; N]
    where
        T: Neg
    {
        self.map(|x| -x)
    }
    
    async fn neg_all_async(self) -> [<T as Neg>::Output; N]
    where
        T: Neg
    {
        self.map_async(async |x| -(x as T)).await
    }
}