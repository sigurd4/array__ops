use core::ops::SubAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArraySubAssign<T, const N: usize>: ArrayMeet<T, N>
{
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
        
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
        
    fn sub_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn sub_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArraySubAssign<T, N> for [T; N]
{
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, SubAssign::sub_assign)
    }
        
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.sub_assign(rhs)).await
    }
        
    fn sub_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, SubAssign::sub_assign)
    }
        
    async fn sub_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.sub_assign(rhs)).await
    }
}