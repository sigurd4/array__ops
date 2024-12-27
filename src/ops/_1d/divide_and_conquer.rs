use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;

use crate::{private::guard::PartialDivideAndConquerGuard, join::FutureDivideAndConquer};

use super::Each;

#[const_trait]
pub trait DivideAndConquer<T, const N: usize>: Array<Item = T>
{
    fn divide_and_conquer<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T + ~const Destruct;
    fn divide_and_conquer_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T + ~const Destruct;
    fn divide_and_conquer_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T + ~const Destruct;
    fn divide_and_conquer_pin_ref<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: FnMut(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T> + ~const Destruct;
    fn divide_and_conquer_pin_mut<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: FnMut(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T> + ~const Destruct;
        
    async fn divide_and_conquer_async<F>(self, reduce: F) -> Option<T>
    where
        F: AsyncFn(T, T) -> T + ~const Destruct;
    async fn divide_and_conquer_ref_async<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: AsyncFn(&'a T, &'a T) -> &'a T + ~const Destruct,
        T: 'a;
    async fn divide_and_conquer_mut_async<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: AsyncFn(&'a mut T, &'a mut T) -> &'a mut T + ~const Destruct,
        T: 'a;
    async fn divide_and_conquer_pin_ref_async<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: AsyncFn(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T> + ~const Destruct,
        T: 'a;
    async fn divide_and_conquer_pin_mut_async<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: AsyncFn(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> DivideAndConquer<T, N> for [T; N]
{
    fn divide_and_conquer<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T
    {
        PartialDivideAndConquerGuard::new_left(self).reduce(reduce)
    }
    fn divide_and_conquer_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T
    {
        PartialDivideAndConquerGuard::new_left(self).reduce(reduce)
    }
    fn divide_and_conquer_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T
    {
        PartialDivideAndConquerGuard::new_left(self).reduce(reduce)
    }
    fn divide_and_conquer_pin_ref<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: FnMut(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T>
    {
        PartialDivideAndConquerGuard::new_left(self).reduce(reduce)
    }
    fn divide_and_conquer_pin_mut<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: FnMut(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T>
    {
        PartialDivideAndConquerGuard::new_left(self).reduce(reduce)
    }
        
    async fn divide_and_conquer_async<F>(self, reduce: F) -> Option<T>
    where
        F: AsyncFn(T, T) -> T
    {
        FutureDivideAndConquer::new(self, |x, y| reduce(x, y)).await
    }
    async fn divide_and_conquer_ref_async<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: AsyncFn(&'a T, &'a T) -> &'a T,
        T: 'a
    {
        FutureDivideAndConquer::new(self.each_ref(), |x, y| reduce(x, y)).await
    }
    async fn divide_and_conquer_mut_async<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: AsyncFn(&'a mut T, &'a mut T) -> &'a mut T,
        T: 'a
    {
        FutureDivideAndConquer::new(self.each_mut(), |x, y| reduce(x, y)).await
    }
    async fn divide_and_conquer_pin_ref_async<'a, F>(self: Pin<&'a Self>, reduce: F) -> Option<Pin<&'a T>>
    where
        F: AsyncFn(Pin<&'a T>, Pin<&'a T>) -> Pin<&'a T>,
        T: 'a
    {
        FutureDivideAndConquer::new(self.each_pin_ref(), |x, y| reduce(x, y)).await
    }
    async fn divide_and_conquer_pin_mut_async<'a, F>(self: Pin<&'a mut Self>, reduce: F) -> Option<Pin<&'a mut T>>
    where
        F: AsyncFn(Pin<&'a mut T>, Pin<&'a mut T>) -> Pin<&'a mut T>,
        T: 'a
    {
        FutureDivideAndConquer::new(self.each_pin_mut(), |x, y| reduce(x, y)).await
    }
}

#[cfg(test)]
mod test
{
    use crate::ops::*;

    #[test]
    fn it_works()
    {
        let a = [1, 2, 3];

        let s = a.divide_and_conquer(|x, y| x + y).unwrap();

        println!("{}", s)
    }
}