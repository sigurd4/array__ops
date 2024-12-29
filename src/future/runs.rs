use core::{future::Future, pin::Pin, task::{Context, Poll}};

use crate::ops::ArrayMap;

use super::MaybeDone;

pub struct Runs<T, U, const N: usize>
where
    T: Future<Output = U>
{
    tasks: [MaybeDone<T>; N]
}

impl<T, U, const N: usize> Runs<T, U, N>
where
    T: Future<Output = U>
{
    pub(crate) fn new(tasks: [T; N]) -> Self
    {
        Self {
            tasks: ArrayMap::map(tasks, |task| MaybeDone::Future(task))
        }
    }
}

impl<T, U, const N: usize> Future for Runs<T, U, N>
where
    T: Future<Output = U>
{
    type Output = [U; N];

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        let mut done = true;

        let mut i = 0;
        while i < N
        {
            let task = unsafe {
                self.as_mut()
                    .map_unchecked_mut(|join| &mut join.tasks[i])
            };
            done &= task.poll(cx)
                .is_ready();
            i += 1;
        }

        if !done
        {
            return Poll::Pending
        }

        let join = unsafe {
            self.as_mut()
                .get_unchecked_mut()
        };

        let result = join.tasks.map_mut(|task| task.take_output().unwrap());

        Poll::Ready(result)
    }
}