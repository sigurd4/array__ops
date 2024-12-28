use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;

use super::ArrayEnumerateVisit;

#[const_trait]
pub trait ArrayVisit<T, const N: usize>: Array<Item = T>
{
    /// Visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// let mut i = 0;
    /// 
    /// x.visit(|&e| {
    ///     i += 1;
    ///     assert_eq!(i, e)
    /// });
    /// ```
    fn visit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(&'a T) + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// x.visit_mut(|e| {
    ///     i += 1;
    ///     *e = i;
    /// });
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    fn visit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(&'a mut T) + ~const Destruct,
        T: 'a;
    fn visit_pin<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: FnMut(Pin<&'a T>) + ~const Destruct,
        T: 'a;
    fn visit_pin_mut<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: FnMut(Pin<&'a mut T>) + ~const Destruct,
        T: 'a;

    /// Visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_visit(|&e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     assert_eq!(i, e);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// ```
    fn try_visit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_visit_mut(|e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     *e = i;
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// assert_eq!(x, [1, 2, 3, 4, 0, 0, 0, 0])
    /// ```
    fn try_visit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_visit_pin<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(Pin<&'a T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_visit_pin_mut<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(Pin<&'a mut T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    /// Visits each element once, from right to left.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// let mut i = 0;
    /// 
    /// x.rvisit(|&e| {
    ///     i += 1;
    ///     assert_eq!(i, e)
    /// });
    /// ```
    fn rvisit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(&'a T) + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, from right to left.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// x.rvisit_mut(|e| {
    ///     i += 1;
    ///     *e = i;
    /// });
    /// 
    /// assert_eq!(x, [8, 7, 6, 5, 4, 3, 2, 1]);
    /// ```
    fn rvisit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(&'a mut T) + ~const Destruct,
        T: 'a;
    fn rvisit_pin<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: FnMut(Pin<&'a T>) + ~const Destruct,
        T: 'a;
    fn rvisit_pin_mut<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: FnMut(Pin<&'a mut T>) + ~const Destruct,
        T: 'a;
        
    /// Visits each element once, from right to left, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_rvisit(|&e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     assert_eq!(i, e);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// ```
    fn try_rvisit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, from right to left, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_rvisit_mut(|e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     *e = i;
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// assert_eq!(x, [0, 0, 0, 0, 4, 3, 2, 1])
    /// ```
    fn try_rvisit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_rvisit_pin<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(Pin<&'a T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_rvisit_pin_mut<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(Pin<&'a mut T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    /// Visits each element once, asyncronously.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// # tokio_test::block_on(async {
    /// x.visit_async(async |&e| {
    ///     assert_eq!(x[e - 1], e)
    /// }).await;
    /// # })
    /// ```
    async fn visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(&'a T) + ~const Destruct,
        T: 'a;
    /// Mutably visits each element once, asyncronously.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// # tokio_test::block_on(async {
    /// x.visit_mut_async(async |e| {
    ///     *e = 9 - *e
    /// }).await;
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// # })
    /// ```
    async fn visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(&'a mut T) + ~const Destruct,
        T: 'a;
    async fn visit_pin_async<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: AsyncFn(Pin<&'a T>) + ~const Destruct,
        T: 'a;
    async fn visit_pin_mut_async<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: AsyncFn(Pin<&'a mut T>) + ~const Destruct,
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
    /// 
    /// # tokio_test::block_on(async {
    /// let result = x.try_visit_async(async |&e| {
    ///     if e > 4
    ///     {
    ///         return Err(e)
    ///     }
    ///     assert_eq!(x[e - 1], e);
    ///     Ok(())
    /// }).await;
    /// 
    /// assert!(result == Err(5) || result == Err(6) || result == Err(7) || result == Err(8));
    /// # })
    /// ```
    async fn try_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a T) -> Result<(), E> + ~const Destruct,
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
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// # tokio_test::block_on(async {
    /// let result = x.try_visit_mut_async(async |e| {
    ///     if *e <= 4
    ///     {
    ///         return Err(*e)
    ///     }
    ///     *e = 9 - *e;
    ///     Ok(())
    /// }).await;
    /// 
    /// assert_eq!(x[..4], [1, 2, 3, 4]);
    /// assert!(x[4] == 5 || x[4] == 4);
    /// assert!(x[5] == 6 || x[5] == 3);
    /// assert!(x[6] == 7 || x[6] == 2);
    /// assert!(x[7] == 8 || x[7] == 1);
    /// # })
    /// ```
    async fn try_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a mut T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_visit_pin_async<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(Pin<&'a T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_visit_pin_mut_async<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(Pin<&'a mut T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayVisit<T, N> for [T; N]
{
    fn visit<'a, F>(&'a self, mut visitor: F)
    where
        F: FnMut(&'a T),
        T: 'a
    {
        self.enumerate_visit(|_, x| visitor(x))
    }
    fn visit_mut<'a, F>(&'a mut self, mut visitor: F)
    where
        F: FnMut(&'a mut T),
        T: 'a
    {
        self.enumerate_visit_mut(|_, x| visitor(x))
    }
    fn visit_pin<'a, F>(self: Pin<&'a Self>, mut visitor: F)
    where
        F: FnMut(Pin<&'a T>),
        T: 'a
    {
        self.enumerate_visit_pin(|_, x| visitor(x))
    }
    fn visit_pin_mut<'a, F>(self: Pin<&'a mut Self>, mut visitor: F)
    where
        F: FnMut(Pin<&'a mut T>),
        T: 'a
    {
        self.enumerate_visit_pin_mut(|_, x| visitor(x))
    }

    fn try_visit<'a, E, F>(&'a self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit(|_, x| visitor(x))
    }
    fn try_visit_mut<'a, E, F>(&'a mut self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_mut(|_, x| visitor(x))
    }
    fn try_visit_pin<'a, E, F>(self: Pin<&'a Self>, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(Pin<&'a T>) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_pin(|_, x| visitor(x))
    }
    fn try_visit_pin_mut<'a, E, F>(self: Pin<&'a mut Self>, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(Pin<&'a mut T>) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_pin_mut(|_, x| visitor(x))
    }
        
    fn rvisit<'a, F>(&'a self, mut visitor: F)
    where
        F: FnMut(&'a T),
        T: 'a
    {
        self.enumerate_rvisit(|_, x| visitor(x))
    }
    fn rvisit_mut<'a, F>(&'a mut self, mut visitor: F)
    where
        F: FnMut(&'a mut T),
        T: 'a
    {
        self.enumerate_rvisit_mut(|_, x| visitor(x))
    }
    fn rvisit_pin<'a, F>(self: Pin<&'a Self>, mut visitor: F)
    where
        F: FnMut(Pin<&'a T>),
        T: 'a
    {
        self.enumerate_rvisit_pin(|_, x| visitor(x))
    }
    fn rvisit_pin_mut<'a, F>(self: Pin<&'a mut Self>, mut visitor: F)
    where
        F: FnMut(Pin<&'a mut T>),
        T: 'a
    {
        self.enumerate_rvisit_pin_mut(|_, x| visitor(x))
    }

    fn try_rvisit<'a, E, F>(&'a self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rvisit(|_, x| visitor(x))
    }
    fn try_rvisit_mut<'a, E, F>(&'a mut self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rvisit_mut(|_, x| visitor(x))
    }
    fn try_rvisit_pin<'a, E, F>(self: Pin<&'a Self>, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(Pin<&'a T>) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rvisit_pin(|_, x| visitor(x))
    }
    fn try_rvisit_pin_mut<'a, E, F>(self: Pin<&'a mut Self>, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(Pin<&'a mut T>) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rvisit_pin_mut(|_, x| visitor(x))
    }
    
    async fn visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(&'a T),
        T: 'a
    {
        self.enumerate_visit_async(|_, x| visitor(x)).await
    }
    async fn visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(&'a mut T),
        T: 'a
    {
        self.enumerate_visit_mut_async(|_, x| visitor(x)).await
    }
    async fn visit_pin_async<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: AsyncFn(Pin<&'a T>),
        T: 'a
    {
        self.enumerate_visit_pin_async(|_, x| visitor(x)).await
    }
    async fn visit_pin_mut_async<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: AsyncFn(Pin<&'a mut T>),
        T: 'a
    {
        self.enumerate_visit_pin_mut_async(|_, x| visitor(x)).await
    }

    async fn try_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a T) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_async(|_, x| visitor(x)).await
    }
    async fn try_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_mut_async(|_, x| visitor(x)).await
    }
    async fn try_visit_pin_async<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(Pin<&'a T>) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_pin_async(|_, x| visitor(x)).await
    }
    async fn try_visit_pin_mut_async<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(Pin<&'a mut T>) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_pin_mut_async(|_, x| visitor(x)).await
    }
}