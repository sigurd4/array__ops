use array_trait::Array;

use crate::private;

#[const_trait]
pub trait FromCollumn<T, const N: usize>: Array<Item = T>
{
    fn from_collumn(collumn: [[T; 1]; N]) -> Self;
    fn from_collumn_ref(collumn: &[[T; 1]; N]) -> &Self;
    fn from_collumn_mut(collumn: &mut [[T; 1]; N]) -> &mut Self;

    fn into_collumn(self) -> [[T; 1]; N];
    fn as_collumn(&self) -> &[[T; 1]; N];
    fn as_collumn_mut(&mut self) -> &mut [[T; 1]; N];
}

impl<T, const N: usize> const FromCollumn<T, N> for [T; N]
{
    fn from_collumn(collumn: [[T; 1]; N]) -> Self
    {
        unsafe {
            private::transmute_unchecked_size(collumn)
        }
    }
    fn from_collumn_ref(collumn: &[[T; 1]; N]) -> &Self
    {
        unsafe {
            &*collumn.as_ptr().cast()
        }
    }
    fn from_collumn_mut(collumn: &mut [[T; 1]; N]) -> &mut Self
    {
        unsafe {
            &mut *collumn.as_mut_ptr().cast()
        }
    }

    fn into_collumn(self) -> [[T; 1]; N]
    {
        unsafe {
            private::transmute_unchecked_size(self)
        }
    }
    fn as_collumn(&self) -> &[[T; 1]; N]
    {
        unsafe {
            &*self.as_ptr().cast()
        }
    }
    fn as_collumn_mut(&mut self) -> &mut [[T; 1]; N]
    {
        unsafe {
            &mut *self.as_mut_ptr().cast()
        }
    }
}