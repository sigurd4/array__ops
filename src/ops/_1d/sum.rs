use core::ops::AddAssign;

use array_trait::Array;

use super::{ArrayDivideAndConquer, ArrayFold, ArrayReduce};

#[const_trait]
pub trait ArrayPartialSum<T, const N: usize>: Array<Item = T>
{
    fn partial_sum(self) -> Option<T>
    where
        T: AddAssign;
    fn sum_from<S>(self, from: S) -> S
    where
        S: AddAssign<T>;
    async fn partial_sum_async(self) -> Option<T>
    where
        T: AddAssign;
}

impl<T, const N: usize> ArrayPartialSum<T, N> for [T; N]
{
    fn partial_sum(self) -> Option<T>
    where
        T: AddAssign
    {
        self.reduce(|mut x, y| {
            x += y;
            x
        })
    }
    fn sum_from<S>(self, from: S) -> S
    where
        S: AddAssign<T>
    {
        self.fold(from, |mut x, y| {
            x += y;
            x
        })
    }
    async fn partial_sum_async(self) -> Option<T>
    where
        T: AddAssign
    {
        self.divide_and_conquer_async(async |mut x, y| {
            x += y;
            x
        }).await
    }
}