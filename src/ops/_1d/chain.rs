use array_trait::Array;

use crate::private;

#[const_trait]
pub trait ArrayChain<T, const N: usize>: Array<Item = T>
{
    /// Chains two arrays with the same item together.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let a = ["one", "two"];
    /// let b = ["three"];
    /// 
    /// assert_eq!(a.chain(b), ["one", "two", "three"]);
    /// ```
    fn chain<const M: usize>(self, rhs: [T; M]) -> [T; N + M];

    /// Chains two arrays with the same item together in reverse.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let a = ["two", "three"];
    /// let b = ["one"];
    /// 
    /// assert_eq!(a.rchain(b), ["one", "two", "three"]);
    /// ```
    fn rchain<const M: usize>(self, lhs: [T; M]) -> [T; N + M];
}

impl<T, const N: usize> const ArrayChain<T, N> for [T; N]
{
    
    fn chain<const M: usize>(self, rhs: [T; M]) -> [T; N + M]
    {
        unsafe {
            private::merge_transmute(self, rhs)
        }
    }
    
    fn rchain<const M: usize>(self, lhs: [T; M]) -> [T; N + M]
    {
        unsafe {
            private::merge_transmute(lhs, self)
        }
    }
}