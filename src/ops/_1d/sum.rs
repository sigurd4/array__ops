use core::ops::AddAssign;

use array_trait::Array;

#[const_trait]
pub trait Sum<T, const N: usize>: Array<Item = T>
{
    fn try_sum(self) -> Option<T>
    where
        T: AddAssign;
    fn sum_from<S>(self, from: S) -> S
    where
        S: AddAssign<T>;
    async fn try_sum_async(self) -> Option<T>
    where
        T: AddAssign;
}

impl<T, const N: usize> Sum<T, N> for [T; N]
{
    fn try_sum(self) -> Option<T>
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
    async fn try_sum_async(self) -> Option<T>
    where
        T: AddAssign
    {
        self.divide_and_conquer_async(async |mut x, y| {
            x += y;
            x
        }).await
    }
}