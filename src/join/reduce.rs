use core::{future::Future, pin::Pin, task::{Context, Poll}};

use crate::ops::ArrayMap;

use super::MaybeDone;

pub struct FutureReduce<T, F, const N: usize>
where
    F: FnMut<(T, T), Output: Future<Output = T>>
{
    tasks: [MaybeDone<F::Output>; N],
    reduce: F
}

impl<T, F, const N: usize> FutureReduce<T, F, N>
where
    F: FnMut<(T, T), Output: Future<Output = T>>
{
    pub(crate) fn new(data: [T; N], reduce: F) -> Self
    {
        Self {
            tasks: ArrayMap::map(data, |x| MaybeDone::Done(x)),
            reduce
        }
    }
}

impl<T, F, const N: usize> Future for FutureReduce<T, F, N>
where
    F: FnMut<(T, T), Output: Future<Output = T>>
{
    type Output = Option<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        if N <= 0
        {
            return Poll::Ready(None)    
        }

        let mut done = true;

        let mut i = 0;
        let mut j = usize::MAX;
        
        while i < N
        {
            let task = unsafe {
                self.as_mut()
                    .map_unchecked_mut(|join| &mut join.tasks[i])
            };
            if !task.is_taken()
            {
                let ready = task.poll(cx)
                    .is_ready();
                done &= ready;

                let join = unsafe {
                    self.as_mut()
                        .get_unchecked_mut()
                };
                if ready
                {
                    if j < i
                    {
                        let reduce = &mut join.reduce;
                        let tasks = &mut join.tasks;
                        let future = reduce(
                            tasks[j].take_output().unwrap(),
                            tasks[i].take_output().unwrap()
                        );
                        tasks[i] = MaybeDone::Future(future);
                        j = usize::MAX;
                        continue
                    }
                    else
                    {
                        j = i;
                    }
                }
            }
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

        let result = join.tasks.get_mut(i - 1).map(|task| task.take_output().unwrap());

        Poll::Ready(result)
    }
}