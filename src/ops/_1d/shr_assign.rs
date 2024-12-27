use core::ops::ShrAssign;

use crate::form::ArrayForm;

use super::Meet;

#[const_trait]
pub trait ArrayShrAssign<T, const N: usize>: Meet<T, N>
{
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
        
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