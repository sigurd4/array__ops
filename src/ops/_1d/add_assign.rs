use core::ops::AddAssign;

use crate::form::ArrayForm;

use super::Meet;

#[const_trait]
pub trait ArrayAddAssign<T, const N: usize>: Meet<T, N>
{
    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
        
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
        
    fn add_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn add_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayAddAssign<T, N> for [T; N]
{
    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, AddAssign::add_assign)
    }
        
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.add_assign(rhs)).await
    }
        
    fn add_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, AddAssign::add_assign)
    }
        
    async fn add_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.add_assign(rhs)).await
    }
}