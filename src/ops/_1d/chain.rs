use core::mem::MaybeUninit;

use array_trait::Array;

use super::Split;

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
        /*unsafe {
            private::merge_transmute(self, rhs)
        }*/
        let mut chain = MaybeUninit::uninit_array();
        let (left, right) = chain.rsplit_mut_ptr(M);
        unsafe {
            core::ptr::copy_nonoverlapping(self.as_ptr(), left.cast(), N);
            core::ptr::copy_nonoverlapping(rhs.as_ptr(), right.cast(), M);
        }
        core::mem::forget(self);
        core::mem::forget(rhs);
        unsafe {
            MaybeUninit::array_assume_init(chain)
        }
    }
    
    fn rchain<const M: usize>(self, lhs: [T; M]) -> [T; N + M]
    {
        /*unsafe {
            private::merge_transmute(rhs, self)
        }*/
        let mut chain = MaybeUninit::uninit_array();
        let (left, right) = chain.split_mut_ptr(M);
        unsafe {
            core::ptr::copy_nonoverlapping(lhs.as_ptr(), left.cast(), M);
            core::ptr::copy_nonoverlapping(self.as_ptr(), right.cast(), N);
        }
        core::mem::forget(lhs);
        core::mem::forget(self);
        unsafe {
            MaybeUninit::array_assume_init(chain)
        }
    }
}