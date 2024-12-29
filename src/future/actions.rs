use core::{future::Future, pin::Pin, task::{Context, Poll}};

pub struct Actions<T, const N: usize>
where
    T: Future<Output = ()>
{
    tasks: [T; N]
}

impl<T, const N: usize> Actions<T, N>
where
    T: Future<Output = ()>
{
    pub(crate) fn new(tasks: [T; N]) -> Self
    {
        Self {
            tasks
        }
    }
}

impl<T, const N: usize> Future for Actions<T, N>
where
    T: Future<Output = ()>
{
    type Output = ();

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

        Poll::Ready(())
    }
}