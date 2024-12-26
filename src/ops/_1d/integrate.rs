use core::ops::{AddAssign, SubAssign};

use array_trait::Array;

#[const_trait]
pub trait Integrate<T, const N: usize>: Array<Item = T>
{
    /// Integrates array (discrete calculus)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
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

impl<T, const N: usize> Integrate<T, N> for [T; N]
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