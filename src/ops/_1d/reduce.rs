use core::marker::Destruct;

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
    /// use array__ops::ArrayOps;
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
}