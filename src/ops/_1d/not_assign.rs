use core::ops::Not;

use super::ArrayMapAssign;

#[const_trait]
pub trait ArrayNotAssign<T, const N: usize>: ArrayMapAssign<T, N>
{
    /// Applies the [`!`](core::ops::Not) operator on all elements, in-place.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [true, false, true];
    /// 
    /// a.not_assign_all();
    /// 
    /// assert_eq!(a, [false, true, false]);
    /// ```
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>;

    /// Asynchronously applies the [`!`](core::ops::Not) operator on all elements, in-place.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut a = [true, false, true];
    /// 
    /// a.not_assign_all_async().await;
    /// 
    /// assert_eq!(a, [false, true, false]);
    /// # })
    /// ```
    async fn not_assign_all_async(&mut self) 
    where
        T: Not<Output = T>;
}

impl<T, const N: usize> ArrayNotAssign<T, N> for [T; N]
{
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>
    {
        self.map_assign(|x| !x)
    }

    async fn not_assign_all_async(&mut self)
    where
        T: Not<Output = T>
    {
        self.map_assign_async(async |x| !x).await
    }
}