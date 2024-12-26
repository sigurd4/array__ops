use core::ops::BitOrAssign;

use crate::form::ArrayForm;

use super::Meet;

#[const_trait]
pub trait ArrayBitOrAssign<T, const N: usize>: Meet<T, N>
{
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
        
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