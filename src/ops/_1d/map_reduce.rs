use core::{marker::Destruct, pin::Pin};

use array_trait::Array;
use slice_ops::AsSlice;

use crate::private::guard::PartialEmptyGuard;

#[const_trait]
pub trait ArrayMapReduce<T, const N: usize>: Array + AsSlice<Item = T>
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
    fn map_reduce<F1, F2>(self, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(T,)> + ~const Destruct,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output + ~const Destruct;
    fn map_reduce_ref<'a, F1, F2>(&'a self, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(&'a T,)> + ~const Destruct,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output + ~const Destruct;
    fn map_reduce_mut<'a, F1, F2>(&'a mut self, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(&'a mut T,)> + ~const Destruct,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output + ~const Destruct;
    fn map_reduce_pin_ref<'a, F1, F2>(self: Pin<&'a Self>, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(Pin<&'a T>,)> + ~const Destruct,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output + ~const Destruct;
    fn map_reduce_pin_mut<'a, F1, F2>(self: Pin<&'a mut Self>, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(Pin<&'a mut T>,)> + ~const Destruct,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output + ~const Destruct;
}

impl<T, const N: usize> ArrayMapReduce<T, N> for [T; N]
{
    fn map_reduce<F1, F2>(self, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(T,)>,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output
    {
        PartialEmptyGuard::new_left(self).map_reduce(mapper, reduce)
    }
    fn map_reduce_ref<'a, F1, F2>(&'a self, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(&'a T,)>,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output
    {
        PartialEmptyGuard::new_left(self).map_reduce(mapper, reduce)
    }
    fn map_reduce_mut<'a, F1, F2>(&'a mut self, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(&'a mut T,)>,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output
    {
        PartialEmptyGuard::new_left(self).map_reduce(mapper, reduce)
    }
    fn map_reduce_pin_ref<'a, F1, F2>(self: Pin<&'a Self>, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(Pin<&'a T>,)>,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output
    {
        PartialEmptyGuard::new_left(self).map_reduce(mapper, reduce)
    }
    fn map_reduce_pin_mut<'a, F1, F2>(self: Pin<&'a mut Self>, mapper: F1, reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(Pin<&'a mut T>,)>,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output
    {
        PartialEmptyGuard::new_left(self).map_reduce(mapper, reduce)
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn it_works()
    {
        
    }
}