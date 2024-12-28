use core::{marker::Destruct, ops::AsyncFn};

use array_trait::Array;

use crate::{join::{Actions, TryActions}, private::guard::PartialEmptyGuard};

use super::ArrayEnumerateMap;

#[const_trait]
pub trait ArrayEnumerateForEach<T, const N: usize>: Array<Item = T>
{
    fn enumerate_for_each<F>(self, action: F)
    where
        F: FnMut(usize, T) + ~const Destruct;
    fn try_enumerate_for_each<F, E>(self, action: F) -> Result<(), E>
    where
        F: FnMut(usize, T) -> Result<(), E> + ~const Destruct;

    fn enumerate_rfor_each<F>(self, action: F)
    where
        F: FnMut(usize, T) + ~const Destruct;
    fn try_enumerate_rfor_each<F, E>(self, action: F) -> Result<(), E>
    where
        F: FnMut(usize, T) -> Result<(), E> + ~const Destruct;

    async fn enumerate_for_each_async<F>(self, action: F)
    where
        F: AsyncFn(usize, T) + ~const Destruct;
    async fn try_enumerate_for_each_async<F, E>(self, action: F) -> Result<(), E>
    where
        F: AsyncFn(usize, T) -> Result<(), E> + ~const Destruct;
}

impl<T, const N: usize> ArrayEnumerateForEach<T, N> for [T; N]
{
    fn enumerate_for_each<F>(self, action: F)
    where
        F: FnMut(usize, T)
    {
        r#impl::enumerate_for_each(self, action)
    }
    fn try_enumerate_for_each<F, E>(self, action: F) -> Result<(), E>
    where
        F: FnMut(usize, T) -> Result<(), E>
    {
        r#impl::try_enumerate_for_each(self, action)
    }

    fn enumerate_rfor_each<F>(self, action: F)
    where
        F: FnMut(usize, T)
    {
        r#impl::enumerate_rfor_each(self, action)
    }
    fn try_enumerate_rfor_each<F, E>(self, action: F) -> Result<(), E>
    where
        F: FnMut(usize, T) -> Result<(), E>
    {
        r#impl::try_enumerate_rfor_each(self, action)
    }

    async fn enumerate_for_each_async<F>(self, action: F)
    where
        F: AsyncFn(usize, T)
    {
        Actions::new(self.enumerate_map(|i, x| action(i, x))).await
    }
    async fn try_enumerate_for_each_async<F, E>(self, action: F) -> Result<(), E>
    where
        F: AsyncFn(usize, T) -> Result<(), E>
    {
        TryActions::new(self.enumerate_map(|i, x| action(i, x))).await
    }
}

mod r#impl
{
    use crate::{form::ArrayForm, private::guard::Dir};

    use super::PartialEmptyGuard;

    pub(super) fn enumerate_for_each<const N: usize, A, F>(array: A, action: F)
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem)
    {
        enumerate_dfor_each::<{Dir::Left}, _, _, _>(array, action);
    }
    pub(super) fn enumerate_rfor_each<const N: usize, A, F>(array: A, action: F)
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem)
    {
        enumerate_dfor_each::<{Dir::Right}, _, _, _>(array, action);
    }
    fn enumerate_dfor_each<const D: Dir, const N: usize, A, F>(array: A, mut action: F)
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem)
    {
        let mut guard = PartialEmptyGuard::<_, D, _>::new(array);

        while guard.more()
        {
            action(
                guard.index(),
                guard.pop()
            )
        }

        guard.done();
    }

    pub(super) fn try_enumerate_for_each<const N: usize, A, F, E>(array: A, action: F) -> Result<(), E>
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem) -> Result<(), E>
    {
        try_enumerate_dfor_each::<{Dir::Left}, _, _, _, _>(array, action)
    }
    pub(super) fn try_enumerate_rfor_each<const N: usize, A, F, E>(array: A, action: F) -> Result<(), E>
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem) -> Result<(), E>
    {
        try_enumerate_dfor_each::<{Dir::Right}, _, _, _, _>(array, action)
    }
    fn try_enumerate_dfor_each<const D: Dir, const N: usize, A, F, E>(array: A, mut action: F) -> Result<(), E>
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem) -> Result<(), E>
    {
        let mut guard = PartialEmptyGuard::<_, D, _>::new(array);

        while guard.more()
        {
            action(
                guard.index(),
                guard.pop()
            )?
        }

        guard.done();

        Ok(())
    }
}