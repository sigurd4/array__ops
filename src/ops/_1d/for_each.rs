use core::{marker::Destruct, ops::AsyncFn};

use array_trait::Array;

use super::ArrayEnumerateForEach;

#[const_trait]
pub trait ArrayForEach<T, const N: usize>: Array<Item = T>
{
    fn for_each<F>(self, action: F)
    where
        F: FnMut(T) + ~const Destruct;
    fn try_for_each<F, E>(self, action: F) -> Result<(), E>
    where
        F: FnMut(T) -> Result<(), E> + ~const Destruct;

    fn rfor_each<F>(self, action: F)
    where
        F: FnMut(T) + ~const Destruct;
    fn try_rfor_each<F, E>(self, action: F) -> Result<(), E>
    where
        F: FnMut(T) -> Result<(), E> + ~const Destruct;

    async fn for_each_async<F>(self, action: F)
    where
        F: AsyncFn(T) + ~const Destruct;
    async fn try_for_each_async<F, E>(self, action: F) -> Result<(), E>
    where
        F: AsyncFn(T) -> Result<(), E> + ~const Destruct;
}

impl<T, const N: usize> ArrayForEach<T, N> for [T; N]
{
    fn for_each<F>(self, mut action: F)
    where
        F: FnMut(T)
    {
        self.enumerate_for_each(|_, x| action(x))
    }
    fn try_for_each<F, E>(self, mut action: F) -> Result<(), E>
    where
        F: FnMut(T) -> Result<(), E>
    {
        self.try_enumerate_for_each(|_, x| action(x))
    }

    fn rfor_each<F>(self, mut action: F)
    where
        F: FnMut(T)
    {
        self.enumerate_rfor_each(|_, x| action(x))
    }
    fn try_rfor_each<F, E>(self, mut action: F) -> Result<(), E>
    where
        F: FnMut(T) -> Result<(), E>
    {
        self.try_enumerate_rfor_each(|_, x| action(x))
    }

    async fn for_each_async<F>(self, action: F)
    where
        F: AsyncFn(T)
    {
        self.enumerate_for_each_async(|_, x| action(x)).await
    }
    async fn try_for_each_async<F, E>(self, action: F) -> Result<(), E>
    where
        F: AsyncFn(T) -> Result<(), E>
    {
        self.try_enumerate_for_each_async(|_, x| action(x)).await
    }
}