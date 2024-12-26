use array_trait::Array;

use core::ops::AsyncFn;

#[const_trait]
pub trait Find<T, const N: usize>: Array<Item = T>
{
    /// Performs a linear search for the first value that equals `x`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                   v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let i = x.find(&5).unwrap();
    /// 
    /// assert_eq!(i, 4);
    /// assert_eq!(x[i], 5);
    /// ```
    fn find(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    /// Performs a linear search for the first value that satisfies the given predicate.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                      v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn > 5; 
    /// 
    /// let i = x.find_by(f).unwrap();
    /// 
    /// assert_eq!(i, 5);
    /// ```
    fn find_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
    /// Performs a linear search for the first value that matches the given key given a hashing function.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //             v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn % 2;
    /// 
    /// let i = x.find_by_key(&0, f).unwrap();
    /// 
    /// assert_eq!(i, 2);
    /// ```
    fn find_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a;
        
    /// Performs a linear search from the right for the first value that equals `x`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                               v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let i = x.rfind(&5).unwrap();
    /// 
    /// assert_eq!(i, 8);
    /// assert_eq!(x[i], 5);
    /// ```
    fn rfind(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    /// Performs a linear search from the right for the first value that satisfies the given predicate.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                            v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn > 5;
    /// 
    /// let i = x.rfind_by(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn rfind_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
    /// Performs a linear search from the right for the first value that matches the given key given a hashing function.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                            v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn % 2;
    /// 
    /// let i = x.rfind_by_key(&0, f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn rfind_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a;

    fn find_async(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    fn find_by_async<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: AsyncFn(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
    fn find_by_key_async<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: AsyncFn(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a;
}

impl<T, const N: usize> Find<T, N> for [T; N]
{
    fn find(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        self.as_slice().find(x)
    }
    fn find_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a
    {
        self.as_slice().find_by(f)
    }
    fn find_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a
    {
        self.as_slice().find_by_key(b, f)
    }
        
    fn rfind(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        self.as_slice().rfind(x)
    }
    fn rfind_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a
    {
        self.as_slice().rfind_by(f)
    }
    fn rfind_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a
    {
        self.as_slice().rfind_by_key(b, f)
    }

    fn find_async(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        todo!()
    }
    fn find_by_async<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: AsyncFn(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a
    {
        todo!()
    }
    fn find_by_key_async<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: AsyncFn(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a
    {
        todo!()
    }
}