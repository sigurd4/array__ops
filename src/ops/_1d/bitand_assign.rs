use core::ops::BitAndAssign;

use crate::form::ArrayForm;

use super::Meet;

#[const_trait]
pub trait ArrayBitAndAssign<T, const N: usize>: Meet<T, N>
{
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
        
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