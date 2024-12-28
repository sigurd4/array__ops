use core::ops::BitXorAssign;

use crate::form::ArrayForm;

use super::ArrayMeet;

#[const_trait]
pub trait ArrayBitXorAssign<T, const N: usize>: ArrayMeet<T, N>
{
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy;
        
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