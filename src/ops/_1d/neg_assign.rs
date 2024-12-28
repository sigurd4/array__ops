use core::ops::Neg;

use super::ArrayMapAssign;

#[const_trait]
pub trait ArrayNegAssign<T, const N: usize>: ArrayMapAssign<T, N>
{
    /// Applies the [`-`](core::ops::Neg) operator on all elements, in-place.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// 
    /// a.neg_assign_all();
    /// 
    /// assert_eq!(a, [0, -1, -2, -3, -4, -5, -6, -7]);
    /// ```
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>;

    /// Asynchronously applies the [`-`](core::ops::Neg) operator on all elements, in-place.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// 
    /// a.neg_assign_all_async().await;
    /// 
    /// assert_eq!(a, [0, -1, -2, -3, -4, -5, -6, -7]);
    /// # })
    /// ```
    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>;
}

impl<T, const N: usize> ArrayNegAssign<T, N> for [T; N]
{
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>
    {
        self.map_assign(|x| -x)
    }

    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>
    {
        self.map_assign_async(async |x| -x).await
    }
}