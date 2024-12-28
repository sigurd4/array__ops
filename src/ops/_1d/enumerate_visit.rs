use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;

use super::{ArrayJoin, ArrayEnumerateMap};

#[const_trait]
pub trait ArrayEnumerateVisit<T, const N: usize>: Array<Item = T>
{
    fn enumerate_visit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(usize, &'a T) + ~const Destruct,
        T: 'a;
    fn enumerate_visit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(usize, &'a mut T) + ~const Destruct,
        T: 'a;
    fn enumerate_visit_pin<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: FnMut(usize, Pin<&'a T>) + ~const Destruct,
        T: 'a;
    fn enumerate_visit_pin_mut<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: FnMut(usize, Pin<&'a mut T>) + ~const Destruct,
        T: 'a;

    fn try_enumerate_visit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, &'a T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_visit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, &'a mut T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_visit_pin<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, Pin<&'a T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_visit_pin_mut<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, Pin<&'a mut T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    fn enumerate_rvisit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(usize, &'a T) + ~const Destruct,
        T: 'a;
    fn enumerate_rvisit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(usize, &'a mut T) + ~const Destruct,
        T: 'a;
    fn enumerate_rvisit_pin<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: FnMut(usize, Pin<&'a T>) + ~const Destruct,
        T: 'a;
    fn enumerate_rvisit_pin_mut<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: FnMut(usize, Pin<&'a mut T>) + ~const Destruct,
        T: 'a;
        
    fn try_enumerate_rvisit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, &'a T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rvisit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, &'a mut T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rvisit_pin<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, Pin<&'a T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rvisit_pin_mut<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, Pin<&'a mut T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    async fn enumerate_visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(usize, &'a T) + ~const Destruct,
        T: 'a;
    async fn enumerate_visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(usize, &'a mut T) + ~const Destruct,
        T: 'a;
    async fn enumerate_visit_pin_async<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: AsyncFn(usize, Pin<&'a T>) + ~const Destruct,
        T: 'a;
    async fn enumerate_visit_pin_mut_async<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: AsyncFn(usize, Pin<&'a mut T>) + ~const Destruct,
        T: 'a;

    async fn try_enumerate_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(usize, &'a T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(usize, &'a mut T) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_visit_pin_async<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(usize, Pin<&'a T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_visit_pin_mut_async<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(usize, Pin<&'a mut T>) -> Result<(), E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayEnumerateVisit<T, N> for [T; N]
{
    fn enumerate_visit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(usize, &'a T),
        T: 'a
    {
        r#impl::enumerate_visit(&self, visitor)
    }
    fn enumerate_visit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(usize, &'a mut T),
        T: 'a
    {
        r#impl::enumerate_visit(&self, visitor)
    }
    fn enumerate_visit_pin<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: FnMut(usize, Pin<&'a T>),
        T: 'a
    {
        r#impl::enumerate_visit(&self, visitor)
    }
    fn enumerate_visit_pin_mut<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: FnMut(usize, Pin<&'a mut T>),
        T: 'a
    {
        r#impl::enumerate_visit(&self, visitor)
    }

    fn try_enumerate_visit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, &'a T) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_visit(&self, visitor)
    }
    fn try_enumerate_visit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, &'a mut T) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_visit(&self, visitor)
    }
    fn try_enumerate_visit_pin<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, Pin<&'a T>) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_visit(&self, visitor)
    }
    fn try_enumerate_visit_pin_mut<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, Pin<&'a mut T>) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_visit(&self, visitor)
    }
        
    fn enumerate_rvisit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(usize, &'a T),
        T: 'a
    {
        r#impl::enumerate_rvisit(&self, visitor)
    }
    fn enumerate_rvisit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(usize, &'a mut T),
        T: 'a
    {
        r#impl::enumerate_rvisit(&self, visitor)
    }
    fn enumerate_rvisit_pin<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: FnMut(usize, Pin<&'a T>),
        T: 'a
    {
        r#impl::enumerate_rvisit(&self, visitor)
    }
    fn enumerate_rvisit_pin_mut<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: FnMut(usize, Pin<&'a mut T>),
        T: 'a
    {
        r#impl::enumerate_rvisit(&self, visitor)
    }
        
    fn try_enumerate_rvisit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, &'a T) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_rvisit(&self, visitor)
    }
    fn try_enumerate_rvisit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, &'a mut T) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_rvisit(&self, visitor)
    }
    fn try_enumerate_rvisit_pin<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, Pin<&'a T>) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_rvisit(&self, visitor)
    }
    fn try_enumerate_rvisit_pin_mut<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: FnMut(usize, Pin<&'a mut T>) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_rvisit(&self, visitor)
    }
    
    async fn enumerate_visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(usize, &'a T),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_ref(|i, x| visitor(i, x)).join_actions().await
    }
    async fn enumerate_visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(usize, &'a mut T),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_mut(|i, x| visitor(i, x)).join_actions().await
    }
    async fn enumerate_visit_pin_async<'a, F>(self: Pin<&'a Self>, visitor: F)
    where
        F: AsyncFn(usize, Pin<&'a T>),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_pin_ref(|i, x| visitor(i, x)).join_actions().await
    }
    async fn enumerate_visit_pin_mut_async<'a, F>(self: Pin<&'a mut Self>, visitor: F)
    where
        F: AsyncFn(usize, Pin<&'a mut T>),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_pin_mut(|i, x| visitor(i, x)).join_actions().await
    }

    async fn try_enumerate_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(usize, &'a T) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_ref(|i, x| visitor(i, x)).try_join_actions().await
    }
    async fn try_enumerate_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(usize, &'a mut T) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_mut(|i, x| visitor(i, x)).try_join_actions().await
    }
    async fn try_enumerate_visit_pin_async<'a, E, F>(self: Pin<&'a Self>, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(usize, Pin<&'a T>) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_pin_ref(|i, x| visitor(i, x)).try_join_actions().await
    }
    async fn try_enumerate_visit_pin_mut_async<'a, E, F>(self: Pin<&'a mut Self>, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(usize, Pin<&'a mut T>) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_pin_mut(|i, x| visitor(i, x)).try_join_actions().await
    }
}

mod r#impl
{
    use crate::form::ArrayForm;

    pub(super) fn enumerate_visit<const N: usize, A, F>(array: &A, mut visitor: F)
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem)
    {
        let mut i = 0;
        while i < N
        {
            visitor(
                i,
                unsafe {
                    array.read_elem(i)
                }
            );
            i += 1
        }
    }
    pub(super) fn enumerate_rvisit<const N: usize, A, F>(array: &A, mut visitor: F)
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem)
    {
        let mut i = N;
        while i > 0
        {
            i -= 1;
            visitor(
                i,
                unsafe {
                    array.read_elem(i)
                }
            )
        }
    }

    pub(super) fn try_enumerate_visit<const N: usize, A, F, E>(array: &A, mut visitor: F) -> Result<(), E>
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem) -> Result<(), E>
    {
        let mut i = 0;
        while i < N
        {
            visitor(
                i,
                unsafe {
                    array.read_elem(i)
                }
            )?;
            i += 1
        }
        Ok(())
    }
    pub(super) fn try_enumerate_rvisit<const N: usize, A, F, E>(array: &A, mut visitor: F) -> Result<(), E>
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem) -> Result<(), E>
    {
        let mut i = N;
        while i > 0
        {
            i -= 1;
            visitor(
                i,
                unsafe {
                    array.read_elem(i)
                }
            )?
        }
        Ok(())
    }
}