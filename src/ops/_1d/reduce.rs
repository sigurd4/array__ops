use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;
use slice_ops::AsSlice;

use crate::{future::FutureReduce, private::guard::PartialEmptyGuard};

use super::ArrayEach;

#[const_trait]
pub trait ArrayReduce<T, const N: usize>: Array + AsSlice<Item = T>
{
    /// Reduces elements in array into one element, using a given operand
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// const A: [u8; 3] = [1, 2, 3];
    /// 
    /// let r: u8 = A.reduce(|a, b| a + b).unwrap();
    /// 
    /// assert_eq!(r, 6);
    /// ```
    fn reduce<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T + ~const Destruct;
    fn reduce_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T + ~const Destruct;
    fn reduce_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T + ~const Destruct;
    fn reduce_pin_ref<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: FnMut(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T> + ~const Destruct;
    fn reduce_pin_mut<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: FnMut(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T> + ~const Destruct;
        
    async fn reduce_async<F>(self, reduce: F) -> Option<T>
    where
        F: AsyncFn(T, T) -> T + ~const Destruct;
    async fn reduce_ref_async<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: AsyncFn(&'a T, &'a T) -> &'a T + ~const Destruct,
        T: 'a;
    async fn reduce_mut_async<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: AsyncFn(&'a mut T, &'a mut T) -> &'a mut T + ~const Destruct,
        T: 'a;
    async fn reduce_pin_ref_async<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: AsyncFn(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T> + ~const Destruct,
        T: 'a;
    async fn reduce_pin_mut_async<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: AsyncFn(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayReduce<T, N> for [T; N]
{
    fn reduce<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_pin_ref<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: FnMut(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T>
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_pin_mut<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: FnMut(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T>
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
        
    async fn reduce_async<F>(self, reduce: F) -> Option<T>
    where
        F: AsyncFn(T, T) -> T
    {
        #[allow(clippy::redundant_closure)]
        FutureReduce::new(self, |x, y| reduce(x, y)).await
    }
    async fn reduce_ref_async<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: AsyncFn(&'a T, &'a T) -> &'a T,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        FutureReduce::new(self.each_ref(), |x, y| reduce(x, y)).await
    }
    async fn reduce_mut_async<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: AsyncFn(&'a mut T, &'a mut T) -> &'a mut T,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        FutureReduce::new(self.each_mut(), |x, y| reduce(x, y)).await
    }
    async fn reduce_pin_ref_async<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: AsyncFn(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        FutureReduce::new(self.each_pin_ref(), |x, y| reduce(x, y)).await
    }
    async fn reduce_pin_mut_async<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: AsyncFn(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        FutureReduce::new(self.each_pin_mut(), |x, y| reduce(x, y)).await
    }
}

#[cfg(test)]
mod test
{
    use crate::ops::*;

    #[test]
    fn it_works()
    {
        let a = [1, 2, 3, 4, 5];

        let s = a.reduce(|x, y| x + y).unwrap();

        println!("{}", s);

        tokio_test::block_on(async {
            let s = a.reduce_async(async |x, y| x + y).await.unwrap();

            println!("{}", s);
        });
    }
}