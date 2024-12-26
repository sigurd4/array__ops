use core::{future::Future, pin::Pin, task::{Context, Poll}};

use super::MaybeDone;

pub struct Runs2D<T, U, const M: usize, const N: usize>
where
    T: Future<Output = U>
{
    tasks: [[MaybeDone<T>; N]; M]
}

impl<T, U, const M: usize, const N: usize> Runs2D<T, U, M, N>
where
    T: Future<Output = U>
{
    pub(crate) fn new(tasks: [[T; N]; M]) -> Self
    {
        Self {
            tasks: tasks.map(|tasks| tasks.map(|task| MaybeDone::Future(task)))
        }
    }
}

impl<T, U, const M: usize, const N: usize> Future for Runs2D<T, U, M, N>
where
    T: Future<Output = U>
{
    type Output = [[U; N]; M];

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
                done &= task.poll(cx)
                    .is_ready();
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

        let result = join.tasks.map_mut(|tasks| tasks.map_mut(|task| task.take_output().unwrap()));

        Poll::Ready(result)
    }
}