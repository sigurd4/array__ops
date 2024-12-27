use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;

use crate::form::ArrayForm;

use super::EnumerateMeet;

#[const_trait]
pub trait Meet<T, const N: usize>: Array<Item = T>
{
    /// Visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let y = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.meet_each(y, |&a, b| {
    ///     assert_eq!(a, b)
    /// });
    /// ```
    fn meet_each<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [0; 8];
    /// let y = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.meet_each_mut(y, |a, b| {
    ///     *a += b
    /// });
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    fn meet_each_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn meet_each_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn meet_each_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a mut T>, Rhs::Elem) + ~const Destruct,
        T: 'a;

    /// Visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let y = [1, 2, 3, 4, -1, -2, -3, -4];
    /// 
    /// let result = x.try_meet_each(y, |&a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     assert_eq!(a, b);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(-1));
    /// ```
    fn try_meet_each<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [0; 8];
    /// let y = [1, 2, 3, 4, -1, -2, -3, -4];
    /// 
    /// let result = x.try_meet_each_mut(y, |a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     *a = b;
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(-1));
    /// assert_eq!(x, [1, 2, 3, 4, 0, 0, 0, 0])
    /// ```
    fn try_meet_each_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_meet_each_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_meet_each_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a mut T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    /// Visits each element once, from right to left.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// let y = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// x.rmeet_each(y, |&a, b| {
    ///     assert_eq!(a, b)
    /// });
    /// ```
    fn rmeet_each<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, from right to left.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [0; 8];
    /// let y = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// x.rmeet_each_mut(y, |a, b| {
    ///     *a = b;
    /// });
    /// 
    /// assert_eq!(x, [8, 7, 6, 5, 4, 3, 2, 1]);
    /// ```
    fn rmeet_each_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn rmeet_each_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn rmeet_each_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a mut T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
        
    /// Visits each element once, from right to left, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// let y = [-4, -3, -2, -1, 4, 3, 2, 1];
    /// 
    /// let result = x.try_rmeet_each(y, |&a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     assert_eq!(a, b);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(-1));
    /// ```
    fn try_rmeet_each<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, from right to left, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [0; 8];
    /// let y = [-4, -3, -2, -1, 4, 3, 2, 1];
    /// 
    /// let result = x.try_rmeet_each_mut(y, |a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     *a = b;
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(-1));
    /// assert_eq!(x, [0, 0, 0, 0, 4, 3, 2, 1])
    /// ```
    fn try_rmeet_each_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_rmeet_each_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_rmeet_each_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a mut T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    /// Visits each element once, asyncronously.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let y = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// # tokio_test::block_on(async {
    /// x.meet_each_async(y, async |&a, b| {
    ///     assert_eq!(x[a as usize - 1], a);
    ///     assert_eq!(y[b as usize - 1], b);
    ///     assert_eq!(a, b);
    /// }).await;
    /// # })
    /// ```
    async fn meet_each_async<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, asyncronously.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// let y = [-7, -5, -3, -1, 1, 3, 5, 7];
    /// 
    /// # tokio_test::block_on(async {
    /// x.meet_each_mut_async(y, async |a, b| {
    ///     *a += b
    /// }).await;
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// # })
    /// ```
    async fn meet_each_mut_async<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a mut T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    async fn meet_each_pin_async<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(Pin<&'a T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
    async fn meet_each_pin_mut_async<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(Pin<&'a mut T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
        
    /// Visits each element once, asyncronously, or short-circuits if visitor returns error.
    /// 
    /// # Warning
    /// 
    /// When any of the tasks return an error, all other tasks will be ignored. The tasks are not nessecarily stopped, and may still be running in the background.
    /// 
    /// If you want to wait for all tasks to complete, keep polling the future until it returns an [Ok](core::result::Result).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let y = [1, 2, 3, 4, -1, -2, -3, -4];
    /// 
    /// # tokio_test::block_on(async {
    /// let result = x.try_meet_each_async(y, async |&a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     assert_eq!(x[a as usize - 1], a);
    ///     assert_eq!(y[b as usize - 1], b);
    ///     assert_eq!(a, b);
    ///     Ok(())
    /// }).await;
    /// 
    /// assert!(result == Err(-1) || result == Err(-2) || result == Err(-3) || result == Err(-4));
    /// # })
    /// ```
    async fn try_meet_each_async<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, asyncronously, or short-circuits if visitor returns error.
    /// 
    /// # Warning
    /// 
    /// When any of the tasks return an error, all other tasks will be ignored. The tasks are not nessecarily stopped, and may still be running in the background.
    /// 
    /// If you want to wait for all tasks to complete, keep polling the future until it returns an [Ok](core::result::Result).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// let y = [-7, -5, -3, -1, 1, 3, 5, 7];
    /// 
    /// # tokio_test::block_on(async {
    /// let result = x.try_meet_each_mut_async(y, async |a, b| {
    ///     if b > 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     *a += b;
    ///     Ok(())
    /// }).await;
    /// 
    /// assert!(x[0] == 8 || x[0] == 1);
    /// assert!(x[1] == 7 || x[1] == 2);
    /// assert!(x[2] == 6 || x[2] == 3);
    /// assert!(x[3] == 5 || x[3] == 4);
    /// assert_eq!(x[4..], [4, 3, 2, 1]);
    /// assert!(result == Err(1) || result == Err(3) || result == Err(5) || result == Err(7));
    /// # })
    /// ```
    async fn try_meet_each_mut_async<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a mut T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_meet_each_pin_async<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(Pin<&'a T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_meet_each_pin_mut_async<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(Pin<&'a mut T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;

    fn meet_all<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(&'a T, Rhs) + ~const Destruct,
        T: 'a;
    fn meet_all_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(&'a mut T, Rhs) + ~const Destruct,
        T: 'a;
    fn meet_all_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a T>, Rhs) + ~const Destruct,
        T: 'a;
    fn meet_all_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a mut T>, Rhs) + ~const Destruct,
        T: 'a;
        
    fn try_meet_all<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(&'a T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_meet_all_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(&'a mut T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_meet_all_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_meet_all_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a mut T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    fn rmeet_all<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(&'a T, Rhs) + ~const Destruct,
        T: 'a;
    fn rmeet_all_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(&'a mut T, Rhs) + ~const Destruct,
        T: 'a;
    fn rmeet_all_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a T>, Rhs) + ~const Destruct,
        T: 'a;
    fn rmeet_all_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a mut T>, Rhs) + ~const Destruct,
        T: 'a;
        
    fn try_rmeet_all<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(&'a T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_rmeet_all_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(&'a mut T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_rmeet_all_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_rmeet_all_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a mut T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    async fn meet_all_async<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(&'a T, Rhs) + ~const Destruct,
        T: 'a;
    async fn meet_all_mut_async<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(&'a mut T, Rhs) + ~const Destruct,
        T: 'a;
    async fn meet_all_pin_async<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(Pin<&'a T>, Rhs) + ~const Destruct,
        T: 'a;
    async fn meet_all_pin_mut_async<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(Pin<&'a mut T>, Rhs) + ~const Destruct,
        T: 'a;

    async fn try_meet_all_async<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(&'a T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_meet_all_mut_async<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(&'a mut T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_meet_all_pin_async<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(Pin<&'a T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_meet_all_pin_mut_async<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(Pin<&'a mut T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> Meet<T, N> for [T; N]
{
    fn meet_each<'a, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem),
        T: 'a
    {
        self.enumerate_meet_each(rhs, |_, x, y| visitor(x, y))
    }
    fn meet_each_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem),
        T: 'a
    {
        self.enumerate_meet_each_mut(rhs, |_, x, y| visitor(x, y))
    }
    fn meet_each_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a T>, Rhs::Elem),
        T: 'a
    {
        self.enumerate_meet_each_pin(rhs, |_, x, y| visitor(x, y))
    }
    fn meet_each_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a mut T>, Rhs::Elem),
        T: 'a
    {
        self.enumerate_meet_each_pin_mut(rhs, |_, x, y| visitor(x, y))
    }

    fn try_meet_each<'a, E, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_each(rhs, |_, x, y| visitor(x, y))
    }
    fn try_meet_each_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_each_mut(rhs, |_, x, y| visitor(x, y))
    }
    fn try_meet_each_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_each_pin(rhs, |_, x, y| visitor(x, y))
    }
    fn try_meet_each_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a mut T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_each_pin_mut(rhs, |_, x, y| visitor(x, y))
    }
        
    fn rmeet_each<'a, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem),
        T: 'a
    {
        self.enumerate_rmeet_each(rhs, |_, x, y| visitor(x, y))
    }
    fn rmeet_each_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem),
        T: 'a
    {
        self.enumerate_rmeet_each_mut(rhs, |_, x, y| visitor(x, y))
    }
    fn rmeet_each_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a T>, Rhs::Elem),
        T: 'a
    {
        self.enumerate_rmeet_each_pin(rhs, |_, x, y| visitor(x, y))
    }
    fn rmeet_each_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a mut T>, Rhs::Elem),
        T: 'a
    {
        self.enumerate_rmeet_each_pin_mut(rhs, |_, x, y| visitor(x, y))
    }

    fn try_rmeet_each<'a, E, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rmeet_each(rhs, |_, x, y| visitor(x, y))
    }
    fn try_rmeet_each_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rmeet_each_mut(rhs, |_, x, y| visitor(x, y))
    }
    fn try_rmeet_each_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rmeet_each_pin(rhs, |_, x, y| visitor(x, y))
    }
    fn try_rmeet_each_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(Pin<&'a mut T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rmeet_each_pin_mut(rhs, |_, x, y| visitor(x, y))
    }
        
    async fn meet_each_async<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a T, Rhs::Elem),
        T: 'a
    {
        self.enumerate_meet_each_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn meet_each_mut_async<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a mut T, Rhs::Elem),
        T: 'a
    {
        self.enumerate_meet_each_mut_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn meet_each_pin_async<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(Pin<&'a T>, Rhs::Elem),
        T: 'a
    {
        self.enumerate_meet_each_pin_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn meet_each_pin_mut_async<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(Pin<&'a mut T>, Rhs::Elem),
        T: 'a
    {
        self.enumerate_meet_each_pin_mut_async(rhs, |_, x, y| visitor(x, y)).await
    }

    async fn try_meet_each_async<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_each_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn try_meet_each_mut_async<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a mut T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_each_mut_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn try_meet_each_pin_async<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(Pin<&'a T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_each_pin_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn try_meet_each_pin_mut_async<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(Pin<&'a mut T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_each_pin_mut_async(rhs, |_, x, y| visitor(x, y)).await
    }

    fn meet_all<'a, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(&'a T, Rhs),
        T: 'a
    {
        self.enumerate_meet_all(rhs, |_, x, y| visitor(x, y))
    }
    fn meet_all_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(&'a mut T, Rhs),
        T: 'a
    {
        self.enumerate_meet_all_mut(rhs, |_, x, y| visitor(x, y))
    }
    fn meet_all_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a T>, Rhs),
        T: 'a
    {
        self.enumerate_meet_all_pin(rhs, |_, x, y| visitor(x, y))
    }
    fn meet_all_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a mut T>, Rhs),
        T: 'a
    {
        self.enumerate_meet_all_pin_mut(rhs, |_, x, y| visitor(x, y))
    }

    fn try_meet_all<'a, E, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(&'a T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_all(rhs, |_, x, y| visitor(x, y))
    }
    fn try_meet_all_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(&'a mut T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_all_mut(rhs, |_, x, y| visitor(x, y))
    }
    fn try_meet_all_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_all_pin(rhs, |_, x, y| visitor(x, y))
    }
    fn try_meet_all_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a mut T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_all_pin_mut(rhs, |_, x, y| visitor(x, y))
    }
        
    fn rmeet_all<'a, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(&'a T, Rhs),
        T: 'a
    {
        self.enumerate_rmeet_all(rhs, |_, x, y| visitor(x, y))
    }
    fn rmeet_all_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(&'a mut T, Rhs),
        T: 'a
    {
        self.enumerate_rmeet_all_mut(rhs, |_, x, y| visitor(x, y))
    }
    fn rmeet_all_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a T>, Rhs),
        T: 'a
    {
        self.enumerate_rmeet_all_pin(rhs, |_, x, y| visitor(x, y))
    }
    fn rmeet_all_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a mut T>, Rhs),
        T: 'a
    {
        self.enumerate_rmeet_all_pin_mut(rhs, |_, x, y| visitor(x, y))
    }

    fn try_rmeet_all<'a, E, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(&'a T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rmeet_all(rhs, |_, x, y| visitor(x, y))
    }
    fn try_rmeet_all_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(&'a mut T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rmeet_all_mut(rhs, |_, x, y| visitor(x, y))
    }
    fn try_rmeet_all_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rmeet_all_pin(rhs, |_, x, y| visitor(x, y))
    }
    fn try_rmeet_all_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(Pin<&'a mut T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rmeet_all_pin_mut(rhs, |_, x, y| visitor(x, y))
    }
        
    async fn meet_all_async<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(&'a T, Rhs),
        T: 'a
    {
        self.enumerate_meet_all_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn meet_all_mut_async<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(&'a mut T, Rhs),
        T: 'a
    {
        self.enumerate_meet_all_mut_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn meet_all_pin_async<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(Pin<&'a T>, Rhs),
        T: 'a
    {
        self.enumerate_meet_all_pin_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn meet_all_pin_mut_async<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(Pin<&'a mut T>, Rhs),
        T: 'a
    {
        self.enumerate_meet_all_pin_mut_async(rhs, |_, x, y| visitor(x, y)).await
    }

    async fn try_meet_all_async<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(&'a T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_all_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn try_meet_all_mut_async<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(&'a mut T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_all_mut_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn try_meet_all_pin_async<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(Pin<&'a T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_all_pin_async(rhs, |_, x, y| visitor(x, y)).await
    }
    async fn try_meet_all_pin_mut_async<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(Pin<&'a mut T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_meet_all_pin_mut_async(rhs, |_, x, y| visitor(x, y)).await
    }
}

#[cfg(test)]
mod test
{
    use crate::ops::Meet;

    #[test]
    fn it_works()
    {        
        let x = [1, 2, 3, 4, 5, 6, 7, 8];
        let y = [1, 2, 3, 4, -1, -2, -3, -4];
        
        tokio_test::block_on(async {
        let result = x.try_meet_each_async(y, async |&a, b| {
            if b < 0
            {
                return Err(b)
            }
            assert_eq!(x[a as usize - 1], a);
            assert_eq!(y[b as usize - 1], b);
            assert_eq!(a, b);
            Ok(())
        }).await;
        
        assert!(result == Err(-1) || result == Err(-2) || result == Err(-3) || result == Err(-4));
        })
    }
}