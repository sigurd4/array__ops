use core::{future::Future, pin::Pin, task::{Context, Poll}};

use super::MaybeDone;

pub struct FutureDivideAndConquer<T, F, const N: usize>
where
    F: FnMut<(T, T), Output: Future<Output = T>>
{
    tasks: [MaybeDone<F::Output>; N],
    k: usize,
    reduce: F
}

impl<T, F, const N: usize> FutureDivideAndConquer<T, F, N>
where
    F: FnMut<(T, T), Output: Future<Output = T>>
{
    pub(crate) fn new(data: [T; N], reduce: F) -> Self
    {
        Self {
            tasks: data.map(|x| MaybeDone::Done(x)),
            k: 1,
            reduce
        }
    }
}

impl<T, F, const N: usize> Future for FutureDivideAndConquer<T, F, N>
where
    F: FnMut<(T, T), Output: Future<Output = T>>
{
    type Output = Option<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        let mut done = true;

        if self.k <= 0
        {
            return Poll::Ready(None)
        }

        while self.k < N
        {
            let mut i = 0;
            
            while i < N
            {
                let task = unsafe {
                    self.as_mut()
                        .map_unchecked_mut(|join| &mut join.tasks[i])
                };
                done &= task.poll(cx)
                    .is_ready();

                let join = unsafe {
                    self.as_mut()
                        .get_unchecked_mut()
                };
                i += join.k;
            }

            if !done
            {
                return Poll::Pending
            }
            
            let join = unsafe {
                self.as_mut()
                    .get_unchecked_mut()
            };
            
            let k2 = join.k*2;
            i = 0;
            while i < N
            {
                let reduce = &mut join.reduce;
                let future = reduce(
                    join.tasks[i].take_output().unwrap(),
                    join.tasks[i + join.k].take_output().unwrap()
                );
                join.tasks[i] = MaybeDone::Future(future);
                i += k2
            }
            join.k = k2;
            if join.k >= N
            {
                break
            }
        }

        let join = unsafe {
            self.as_mut()
                .get_unchecked_mut()
        };

        let result = join.tasks.get_mut(0).map(|task| task.take_output().unwrap());
        join.k = 0;

        Poll::Ready(result)
    }
}