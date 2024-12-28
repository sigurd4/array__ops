use core::{future::Future, pin::Pin, task::{Context, Poll}};

use crate::ops::ArrayMap;

use super::MaybeDone;

pub struct TryActions2D<T, E, const M: usize, const N: usize>
where
    T: Future<Output = Result<(), E>>
{
    tasks: [[MaybeDone<T>; N]; M]
}

impl<T, E, const M: usize, const N: usize> TryActions2D<T, E, M, N>
where
    T: Future<Output = Result<(), E>>
{
    pub(crate) fn new(tasks: [[T; N]; M]) -> Self
    {
        Self {
            tasks: ArrayMap::map(tasks, |tasks| ArrayMap::map(tasks, |task| MaybeDone::Future(task)))
        }
    }
}

impl<T, E, const M: usize, const N: usize> Future for TryActions2D<T, E, M, N>
where
    T: Future<Output = Result<(), E>>
{
    type Output = Result<(), E>;

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
                    if let Some(result) = result && result.is_err()
                    {
                        for task in join.tasks.iter_mut().flatten()
                        {
                            task.cancel()
                        }
                        return Poll::Ready(result)
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

        Poll::Ready(Ok(()))
    }
}