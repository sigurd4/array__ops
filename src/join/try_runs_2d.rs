use core::{future::Future, pin::Pin, task::{Context, Poll}};

use crate::ops::Map;

use super::MaybeDone;

pub struct TryRuns2D<T, U, E, const M: usize, const N: usize>
where
    T: Future<Output = Result<U, E>>
{
    tasks: [[MaybeDone<T>; N]; M]
}

impl<T, U, E, const M: usize, const N: usize> TryRuns2D<T, U, E, M, N>
where
    T: Future<Output = Result<U, E>>
{
    pub(crate) fn new(tasks: [[T; N]; M]) -> Self
    {
        Self {
            tasks: tasks.map(|tasks| tasks.map(|task| MaybeDone::Future(task)))
        }
    }
}

impl<T, U, E, const M: usize, const N: usize> Future for TryRuns2D<T, U, E, M, N>
where
    T: Future<Output = Result<U, E>>
{
    type Output = Result<[[U; N]; M], E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        let mut done = true;

        let mut j = 0;
        while j < M
        {
            let mut i = 0;
            while i < N
            {
                let task = unsafe {
                    self.as_mut()
                        .map_unchecked_mut(|join| &mut join.tasks[j][i])
                };
                let ready = task.poll(cx)
                    .is_ready();
                if ready
                {
                    let join = unsafe {
                        self.as_mut()
                            .get_unchecked_mut()
                    };
                    let result = join.tasks[j][i].take_output();
                    if let Some(Err(error)) = result
                    {
                        return Poll::Ready(Err(error))
                    }
                }
                else
                {
                    done = false
                }
                i += 1
            }
            j += 1
        }

        if !done
        {
            return Poll::Pending
        }

        let join = unsafe {
            self.as_mut()
                .get_unchecked_mut()
        };

        let result = join.tasks.try_map_mut(|tasks| tasks.try_map_mut(|task| task.take_output().unwrap()));

        Poll::Ready(result)
    }
}