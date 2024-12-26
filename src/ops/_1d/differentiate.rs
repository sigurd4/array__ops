use core::ops::SubAssign;

use array_trait::Array;

#[const_trait]
pub trait Differentiate<T, const N: usize>: Array<Item = T>
{
    /// Differentiates array (discrete calculus)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut a = [1, 2, 3];
    /// 
    /// a.differentiate();
    /// 
    /// assert_eq!(a, [1, 2 - 1, 3 - 2]);
    /// ```
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy;
}

impl<T, const N: usize> Differentiate<T, N> for [T; N]
{
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy
    {
        // TODO: visit_windowed_mut
        if N > 0
        {
            let mut i = N - 1;
            while i > 0
            {
                self[i] -= self[i - 1];
                i -= 1;
            }
        }
    }
}