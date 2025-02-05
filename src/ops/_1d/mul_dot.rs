use core::ops::{AddAssign, Mul};

use array_trait::Array;
use slice_ops::AsSlice;

use crate::{private::guard::PartialZipEmptyGuard, form::ArrayForm};

use super::{sum::ArrayPartialSum, ArrayZipWith};

#[const_trait]
pub trait ArrayPartialMulDot<T, const N: usize>: Array + AsSlice<Item = T>
{
    fn try_mul_dot<Rhs>(self, rhs: Rhs) -> Option<<T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output: AddAssign>;

    async fn try_mul_dot_async<Rhs>(self, rhs: Rhs) -> Option<<T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output: AddAssign>;
        
    fn mul_dot_bias<Rhs, U>(self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign;

    async fn mul_dot_bias_async<Rhs, U>(self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign;
}

impl<T, const N: usize> ArrayPartialMulDot<T, N> for [T; N]
{
    fn try_mul_dot<Rhs>(self, rhs: Rhs) -> Option<<T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output: AddAssign>
    {
        if N == 0
        {
            return None
        }

        let mut guard = PartialZipEmptyGuard::new_left(
            self,
            rhs
        );
        let mut value = None;
        if guard.more()
        {
            let value = value.insert(guard.pop_with(Mul::mul));
            while guard.more()
            {
                *value += guard.pop_with(Mul::mul)
            }
        }
        guard.done();

        value
    }
    
    async fn try_mul_dot_async<Rhs>(self, rhs: Rhs) -> Option<<T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output: AddAssign>
    {
        self.zip_async_with(rhs, async |x, y| x*y).await
            .partial_sum_async().await
    }
        
    fn mul_dot_bias<Rhs, U>(self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign
    {
        if N == 0
        {
            return bias
        }

        let mut guard = PartialZipEmptyGuard::new_left(
            self,
            rhs
        );
        let mut value = bias;
        while guard.more()
        {
            value += guard.pop_with(Mul::mul)
        }
        guard.done();

        value
    }
    
    async fn mul_dot_bias_async<Rhs, U>(self, rhs: Rhs, mut bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign
    {
        if let Some(x) = self.try_mul_dot_async(rhs).await
        {
            bias += x
        }
        bias
    }
}