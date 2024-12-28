use core::ops::BitXorAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayBitXorAssign<T, const N: usize>: ArrayMeet<T, N>
{
    /// Applies the [`^=`](core::ops::BitXorAssign) operator to all elements, copying the operand for each operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [0b0, 0b1, 0b10, 0b11, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// a.bitxor_assign_all(0b10);
    /// 
    /// assert_eq!(a, [0b10, 0b11, 0b0, 0b1, 0b110, 0b111, 0b100, 0b101]);
    /// ```
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy;
        
    /// Asynchronously applies the [`^=`](core::ops::BitXorAssign) operator to all elements, copying the operand for each operation.
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
    /// a.bitxor_assign_all_async(0b10).await;
    /// 
    /// assert_eq!(a, [0b10, 0b11, 0b0, 0b1, 0b110, 0b111, 0b100, 0b101]);
    /// # })
    /// ```
    async fn bitxor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy;
        
    fn bitxor_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn bitxor_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayBitXorAssign<T, N> for [T; N]
{
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, BitXorAssign::bitxor_assign)
    }
        
    async fn bitxor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.bitxor_assign(rhs)).await
    }
        
    fn bitxor_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, BitXorAssign::bitxor_assign)
    }
        
    async fn bitxor_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.bitxor_assign(rhs)).await
    }
}