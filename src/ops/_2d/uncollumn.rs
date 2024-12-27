use core::pin::Pin;

use array_trait::Array;

use crate::private;

#[const_trait]
pub trait ArrayUncollumn<T, const N: usize>: Array<Item = [T; 1]>
{
    fn uncollumn(self) -> [T; N];
    fn uncollumn_ref(&self) -> &[T; N];
    fn uncollumn_mut(&mut self) -> &mut [T; N];
    fn uncollumn_pin_ref(self: Pin<&Self>) -> Pin<&[T; N]>;
    fn uncollumn_pin_mut(self: Pin<&mut Self>) -> Pin<&mut [T; N]>;
}

impl<T, const N: usize> ArrayUncollumn<T, N> for [[T; 1]; N]
{
    fn uncollumn(self) -> [T; N]
    {
        unsafe {
            private::transmute(self)
        }
    }
    fn uncollumn_ref(&self) -> &[T; N]
    {
        unsafe {
            self.as_ptr().cast::<[T; N]>().as_ref_unchecked()
        }
    }
    fn uncollumn_mut(&mut self) -> &mut [T; N]
    {
        unsafe {
            self.as_mut_ptr().cast::<[T; N]>().as_mut_unchecked()
        }
    }
    fn uncollumn_pin_ref(self: Pin<&Self>) -> Pin<&[T; N]>
    {
        unsafe {
            Pin::new_unchecked(self.get_ref().uncollumn_ref())
        }
    }
    fn uncollumn_pin_mut(self: Pin<&mut Self>) -> Pin<&mut [T; N]>
    {
        unsafe {
            Pin::new_unchecked(self.get_unchecked_mut().uncollumn_mut())
        }
    }
}