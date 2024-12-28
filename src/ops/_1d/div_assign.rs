use core::ops::DivAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayDivAssign<T, const N: usize>: ArrayMeet<T, N>
{
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
        
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
        
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