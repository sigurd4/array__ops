use core::pin::Pin;

use array_trait::Array;

#[const_trait]
pub trait FromItem<T>: Array<Item = T, LENGTH = 1>
{
    fn from_item(value: T) -> Self;
    fn from_item_ref(value: &T) -> &Self;
    fn from_item_mut(value: &mut T) -> &mut Self;
    fn from_item_pin_ref(value: Pin<&T>) -> Pin<&Self>;
    fn from_item_pin_mut(value: Pin<&mut T>) -> Pin<&mut Self>;

    fn into_item(self) -> T;
    fn as_item(&self) -> &T;
    fn as_item_mut(&mut self) -> &mut T;
    fn as_item_pin(self: Pin<&Self>) -> Pin<&T>;
    fn as_item_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>;
}

impl<T> const FromItem<T> for [T; 1]
{
    fn from_item(value: T) -> Self
    {
        [value]
    }
    fn from_item_ref(value: &T) -> &Self
    {
        core::array::from_ref(value)
    }
    fn from_item_mut(value: &mut T) -> &mut Self
    {
        core::array::from_mut(value)
    }
    fn from_item_pin_ref(value: Pin<&T>) -> Pin<&Self>
    {
        unsafe {
            Pin::new_unchecked(core::array::from_ref(value.get_ref()))
        }
    }
    fn from_item_pin_mut(value: Pin<&mut T>) -> Pin<&mut Self>
    {
        unsafe {
            Pin::new_unchecked(core::array::from_mut(value.get_unchecked_mut()))
        }
    }

    fn into_item(self) -> T
    {
        let value = unsafe {
            core::ptr::read(self.as_item())
        };
        core::mem::forget(self);
        value
    }
    fn as_item(&self) -> &T
    {
        &self[0]
    }
    fn as_item_mut(&mut self) -> &mut T
    {
        &mut self[0]
    }
    fn as_item_pin(self: Pin<&Self>) -> Pin<&T>
    {
        unsafe {
            Pin::new_unchecked(self.get_ref().as_item())
        }
    }
    fn as_item_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>
    {
        unsafe {
            Pin::new_unchecked(self.get_unchecked_mut().as_item_mut())
        }
    }
}