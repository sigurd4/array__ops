use core::cmp::Ordering;

use super::ArrayArgReduce;

#[const_trait]
pub trait ArrayArgMinMax<T, const N: usize>: ArrayArgReduce<T, N>
{
    /// Finds the index of the maximum value in the slice.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// //                v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let i = x.argmax().unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    /// Finds the index of the minimum value in the slice.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// //                              v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let i = x.argmin().unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    /// Finds the index of the maximum value in the slice, given a comparison predicate.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// //                v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let f = Ord::cmp;
    /// 
    /// let i = x.argmax_by(f).unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering /*+ ~const Destruct*/,
        T: 'a;
    /// Finds the index of the minimum value in the slice, given a comparison predicate.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// //                              v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let f = Ord::cmp;
    /// 
    /// let i = x.argmin_by(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering /*+ ~const Destruct*/,
        T: 'a;
    /// Finds the index of the maximum key in the slice, given a hashing function.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// //                       v
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// let f = |&e| i32::from_str_radix(e, 10).unwrap();
    /// 
    /// let i = x.argmax_by_key(f).unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialOrd,
        T: 'a;
    /// Finds the index of the minimum key in the slice, given a hashing function.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// //                                  v
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// let f = |&e| i32::from_str_radix(e, 10).unwrap();
    /// 
    /// let i = x.argmin_by_key(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialOrd,
        T: 'a;
}

impl<T, const N: usize> ArrayArgMinMax<T, N> for [T; N]
{
    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd<T>
    {
        self.argreduce(PartialOrd::gt)
    }
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd<T>
    {
        self.argreduce(PartialOrd::lt)
    }
    fn argmax_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering,
        T: 'a
    {
        self.argreduce(|a, b| matches!(f(a, b), Ordering::Greater))
    }
    fn argmin_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering,
        T: 'a
    {
        self.argreduce(|a, b| matches!(f(a, b), Ordering::Less))
    }
    fn argmax_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B,
        B: PartialOrd,
        T: 'a
    {
        self.argreduce_key(PartialOrd::gt, f)
    }
    fn argmin_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B,
        B: PartialOrd,
        T: 'a
    {
        self.argreduce_key(PartialOrd::lt, f)
    }
}