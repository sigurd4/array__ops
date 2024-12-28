use core::ops::AddAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayAddAssign<T, const N: usize>: ArrayMeet<T, N>
{
    /// Applies the [`+=`](core::ops::AddAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// 
    /// a.add_assign_all(2);
    /// 
    /// assert_eq!(a, [2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously applies the [`+=`](core::ops::AddAssign) operator to all elements, copying the operand for each operation.
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
    /// a.add_assign_all_async(2).await;
    /// 
    /// assert_eq!(a, [2, 3, 4, 5, 6, 7, 8, 9]);
    /// # })
    /// ```
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
        
    /// Applies [`+=`](core::ops::AddAssign) `rhs[..]` to each element pairwise.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
    /// let b = [7, 6, 5, 4, 3, 2, 1, 0];
    /// 
    /// a.add_assign_each(b);
    /// 
    /// assert_eq!(a, [7, 7, 7, 7, 7, 7, 7, 7]);
    /// ```
    fn add_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn add_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayAddAssign<T, N> for [T; N]
{
    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, AddAssign::add_assign)
    }
        
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.add_assign(rhs)).await
    }
        
    fn add_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, AddAssign::add_assign)
    }
        
    async fn add_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.add_assign(rhs)).await
    }
}