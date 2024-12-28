use core::ops::BitAndAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayBitAndAssign<T, const N: usize>: ArrayMeet<T, N>
{
    /// Applies the [`&=`](core::ops::BitAndAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0b0, 0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// a.bitand_assign_all(0b10);
    /// 
    /// assert_eq!(a, [0b0, 0b0, 0b10, 0b10, 0b0, 0b0, 0b10, 0b10]);
    /// ```
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously applies the [`&=`](core::ops::BitAndAssign) operator to all elements, copying the operand for each operation.
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
    /// a.bitand_assign_all_async(0b10).await;
    /// 
    /// assert_eq!(a, [0b0, 0b0, 0b10, 0b10, 0b0, 0b0, 0b10, 0b10]);
    /// # })
    /// ```
    async fn bitand_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
        
    fn bitand_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn bitand_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayBitAndAssign<T, N> for [T; N]
{
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, BitAndAssign::bitand_assign)
    }
        
    async fn bitand_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.bitand_assign(rhs)).await
    }
        
    fn bitand_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, BitAndAssign::bitand_assign)
    }
        
    async fn bitand_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.bitand_assign(rhs)).await
    }
}