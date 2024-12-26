use core::ops::{AddAssign, Mul};

use array_trait::Array;

use crate::{private::guard::PartialZipEmptyGuard, ArrayForm};

#[const_trait]
pub trait ArrayMulDot<T, const N: usize>: Array<Item = T>
{
    fn try_mul_dot<Rhs>(self, rhs: Rhs) -> Option<<T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output: AddAssign>;
    fn try_mul_dot_ref<'a, Rhs>(&'a self, rhs: Rhs) -> Option<<&'a T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        &'a T: Mul<Rhs::Elem, Output: AddAssign>;
    async fn try_mul_dot_async<Rhs>(self, rhs: Rhs) -> Option<<T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output: AddAssign>;
    async fn try_mul_dot_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> Option<<&'a T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        &'a T: Mul<Rhs::Elem, Output: AddAssign>;
        
    fn mul_dot_bias<Rhs, U>(self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign;
    fn mul_dot_bias_ref<'a, Rhs, U>(&'a self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        &'a T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign;
    async fn mul_dot_bias_async<Rhs, U>(self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign;
    async fn mul_dot_bias_ref_async<'a, Rhs, U>(&'a self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        &'a T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign;
}

impl<T, const N: usize> ArrayMulDot<T, N> for [T; N]
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
    fn try_mul_dot_ref<'a, Rhs>(&'a self, rhs: Rhs) -> Option<<&'a T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        &'a T: Mul<Rhs::Elem, Output: AddAssign>
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
            .try_sum_async().await
    }
    async fn try_mul_dot_ref_async<'a, Rhs>(&'a self, rhs: Rhs) -> Option<<&'a T as Mul<Rhs::Elem>>::Output>
    where
        Rhs: ArrayForm<N>,
        &'a T: Mul<Rhs::Elem, Output: AddAssign>
    {
        self.zip_ref_async_with(rhs, async |x, y| x*y).await
            .try_sum_async().await
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
    fn mul_dot_bias_ref<'a, Rhs, U>(&'a self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        &'a T: Mul<Rhs::Elem, Output = U>,
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
    async fn mul_dot_bias_async<Rhs, U>(self, rhs: Rhs, bias: U) -> U
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
    async fn mul_dot_bias_ref_async<'a, Rhs, U>(&'a self, rhs: Rhs, bias: U) -> U
    where
        Rhs: ArrayForm<N>,
        &'a T: Mul<Rhs::Elem, Output = U>,
        U: AddAssign
    {
        if let Some(x) = self.try_mul_dot_ref_async(rhs).await
        {
            bias += x
        }
        bias
    }
}