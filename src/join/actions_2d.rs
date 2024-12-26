use core::{future::Future, pin::Pin, task::{Context, Poll}};

pub struct Actions2D<T, const M: usize, const N: usize>
where
    T: Future<Output = ()>
{
    tasks: [[T; N]; M]
}

impl<T, const M: usize, const N: usize> Actions2D<T, M, N>
where
    T: Future<Output = ()>
{
    pub(crate) fn new(tasks: [[T; N]; M]) -> Self
    {
        Self {
            tasks
        }
    }
}

impl<T, const M: usize, const N: usize> Future for Actions2D<T, M, N>
where
    T: Future<Output = ()>
{
    type Output = ();

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

        Poll::Ready(())
    }
}