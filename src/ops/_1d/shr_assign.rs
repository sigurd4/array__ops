use core::ops::ShrAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayShrAssign<T, const N: usize>: ArrayMeet<T, N>
{
    /// Applies the [`>>=`](core::ops::ShrAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0b0, 0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// a.shr_assign_all(1);
    /// 
    /// assert_eq!(a, [0b0, 0b0, 0b1, 0b1, 0b10, 0b10, 0b11, 0b11]);
    /// ```
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously applies the [`<<=`](core::ops::ShlAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// This way, each operation is a seperate `async` task that may be executed in parallel, but with some extra overhead.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// # tokio_test::block_on(async {
    /// let mut a = [0b0, 0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// a.shl_assign_all_async(1).await;
    /// 
    /// assert_eq!(a, [0b0, 0b10, 0b100, 0b110, 0b1000, 0b1010, 0b1100, 0b1110]);
    /// # })
    /// ```
    async fn shr_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
        
    fn shr_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn shr_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayShrAssign<T, N> for [T; N]
{
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, ShrAssign::shr_assign)
    }
        
    async fn shr_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.shr_assign(rhs)).await
    }
        
    fn shr_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, ShrAssign::shr_assign)
    }
        
    async fn shr_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.shr_assign(rhs)).await
    }
}