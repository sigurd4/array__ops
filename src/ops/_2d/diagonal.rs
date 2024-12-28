use core::{marker::Destruct, pin::Pin};

use array_trait::Array;

use crate::{ops::ArrayIsolate, private::guard::PartialEmptyGuard};

#[const_trait]
pub trait ArrayDiagonal<T, const M: usize, const N: usize>: Array<Item = [T; N]>
{
    fn diagonal(self) -> [T; crate::min_len(M, N)]
    where
        T: ~const Destruct;
    fn diagonal_ref(&self) -> [&T; crate::min_len(M, N)];
    fn diagonal_mut(&mut self) -> [&mut T; crate::min_len(M, N)];
    fn diagonal_pin_ref(self: Pin<&Self>) -> [Pin<&T>; crate::min_len(M, N)];
    fn diagonal_pin_mut(self: Pin<&mut Self>) -> [Pin<&mut T>; crate::min_len(M, N)];
}

impl<T, const M: usize, const N: usize> ArrayDiagonal<T, M, N> for [[T; N]; M]
{
    fn diagonal(self) -> [T; crate::min_len(M, N)]
    {
        let mut guard = PartialEmptyGuard::new_left(self);
        crate::from_fn(move |n| unsafe {
            guard.pop().isolate(n).unwrap_unchecked()
        })
    }
    fn diagonal_ref(&self) -> [&T; crate::min_len(M, N)]
    {
        crate::from_fn(|n| &self[n][n])
    }
    fn diagonal_mut(&mut self) -> [&mut T; crate::min_len(M, N)]
    {
        crate::from_fn(|n| unsafe {
            (&mut self[n][n] as *mut T).as_mut_unchecked()
        })
    }
    fn diagonal_pin_ref(self: Pin<&Self>) -> [Pin<&T>; crate::min_len(M, N)]
    {
        crate::from_fn(|n| unsafe {
            Pin::new_unchecked(&self.get_ref()[n][n])
        })
    }
    fn diagonal_pin_mut(mut self: Pin<&mut Self>) -> [Pin<&mut T>; crate::min_len(M, N)]
    {
        crate::from_fn(|n| unsafe {
            Pin::new_unchecked(&mut (&mut self as *mut Pin<&mut Self>).as_mut_unchecked().as_mut().get_unchecked_mut()[n][n])
        })
    }
}