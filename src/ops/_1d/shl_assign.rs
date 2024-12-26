use core::ops::ShlAssign;

use crate::form::ArrayForm;

use super::Meet;

#[const_trait]
pub trait ArrayShlAssign<T, const N: usize>: Meet<T, N>
{
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy;
        
    async fn shl_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy;
        
    fn shl_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn shl_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayShlAssign<T, N> for [T; N]
{
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, ShlAssign::shl_assign)
    }
        
    async fn shl_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.shl_assign(rhs)).await
    }
        
    fn shl_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, ShlAssign::shl_assign)
    }
        
    async fn shl_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.shl_assign(rhs)).await
    }
}