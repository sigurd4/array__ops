use core::ops::SubAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArraySubAssign<T, const N: usize>: ArrayMeet<T, N>
{
    /// Applies the [`-=`](core::ops::SubAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// 
    /// a.sub_assign_all(2);
    /// 
    /// assert_eq!(a, [-2, -1, 0, 1, 2, 3, 4, 5]);
    /// ```
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously applies the [`-=`](core::ops::SubAssign) operator to all elements, copying the operand for each operation.
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
    /// a.sub_assign_all_async(2).await;
    /// 
    /// assert_eq!(a, [-2, -1, 0, 1, 2, 3, 4, 5]);
    /// # })
    /// ```
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
        
    /// Applies [`-=`](core::ops::SubAssign) `rhs[..]` to each element pairwise.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// let b = [7, 8, 9, 10, 11, 12, 13, 14];
    /// 
    /// a.sub_assign_each(b);
    /// 
    /// assert_eq!(a, [-7, -7, -7, -7, -7, -7, -7, -7]);
    /// ```
    fn sub_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn sub_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArraySubAssign<T, N> for [T; N]
{
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, SubAssign::sub_assign)
    }
        
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.sub_assign(rhs)).await
    }
        
    fn sub_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, SubAssign::sub_assign)
    }
        
    async fn sub_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.sub_assign(rhs)).await
    }
}