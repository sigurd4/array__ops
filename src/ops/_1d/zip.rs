use core::{marker::Destruct, mem::MaybeUninit, ops::AsyncFn};

use array_trait::Array;

use crate::{private::guard::PartialZipGuard, ArrayForm, Runs, TryRuns};

use super::ZipWith;

#[const_trait]
pub trait Zip<T, const N: usize>: Array<Item = T>
{
    /// Combines two arrays with possibly different items into parallel, where each element lines up in the same position.
    /// 
    /// This method can be executed at compile-time, as opposed to the standard-library method.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// const A: [u8; 4] = [4, 3, 2, 1];
    /// const B: [&str; 4] = ["four", "three", "two", "one"];
    /// let c = A.zip(B);
    /// 
    /// assert_eq!(c, [(4, "four"), (3, "three"), (2, "two"), (1, "one")]);
    /// ```
    fn zip<Z>(self, other: Z) -> [(T, Z::Elem); N]
    where
        Z: ArrayForm<N>;
    fn zip_ref<Z>(&self, other: Z) -> [(&T, Z::Elem); N]
    where
        Z: ArrayForm<N>;
    fn zip_mut<Z>(&mut self, other: Z) -> [(&mut T, Z::Elem); N]
    where
        Z: ArrayForm<N>;
}

impl<T, const N: usize> Zip<T, N> for [T; N]
{
    fn zip<Z>(self, other: Z) -> [(T, Z::Elem); N]
    where
        Z: ArrayForm<N>
    {
        self.zip_with(other, const |x, y| (x, y))
    }
    fn zip_ref<Z>(&self, other: Z) -> [(&T, Z::Elem); N]
    where
        Z: ArrayForm<N>
    {
        self.zip_ref_with(other, const |x, y| (x, y))
    }
    fn zip_mut<Z>(&mut self, other: Z) -> [(&mut T, Z::Elem); N]
    where
        Z: ArrayForm<N>
    {
        self.zip_mut_with(other, const |x, y| (x, y))
    }
}