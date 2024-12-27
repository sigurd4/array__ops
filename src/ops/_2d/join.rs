use core::future::Future;

use array_trait::Array;

use crate::join::{Actions2D, Runs2D, TryActions2D, TryRuns2D};

pub trait ArrayJoin2D<T, const M: usize, const N: usize>: Array<Item = [T; N]>
{
    async fn join_actions_2d(self)
    where
        T: Future<Output = ()>;

    async fn try_join_actions_2d<E>(self) -> Result<(), E>
    where
        T: Future<Output = Result<(), E>>;

    async fn join_runs_2d(self) -> [[T::Output; N]; M]
    where
        T: Future;

    async fn try_join_runs_2d<U, E>(self) -> Result<[[U; N]; M], E>
    where
        T: Future<Output = Result<U, E>>;
}

impl<T, const M: usize, const N: usize> ArrayJoin2D<T, M, N> for [[T; N]; M]
{
    async fn join_actions_2d(self)
    where
        T: Future<Output = ()>
    {
        Actions2D::new(self).await
    }

    async fn try_join_actions_2d<E>(self) -> Result<(), E>
    where
        T: Future<Output = Result<(), E>>
    {
        TryActions2D::new(self).await
    }

    async fn join_runs_2d(self) -> [[T::Output; N]; M]
    where
        T: Future
    {
        Runs2D::new(self).await
    }

    async fn try_join_runs_2d<U, E>(self) -> Result<[[U; N]; M], E>
    where
        T: Future<Output = Result<U, E>>
    {
        TryRuns2D::new(self).await
    }
}