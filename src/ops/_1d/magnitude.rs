use core::ops::{AddAssign, Mul};

use super::ArrayPartialMulDot;

#[const_trait]
pub trait ArrayPartialMagnitude<T, const N: usize>: ArrayPartialMulDot<T, N>
{
    fn try_magnitude_squared(&self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy;
    async fn try_magnitude_squared_async(&self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy;
}

impl<T, const N: usize> ArrayPartialMagnitude<T, N> for [T; N]
{
    fn try_magnitude_squared(&self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy
    {
        if N == 0
        {
            return None;
        }

        let mut y = self[0]*self[0];
        let mut i = 1;
        while i < N
        {
            y += self[i]*self[i];
            i += 1
        }
        Some(y)
    }
    async fn try_magnitude_squared_async(&self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy
    {
        self.try_mul_dot_async(*self).await
    }
}