use core::ops::Not;

use super::ArrayMapAssign;

#[const_trait]
pub trait ArrayNotAssign<T, const N: usize>: ArrayMapAssign<T, N>
{
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>;

    async fn not_assign_all_async(&mut self) 
    where
        T: Not<Output = T>;
}

impl<T, const N: usize> ArrayNotAssign<T, N> for [T; N]
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