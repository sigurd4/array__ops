use core::ops::MulAssign;

use array_trait::Array;

use super::{DivideAndConquer, Fold, Reduce};

#[const_trait]
pub trait Product<T, const N: usize>: Array<Item = T>
{
    fn try_product(self) -> Option<T>
    where
        T: MulAssign;
    fn product_from<P>(self, from: P) -> P
    where
        P: MulAssign<T>;
    async fn try_product_async(self) -> Option<T>
    where
        T: MulAssign;
}

impl<T, const N: usize> Product<T, N> for [T; N]
{
    fn try_product(self) -> Option<T>
    where
        T: MulAssign
    {
        self.reduce(|mut x, y| {
            x *= y;
            x
        })
    }
    fn product_from<P>(self, from: P) -> P
    where
        P: MulAssign<T>
    {
        self.fold(from, |mut x, y| {
            x *= y;
            x
        })
    }
    async fn try_product_async(self) -> Option<T>
    where
        T: MulAssign
    {
        self.divide_and_conquer_async(async |mut x, y| {
            x *= y;
            x
        }).await
    }
}