use core::pin::Pin;

use array_trait::Array;
use slice_ops::AsSlice;

use super::ArrayEnumerateMap;

#[const_trait]
pub trait ArrayEnumerate<T, const N: usize>: Array + AsSlice<Item = T>
{
    /// Enumerates each element in the array.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let a = ["zero", "one", "two", "three", "four", "five"];
    /// 
    /// let ea = a.enumerate();
    /// 
    /// assert_eq!(ea, [(0, "zero"), (1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five")]);
    /// ```
    fn enumerate(self) -> [(usize, T); N];
    /// Enumerates each element in the array passed as reference.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let a = ["zero", "one", "two", "three", "four", "five"];
    /// 
    /// let ea = a.enumerate_ref();
    /// 
    /// assert_eq!(ea, [(0, &"zero"), (1, &"one"), (2, &"two"), (3, &"three"), (4, &"four"), (5, &"five")]);
    /// ```
    fn enumerate_ref(&self) -> [(usize, &T); N];
    /// Enumerates each element in the array passed as mutable reference.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = ["zero", "one", "two", "three", "four", "five"];
    /// 
    /// let ea = a.enumerate_mut();
    /// 
    /// for (e, a) in ea
    /// {
    ///     *a = e.to_string().leak();
    /// }
    /// 
    /// assert_eq!(a, ["0", "1", "2", "3", "4", "5"]);
    /// ```
    fn enumerate_mut(&mut self) -> [(usize, &mut T); N];
    /// Enumerates each element in the array passed as pinned reference.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// use core::pin::Pin;
    /// 
    /// let a = Pin::new(&["zero", "one", "two", "three", "four", "five"]);
    /// 
    /// let ea = a.as_ref().enumerate_pin_ref();
    /// 
    /// assert_eq!(ea, [(0, Pin::new(&"zero")), (1, Pin::new(&"one")), (2, Pin::new(&"two")), (3, Pin::new(&"three")), (4, Pin::new(&"four")), (5, Pin::new(&"five"))]);
    /// ```
    fn enumerate_pin_ref(self: Pin<&Self>) -> [(usize, Pin<&T>); N];
    /// Enumerates each element in the array passed as mutable pinned reference.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// use core::pin::pin;
    /// 
    /// let mut a = pin!(["zero", "one", "two", "three", "four", "five"]);
    /// 
    /// let ea = a.as_mut().enumerate_pin_mut();
    /// 
    /// for (e, mut a) in ea
    /// {
    ///     *a = e.to_string().leak();
    /// }
    /// 
    /// assert_eq!(a, pin!(["0", "1", "2", "3", "4", "5"]));
    /// ```
    fn enumerate_pin_mut(self: Pin<&mut Self>) -> [(usize, Pin<&mut T>); N];
}

impl<T, const N: usize> ArrayEnumerate<T, N> for [T; N]
{
    fn enumerate(self) -> [(usize, T); N]
    {
        self.enumerate_map(|i, x| (i, x))
    }
    fn enumerate_ref(&self) -> [(usize, &T); N]
    {
        self.enumerate_map_ref(|i, x| (i, x))
    }
    fn enumerate_mut(&mut self) -> [(usize, &mut T); N]
    {
        self.enumerate_map_mut(|i, x| (i, x))
    }
    fn enumerate_pin_ref(self: Pin<&Self>) -> [(usize, Pin<&T>); N]
    {
        self.enumerate_map_pin_ref(|i, x| (i, x))
    }
    fn enumerate_pin_mut(self: Pin<&mut Self>) -> [(usize, Pin<&mut T>); N]
    {
        self.enumerate_map_pin_mut(|i, x| (i, x))
    }
}

#[cfg(test)]
mod test
{
    use super::ArrayEnumerate;

    #[test]
    fn test()
    {
        use core::pin::pin;

        let mut a = pin!(["zero", "one", "two", "three", "four", "five"]);
        
        let ea = a.as_mut().enumerate_pin_mut();
        
        for (e, mut a) in ea
        {
            *a = e.to_string().leak();
        }
        
        assert_eq!(a, pin!(["0", "1", "2", "3", "4", "5"]));
    }
}