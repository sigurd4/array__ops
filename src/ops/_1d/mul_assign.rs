use core::ops::MulAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayMulAssign<T, const N: usize>: ArrayMeet<T, N>
{
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
        
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
        
    fn mul_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
        
    async fn mul_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>;
}

impl<T, const N: usize> ArrayMulAssign<T, N> for [T; N]
{
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut(rhs, MulAssign::mul_assign)
    }
        
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.meet_all_mut_async(rhs, async |x, rhs| x.mul_assign(rhs)).await
    }
        
    fn mul_assign_each<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut(rhs, MulAssign::mul_assign)
    }
        
    async fn mul_assign_each_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs::Elem>,
        Rhs: ArrayForm<N>
    {
        self.meet_each_mut_async(rhs, async |x, rhs| x.mul_assign(rhs)).await
    }
}