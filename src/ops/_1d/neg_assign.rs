use core::ops::Neg;

use super::MapAssign;

#[const_trait]
pub trait ArrayNegAssign<T, const N: usize>: MapAssign<T, N>
{
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>;

    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>;
}

impl<T, const N: usize> ArrayNegAssign<T, N> for [T; N]
{
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>
    {
        self.map_assign(|x| -x)
    }

    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>
    {
        self.map_assign_async(async |x| -x).await
    }
}