use core::ops::{AddAssign, Div, Mul};

use crate::private;

use super::{ArrayPartialMagnitude, ArrayMul, ArrayPartialMulDot};

#[const_trait]
pub trait ArrayProj<T, const N: usize>: ArrayPartialMulDot<T, N>
{
    fn proj<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output>>::Output; N]
    where
        T: Mul<Rhs, Output: AddAssign + Div<<T as Mul>::Output, Output: Copy>> + Mul<T, Output: AddAssign> + Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output> + Copy;
    async fn proj_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output>>::Output; N]
    where
        T: Mul<Rhs, Output: AddAssign + Div<<T as Mul>::Output, Output: Copy>> + Mul<T, Output: AddAssign> + Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output> + Copy;
}

impl<T, const N: usize> ArrayProj<T, N> for [T; N]
{
    fn proj<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output>>::Output; N]
    where
        T: Mul<Rhs, Output: AddAssign + Div<<T as Mul>::Output, Output: Copy>> + Mul<T, Output: AddAssign> + Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output> + Copy
    {
        if N == 0
        {
            return private::empty()
        }
        let uv = self.try_mul_dot(rhs);
        let uu = self.try_magnitude_squared();
        let a = unsafe {
            uv.unwrap_unchecked()/uu.unwrap_unchecked()
        };
        self.mul_all(a)
    }
    async fn proj_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output>>::Output; N]
    where
        T: Mul<Rhs, Output: AddAssign + Div<<T as Mul>::Output, Output: Copy>> + Mul<T, Output: AddAssign> + Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output> + Copy
    {
        if N == 0
        {
            return private::empty()
        }
        let (uv, uu) = core::future::join!(
            self.try_mul_dot_async(rhs),
            self.try_magnitude_squared_async()
        ).await;
        let a = unsafe {
            uv.unwrap_unchecked()/uu.unwrap_unchecked()
        };
        self.mul_all_async(a).await
    }
}