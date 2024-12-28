use core::pin::Pin;

use array_trait::Array;

use crate::private;

#[const_trait]
pub trait ArrayFromCollumn<T, const N: usize>: Array<Item = T>
{
    fn from_collumn(collumn: [[T; 1]; N]) -> Self;
    fn from_collumn_ref(collumn: &[[T; 1]; N]) -> &Self;
    fn from_collumn_mut(collumn: &mut [[T; 1]; N]) -> &mut Self;
    fn from_collumn_pin_ref(collumn: Pin<&[[T; 1]; N]>) -> Pin<&Self>;
    fn from_collumn_pin_mut(collumn: Pin<&mut [[T; 1]; N]>) -> Pin<&mut Self>;

    fn into_collumn(self) -> [[T; 1]; N];
    fn as_collumn(&self) -> &[[T; 1]; N];
    fn as_collumn_mut(&mut self) -> &mut [[T; 1]; N];
    fn as_collumn_pin(self: Pin<&Self>) -> Pin<&[[T; 1]; N]>;
    fn as_collumn_pin_mut(self: Pin<&mut Self>) -> Pin<&mut [[T; 1]; N]>;
}

impl<T, const N: usize> const ArrayFromCollumn<T, N> for [T; N]
{
    fn from_collumn(collumn: [[T; 1]; N]) -> Self
    {
        unsafe {
            private::transmute(collumn)
        }
    }
    fn from_collumn_ref(collumn: &[[T; 1]; N]) -> &Self
    {
        unsafe {
            collumn.as_ptr().cast::<Self>().as_ref_unchecked()
        }
    }
    fn from_collumn_mut(collumn: &mut [[T; 1]; N]) -> &mut Self
    {
        unsafe {
            collumn.as_mut_ptr().cast::<Self>().as_mut_unchecked()
        }
    }
    fn from_collumn_pin_ref(collumn: Pin<&[[T; 1]; N]>) -> Pin<&Self>
    {
        unsafe {
            Pin::new_unchecked(Self::from_collumn_ref(collumn.get_ref()))
        }
    }
    fn from_collumn_pin_mut(collumn: Pin<&mut [[T; 1]; N]>) -> Pin<&mut Self>
    {
        unsafe {
            Pin::new_unchecked(Self::from_collumn_mut(collumn.get_unchecked_mut()))
        }
    }

    fn into_collumn(self) -> [[T; 1]; N]
    {
        unsafe {
            private::transmute(self)
        }
    }
    fn as_collumn(&self) -> &[[T; 1]; N]
    {
        unsafe {
            self.as_ptr().cast::<[[T; 1]; N]>().as_ref_unchecked()
        }
    }
    fn as_collumn_mut(&mut self) -> &mut [[T; 1]; N]
    {
        unsafe {
            self.as_mut_ptr().cast::<[[T; 1]; N]>().as_mut_unchecked()
        }
    }
    fn as_collumn_pin(self: Pin<&Self>) -> Pin<&[[T; 1]; N]>
    {
        unsafe {
            Pin::new_unchecked(self.get_ref().as_collumn())
        }
    }
    fn as_collumn_pin_mut(self: Pin<&mut Self>) -> Pin<&mut [[T; 1]; N]>
    {
        unsafe {
            Pin::new_unchecked(self.get_unchecked_mut().as_collumn_mut())
        }
    }
}