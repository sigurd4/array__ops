use core::ops::RemAssign;

use crate::form::ArrayForm;

use super::Meet;

#[const_trait]
pub trait ArrayRemAssign<T, const N: usize>: Meet<T, N>
{
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
        
    async fn rem_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
        
    fn rem_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn rem_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayRemAssign<T, N> for [T; N]
{
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, RemAssign::rem_assign)
    }
        
    async fn rem_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.rem_assign(rhs)).await
    }
        
    fn rem_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, RemAssign::rem_assign)
    }
        
    async fn rem_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.rem_assign(rhs)).await
    }
}