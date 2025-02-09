use core::ops::{AddAssign, Mul};

use slice_ops::ops::SliceVisit;

use super::{ArrayPartialMulDot, ArrayVisit};

#[const_trait]
pub trait ArrayPartialMagnitude<T, const N: usize>: ArrayPartialMulDot<T, N>
{
    fn partial_magnitude_squared(&self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy;
    fn magnitude_squared_from<O>(&self, from: O) -> O
    where
        T: Mul<T> + Copy,
        O: AddAssign<<T as Mul<T>>::Output>;
    async fn partial_magnitude_squared_async(&self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy;
}

impl<T, const N: usize> ArrayPartialMagnitude<T, N> for [T; N]
{
    fn partial_magnitude_squared(&self) -> Option<<T as Mul<T>>::Output>
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
    fn magnitude_squared_from<O>(&self, mut from: O) -> O
    where
        T: Mul<T> + Copy,
        O: AddAssign<<T as Mul<T>>::Output>
    {
        self.visit(|x| from += *x**x);
        from
    }
    async fn partial_magnitude_squared_async(&self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy
    {
        self.partial_mul_dot_async(*self).await
    }
}