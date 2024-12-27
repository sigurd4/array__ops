use core::pin::Pin;

use array_trait::Array;

#[const_trait]
pub trait ArrayFlatten<T, const M: usize, const N: usize>: Array<Item = [T; N]>
{
    fn flatten(self) -> [T; M*N];
    fn flatten_ref(&self) -> &[T; M*N];
    fn flatten_mut(&mut self) -> &mut [T; M*N];
    fn flatten_pin_ref(self: Pin<&Self>) -> Pin<&[T; M*N]>;
    fn flatten_pin_mut(self: Pin<&mut Self>) -> Pin<&mut [T; M*N]>;
}

impl<T, const M: usize, const N: usize> const ArrayFlatten<T, M, N> for [[T; N]; M]
{
    fn flatten(self) -> [T; M*N]
    {
        let flattened = unsafe {
            self.as_ptr().cast::<[T; M*N]>().read()
        };
        core::mem::forget(self);
        flattened
    }
    fn flatten_ref(&self) -> &[T; M*N]
    {
        unsafe {
            self.as_ptr().cast::<[T; M*N]>().as_ref_unchecked()
        }
    }
    fn flatten_mut(&mut self) -> &mut [T; M*N]
    {
        unsafe {
            self.as_mut_ptr().cast::<[T; M*N]>().as_mut_unchecked()
        }
    }
    fn flatten_pin_ref(self: Pin<&Self>) -> Pin<&[T; M*N]>
    {
        unsafe {
            Pin::new_unchecked(self.get_ref().flatten_ref())
        }
    }
    fn flatten_pin_mut(self: Pin<&mut Self>) -> Pin<&mut [T; M*N]>
    {
        unsafe {
            Pin::new_unchecked(self.get_unchecked_mut().flatten_mut())
        }
    }
}