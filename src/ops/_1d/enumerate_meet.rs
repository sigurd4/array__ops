use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;
use slice_ops::AsSlice;

use crate::{private::guard, form::ArrayForm};

use self::guard::PartialEmptyGuard;

use super::{ArrayJoin, ArrayEnumerateVisit, ArrayEnumerateZipWith};

#[const_trait]
pub trait ArrayEnumerateMeet<T, const N: usize>: Array + AsSlice<Item = T>
{
    fn enumerate_meet_each<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn enumerate_meet_each_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a mut T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn enumerate_meet_each_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn enumerate_meet_each_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a mut T>, Rhs::Elem) + ~const Destruct,
        T: 'a;

    fn try_enumerate_meet_each<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_meet_each_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a mut T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_meet_each_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_meet_each_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    fn enumerate_rmeet_each<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn enumerate_rmeet_each_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a mut T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn enumerate_rmeet_each_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
    fn enumerate_rmeet_each_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a mut T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
        
    fn try_enumerate_rmeet_each<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rmeet_each_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a mut T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rmeet_each_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rmeet_each_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    async fn enumerate_meet_each_async<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, &'a T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    async fn enumerate_meet_each_mut_async<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, &'a mut T, Rhs::Elem) + ~const Destruct,
        T: 'a;
    async fn enumerate_meet_each_pin_async<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, Pin<&'a T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
    async fn enumerate_meet_each_pin_mut_async<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, Pin<&'a mut T>, Rhs::Elem) + ~const Destruct,
        T: 'a;
        
    async fn try_enumerate_meet_each_async<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, &'a T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_meet_each_mut_async<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, &'a mut T, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_meet_each_pin_async<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, Pin<&'a T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_meet_each_pin_mut_async<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<(), E> + ~const Destruct,
        T: 'a;

    fn enumerate_meet_all<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, &'a T, Rhs) + ~const Destruct,
        T: 'a;
    fn enumerate_meet_all_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, &'a mut T, Rhs) + ~const Destruct,
        T: 'a;
    fn enumerate_meet_all_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a T>, Rhs) + ~const Destruct,
        T: 'a;
    fn enumerate_meet_all_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a mut T>, Rhs) + ~const Destruct,
        T: 'a;
        
    fn try_enumerate_meet_all<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, &'a T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_meet_all_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, &'a mut T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_meet_all_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_meet_all_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a mut T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    fn enumerate_rmeet_all<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, &'a T, Rhs) + ~const Destruct,
        T: 'a;
    fn enumerate_rmeet_all_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, &'a mut T, Rhs) + ~const Destruct,
        T: 'a;
    fn enumerate_rmeet_all_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a T>, Rhs) + ~const Destruct,
        T: 'a;
    fn enumerate_rmeet_all_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a mut T>, Rhs) + ~const Destruct,
        T: 'a;
        
    fn try_enumerate_rmeet_all<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, &'a T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rmeet_all_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, &'a mut T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rmeet_all_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rmeet_all_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a mut T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
        
    async fn enumerate_meet_all_async<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(usize, &'a T, Rhs) + ~const Destruct,
        T: 'a;
    async fn enumerate_meet_all_mut_async<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(usize, &'a mut T, Rhs) + ~const Destruct,
        T: 'a;
    async fn enumerate_meet_all_pin_async<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(usize, Pin<&'a T>, Rhs) + ~const Destruct,
        T: 'a;
    async fn enumerate_meet_all_pin_mut_async<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(usize, Pin<&'a mut T>, Rhs) + ~const Destruct,
        T: 'a;

    async fn try_enumerate_meet_all_async<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(usize, &'a T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_meet_all_mut_async<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(usize, &'a mut T, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_meet_all_pin_async<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(usize, Pin<&'a T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_meet_all_pin_mut_async<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(usize, Pin<&'a mut T>, Rhs) -> Result<(), E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayEnumerateMeet<T, N> for [T; N]
{
    fn enumerate_meet_each<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a T, Rhs::Elem),
        T: 'a
    {
        r#impl::enumerate_meet_each(&self, rhs, visitor)
    }
    fn enumerate_meet_each_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a mut T, Rhs::Elem),
        T: 'a
    {
        r#impl::enumerate_meet_each(&self, rhs, visitor)
    }
    fn enumerate_meet_each_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a T>, Rhs::Elem),
        T: 'a
    {
        r#impl::enumerate_meet_each(&self, rhs, visitor)
    }
    fn enumerate_meet_each_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a mut T>, Rhs::Elem),
        T: 'a
    {
        r#impl::enumerate_meet_each(&self, rhs, visitor)
    }

    fn try_enumerate_meet_each<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_meet_each(&self, rhs, visitor)
    }
    fn try_enumerate_meet_each_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a mut T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_meet_each(&self, rhs, visitor)
    }
    fn try_enumerate_meet_each_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_meet_each(&self, rhs, visitor)
    }
    fn try_enumerate_meet_each_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_meet_each(&self, rhs, visitor)
    }
        
    fn enumerate_rmeet_each<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a T, Rhs::Elem),
        T: 'a
    {
        r#impl::enumerate_rmeet_each(&self, rhs, visitor)
    }
    fn enumerate_rmeet_each_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a mut T, Rhs::Elem),
        T: 'a
    {
        r#impl::enumerate_rmeet_each(&self, rhs, visitor)
    }
    fn enumerate_rmeet_each_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a T>, Rhs::Elem),
        T: 'a
    {
        r#impl::enumerate_rmeet_each(&self, rhs, visitor)
    }
    fn enumerate_rmeet_each_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a mut T>, Rhs::Elem),
        T: 'a
    {
        r#impl::enumerate_rmeet_each(&self, rhs, visitor)
    }
        
    fn try_enumerate_rmeet_each<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_rmeet_each(&self, rhs, visitor)
    }
    fn try_enumerate_rmeet_each_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, &'a mut T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_rmeet_each(&self, rhs, visitor)
    }
    fn try_enumerate_rmeet_each_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_rmeet_each(&self, rhs, visitor)
    }
    fn try_enumerate_rmeet_each_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        r#impl::try_enumerate_rmeet_each(&self, rhs, visitor)
    }
        
    async fn enumerate_meet_each_async<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, &'a T, Rhs::Elem),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_zip_ref_with(rhs, |i, x, y| visitor(i, x, y)).join_actions().await
    }
    async fn enumerate_meet_each_mut_async<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, &'a mut T, Rhs::Elem),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_zip_mut_with(rhs, |i, x, y| visitor(i, x, y)).join_actions().await
    }
    async fn enumerate_meet_each_pin_async<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, Pin<&'a T>, Rhs::Elem),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_zip_pin_ref_with(rhs, |i, x, y| visitor(i, x, y)).join_actions().await
    }
    async fn enumerate_meet_each_pin_mut_async<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, Pin<&'a mut T>, Rhs::Elem),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_zip_pin_mut_with(rhs, |i, x, y| visitor(i, x, y)).join_actions().await
    }
        
    async fn try_enumerate_meet_each_async<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, &'a T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_zip_ref_with(rhs, |i, x, y| visitor(i, x, y)).try_join_actions().await
    }
    async fn try_enumerate_meet_each_mut_async<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, &'a mut T, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_zip_mut_with(rhs, |i, x, y| visitor(i, x, y)).try_join_actions().await
    }
    async fn try_enumerate_meet_each_pin_async<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, Pin<&'a T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_zip_pin_ref_with(rhs, |i, x, y| visitor(i, x, y)).try_join_actions().await
    }
    async fn try_enumerate_meet_each_pin_mut_async<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_zip_pin_mut_with(rhs, |i, x, y| visitor(i, x, y)).try_join_actions().await
    }

    fn enumerate_meet_all<'a, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, &'a T, Rhs),
        T: 'a
    {
        self.enumerate_visit(|i, x| visitor(i, x, rhs))
    }
    fn enumerate_meet_all_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, &'a mut T, Rhs),
        T: 'a
    {
        self.enumerate_visit_mut(|i, x| visitor(i, x, rhs))
    }
    fn enumerate_meet_all_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a T>, Rhs),
        T: 'a
    {
        self.enumerate_visit_pin(|i, x| visitor(i, x, rhs))
    }
    fn enumerate_meet_all_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a mut T>, Rhs),
        T: 'a
    {
        self.enumerate_visit_pin_mut(|i, x| visitor(i, x, rhs))
    }
        
    fn try_enumerate_meet_all<'a, E, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, &'a T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit(|i, x| visitor(i, x, rhs))
    }
    fn try_enumerate_meet_all_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, &'a mut T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_mut(|i, x| visitor(i, x, rhs))
    }
    fn try_enumerate_meet_all_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_pin(|i, x| visitor(i, x, rhs))
    }
    fn try_enumerate_meet_all_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a mut T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_pin_mut(|i, x| visitor(i, x, rhs))
    }
        
    fn enumerate_rmeet_all<'a, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, &'a T, Rhs),
        T: 'a
    {
        self.enumerate_rvisit(|i, x| visitor(i, x, rhs))
    }
    fn enumerate_rmeet_all_mut<'a, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, &'a mut T, Rhs),
        T: 'a
    {
        self.enumerate_rvisit_mut(|i, x| visitor(i, x, rhs))
    }
    fn enumerate_rmeet_all_pin<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a T>, Rhs),
        T: 'a
    {
        self.enumerate_rvisit_pin(|i, x| visitor(i, x, rhs))
    }
    fn enumerate_rmeet_all_pin_mut<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F)
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a mut T>, Rhs),
        T: 'a
    {
        self.enumerate_rvisit_pin_mut(|i, x| visitor(i, x, rhs))
    }
        
    fn try_enumerate_rmeet_all<'a, E, F, Rhs>(&'a self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, &'a T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rvisit(|i, x| visitor(i, x, rhs))
    }
    fn try_enumerate_rmeet_all_mut<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, &'a mut T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rvisit_mut(|i, x| visitor(i, x, rhs))
    }
    fn try_enumerate_rmeet_all_pin<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rvisit_pin(|i, x| visitor(i, x, rhs))
    }
    fn try_enumerate_rmeet_all_pin_mut<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: FnMut(usize, Pin<&'a mut T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_rvisit_pin_mut(|i, x| visitor(i, x, rhs))
    }
        
    async fn enumerate_meet_all_async<'a, F, Rhs>(&'a self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(usize, &'a T, Rhs),
        T: 'a
    {
        self.enumerate_visit_async(|i, x| visitor(i, x, rhs)).await
    }
    async fn enumerate_meet_all_mut_async<'a, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(usize, &'a mut T, Rhs),
        T: 'a
    {
        self.enumerate_visit_mut_async(|i, x| visitor(i, x, rhs)).await
    }
    async fn enumerate_meet_all_pin_async<'a, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(usize, Pin<&'a T>, Rhs),
        T: 'a
    {
        self.enumerate_visit_pin_async(|i, x| visitor(i, x, rhs)).await
    }
    async fn enumerate_meet_all_pin_mut_async<'a, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F)
    where
        Rhs: Copy,
        F: AsyncFn(usize, Pin<&'a mut T>, Rhs),
        T: 'a
    {
        self.enumerate_visit_pin_mut_async(|i, x| visitor(i, x, rhs)).await
    }

    async fn try_enumerate_meet_all_async<'a, E, F, Rhs>(&'a self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(usize, &'a T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_async(|i, x| visitor(i, x, rhs)).await
    }
    async fn try_enumerate_meet_all_mut_async<'a, E, F, Rhs>(&'a mut self, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(usize, &'a mut T, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_mut_async(|i, x| visitor(i, x, rhs)).await
    }
    async fn try_enumerate_meet_all_pin_async<'a, E, F, Rhs>(self: Pin<&'a Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(usize, Pin<&'a T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_pin_async(|i, x| visitor(i, x, rhs)).await
    }
    async fn try_enumerate_meet_all_pin_mut_async<'a, E, F, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: Copy,
        F: AsyncFn(usize, Pin<&'a mut T>, Rhs) -> Result<(), E>,
        T: 'a
    {
        self.try_enumerate_visit_pin_mut_async(|i, x| visitor(i, x, rhs)).await
    }
}

mod r#impl
{
    use crate::form::ArrayForm;

    use super::{guard::Dir, PartialEmptyGuard};

    pub(super) fn enumerate_meet_each<const N: usize, Lhs, Rhs, F>(lhs: &Lhs, rhs: Rhs, visitor: F)
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Lhs::Elem, Rhs::Elem)
    {
        enumerate_dmeet_each::<{Dir::Left}, _, _, _, _>(lhs, rhs, visitor)
    }
    pub(super) fn enumerate_rmeet_each<const N: usize, Lhs, Rhs, F>(lhs: &Lhs, rhs: Rhs, visitor: F)
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Lhs::Elem, Rhs::Elem)
    {
        enumerate_dmeet_each::<{Dir::Right}, _, _, _, _>(lhs, rhs, visitor)
    }
    fn enumerate_dmeet_each<const D: Dir, const N: usize, Lhs, Rhs, F>(lhs: &Lhs, rhs: Rhs, mut visitor: F)
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Lhs::Elem, Rhs::Elem)
    {
        let mut guard = PartialEmptyGuard::<Rhs, D, N>::new(rhs);
        while guard.more()
        {
            let i = guard.index();
            visitor(
                i,
                unsafe {
                    lhs.read_elem(i)
                },
                guard.pop()
            )
        }
        guard.done()
    }

    pub(super) fn try_enumerate_meet_each<const N: usize, Lhs, Rhs, F, E>(lhs: &Lhs, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Lhs::Elem, Rhs::Elem) -> Result<(), E>
    {
        try_enumerate_dmeet_each::<{Dir::Left}, _, _, _, _, _>(lhs, rhs, visitor)
    }
    pub(super) fn try_enumerate_rmeet_each<const N: usize, Lhs, Rhs, F, E>(lhs: &Lhs, rhs: Rhs, visitor: F) -> Result<(), E>
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Lhs::Elem, Rhs::Elem) -> Result<(), E>
    {
        try_enumerate_dmeet_each::<{Dir::Right}, _, _, _, _, _>(lhs, rhs, visitor)
    }
    fn try_enumerate_dmeet_each<const D: Dir, const N: usize, Lhs, Rhs, F, E>(lhs: &Lhs, rhs: Rhs, mut visitor: F) -> Result<(), E>
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        F: FnMut(usize, Lhs::Elem, Rhs::Elem) -> Result<(), E>
    {
        let mut guard = PartialEmptyGuard::<Rhs, D, N>::new(rhs);
        while guard.more()
        {
            let i = guard.index();
            visitor(
                i,
                unsafe {
                    lhs.read_elem(i)
                },
                guard.pop()
            )?
        }
        guard.done();
        Ok(())
    }
}