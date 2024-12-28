use core::ops::MulAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayMulAssign<T, const N: usize>: ArrayMeet<T, N>
{
    /// Applies the [`*=`](core::ops::MulAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// 
    /// a.mul_assign_all(2);
    /// 
    /// assert_eq!(a, [0, 2, 4, 6, 8, 10, 12, 14]);
    /// ```
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously applies the [`*=`](core::ops::MulAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// This way, each operation is a seperate `async` task that may be executed in parallel, but with some extra overhead.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// 
    /// a.mul_assign_all_async(2).await;
    /// 
    /// assert_eq!(a, [0, 2, 4, 6, 8, 10, 12, 14]);
    /// # })
    /// ```
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
        
    /// Applies [`*=`](core::ops::MulAssign) `rhs[..]` to each element pairwise.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// let b = [7, 6, 5, 4, 3, 2, 1, 0];
    /// 
    /// a.mul_assign_each(b);
    /// 
    /// assert_eq!(a, [0, 6, 10, 12, 12, 10, 6, 0]);
    /// ```
    fn mul_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn mul_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayMulAssign<T, N> for [T; N]
{
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, MulAssign::mul_assign)
    }
        
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.mul_assign(rhs)).await
    }
        
    fn mul_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, MulAssign::mul_assign)
    }
        
    async fn mul_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.mul_assign(rhs)).await
    }
}