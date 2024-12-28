use core::ops::BitOrAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayBitOrAssign<T, const N: usize>: ArrayMeet<T, N>
{
    /// Applies the [`|=`](core::ops::BitOrAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0b0, 0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// a.bitor_assign_all(0b10);
    /// 
    /// assert_eq!(a, [0b10, 0b11, 0b10, 0b11, 0b110, 0b111, 0b110, 0b111]);
    /// ```
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously applies the [`|=`](core::ops::BitOrAssign) operator to all elements, copying the operand for each operation.
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
    /// a.bitor_assign_all_async(0b10).await;
    /// 
    /// assert_eq!(a, [0b10, 0b11, 0b10, 0b11, 0b110, 0b111, 0b110, 0b111]);
    /// # })
    /// ```
    async fn bitor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
        
    fn bitor_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn bitor_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayBitOrAssign<T, N> for [T; N]
{
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, BitOrAssign::bitor_assign)
    }
        
    async fn bitor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.bitor_assign(rhs)).await
    }
        
    fn bitor_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, BitOrAssign::bitor_assign)
    }
        
    async fn bitor_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.bitor_assign(rhs)).await
    }
}