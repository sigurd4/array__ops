use core::ops::{AddAssign, Mul};

use super::ArrayMulDot;

#[const_trait]
pub trait ArrayMagnitude<T, const N: usize>: ArrayMulDot<T, N>
{
    fn try_magnitude_squared(self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy;
    async fn try_magnitude_squared_async(self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy;
}

impl<T, const N: usize> ArrayMagnitude<T, N> for [T; N]
{
    fn try_magnitude_squared(self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy
    {
        self.try_mul_dot(self)
    }
    async fn try_magnitude_squared_async(self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy
    {
        self.try_mul_dot_async(self).await
    }
}