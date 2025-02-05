use core::ops::AddAssign;

use array_trait::Array;
use slice_ops::AsSlice;

#[const_trait]
pub trait ArrayIntegrate<T, const N: usize>: Array + AsSlice<Item = T>
{
    /// Integrates array (discrete calculus)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [1, 2, 3];
    /// 
    /// a.integrate();
    /// 
    /// assert_eq!(a, [1, 1 + 2, 1 + 2 + 3])
    /// ```
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy;
}

impl<T, const N: usize> ArrayIntegrate<T, N> for [T; N]
{
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy
    {
        let mut i = 1;
        while i < N
        {
            self[i] += self[i - 1];
            i += 1;
        }
    }
}