use core::ops::Not;

use super::MapAssign;

#[const_trait]
pub trait ArrayShrAssign<T, const N: usize>: MapAssign<T, N>
{
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>;

    async fn not_assign_all_async(&mut self) 
    where
        T: Not<Output = T>;
}

impl<T, const N: usize> ArrayShrAssign<T, N> for [T; N]
{
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>
    {
        self.map_assign(|x| !x)
    }

    async fn not_assign_all_async(&mut self)
    where
        T: Not<Output = T>
    {
        self.map_assign_async(async |x| !x).await
    }
}