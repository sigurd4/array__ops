use core::{marker::Destruct, pin::Pin};

use array_trait::Array;

use crate::private::guard::PartialEmptyGuard;

#[const_trait]
pub trait Reduce<T, const N: usize>: Array<Item = T>
{
    /// Reduces elements in array into one element, using a given operand
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// const A: [u8; 3] = [1, 2, 3];
    /// 
    /// let r: u8 = A.reduce(|a, b| a + b).unwrap();
    /// 
    /// assert_eq!(r, 6);
    /// ```
    fn reduce<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T + ~const Destruct;
    fn reduce_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T + ~const Destruct;
    fn reduce_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T + ~const Destruct;
    fn reduce_pin_ref<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: FnMut(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T> + ~const Destruct;
    fn reduce_pin_mut<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: FnMut(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T> + ~const Destruct;
}

impl<T, const N: usize> Reduce<T, N> for [T; N]
{
    fn reduce<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_pin_ref<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: FnMut(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T>
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_pin_mut<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: FnMut(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T>
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
}