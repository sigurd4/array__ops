use core::ops::DivAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayDivAssign<T, const N: usize>: ArrayMeet<T, N>
{
    /// Applies the [`/=`](core::ops::DivAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// 
    /// a.div_assign_all(2);
    /// 
    /// assert_eq!(a, [0, 0, 1, 1, 2, 2, 3, 3]);
    /// ```
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously applies the [`/=`](core::ops::DivAssign) operator to all elements, copying the operand for each operation.
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
    /// a.div_assign_all_async(2).await;
    /// 
    /// assert_eq!(a, [0, 0, 1, 1, 2, 2, 3, 3]);
    /// # })
    /// ```
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
        
    /// Applies [`*=`](core::ops::MulAssign) `rhs[..]` to each element pairwise.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [7, 14, 21, 28, 35, 42, 49, 56];
    /// let b = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// a.div_assign_each(b);
    /// 
    /// assert_eq!(a, [7, 7, 7, 7, 7, 7, 7, 7]);
    /// ```
    fn div_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn div_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayDivAssign<T, N> for [T; N]
{
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, DivAssign::div_assign)
    }
        
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.div_assign(rhs)).await
    }
        
    fn div_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, DivAssign::div_assign)
    }
        
    async fn div_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.div_assign(rhs)).await
    }
}