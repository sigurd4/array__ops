use core::future::Future;

use array_trait::Array;

use crate::{Actions, Runs, TryActions, TryRuns};

#[const_trait]
pub trait ArrayJoin<T, const N: usize>: Array<Item = T>
{
    async fn join_actions(self)
    where
        T: Future<Output = ()>;

    async fn try_join_actions<E>(self) -> Result<(), E>
    where
        T: Future<Output = Result<(), E>>;

    async fn join_runs(self) -> [T::Output; N]
    where
        T: Future;

    async fn try_join_runs<U, E>(self) -> Result<[U; N], E>
    where
        T: Future<Output = Result<U, E>>;
}

impl<T, const N: usize> ArrayJoin<T, N> for [T; N]
{
    async fn join_actions(self)
    where
        T: Future<Output = ()>
    {
        Actions::new(self)
    }

    async fn try_join_actions<E>(self) -> Result<(), E>
    where
        T: Future<Output = Result<(), E>>
    {
        TryActions::new(self)
    }

    async fn join_runs(self) -> [T::Output; N]
    where
        T: Future
    {
        Runs::new(self).await
    }

    async fn try_join_runs<U, E>(self) -> Result<[U; N], E>
    where
        T: Future<Output = Result<U, E>>
    {
        TryRuns::new(self).await
    }
}