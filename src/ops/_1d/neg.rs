use core::ops::Neg;

use super::Map;

#[const_trait]
pub trait ArrayNeg<T, const N: usize>: Map<T, N>
{
    fn neg_all(self) -> [<T as Neg>::Output; N]
    where
        T: Neg
    {
        self.map(|x| -x)
    }
    fn neg_ref_all<'a>(&'a self) -> [<&'a T as Neg>::Output; N]
    where
        &'a T: Neg,
        T: 'a
    {
        self.map_ref(|x| -x)
    }
    
    async fn neg_all_async(self) -> [<T as Neg>::Output; N]
    where
        T: Neg
    {
        self.map_async(async |x| -(x as T)).await
    }
    async fn neg_ref_all_async<'a>(&'a self) -> [<&'a T as Neg>::Output; N]
    where
        &'a T: Neg,
        T: 'a
    {
        self.map_ref_async(async |x| -(x as &T)).await
    }
}

impl<T, const N: usize> ArrayNeg<T, N> for [T; N]
{
    
}