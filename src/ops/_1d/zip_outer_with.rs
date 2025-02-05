use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;
use slice_ops::AsSlice;

use crate::form::ArrayForm;

use super::ArrayEnumerateZipOuterWith;

#[const_trait]
pub trait ArrayZipOuterWith<T, const N: usize>: Array + AsSlice<Item = T>
{
    fn zip_outer_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(T, Rhs::Elem)> + ~const Destruct,
        T: Copy;
    fn zip_outer_ref_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(&'a T, Rhs::Elem)> + ~const Destruct;
    fn zip_outer_pin_ref_with<'a, Zip, Rhs, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(Pin<&'a T>, Rhs::Elem)> + ~const Destruct;

    async fn zip_outer_async_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(T, Rhs::Elem)> + ~const Destruct,
        T: Copy;
    async fn zip_outer_ref_async_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(&'a T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn zip_outer_pin_ref_async_with<'a, Zip, Rhs, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(Pin<&'a T>, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    
    fn try_zip_outer_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: Copy;
    fn try_zip_outer_ref_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(&'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_zip_outer_pin_ref_with<'a, Zip, Rhs, U, E, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(Pin<&'a T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    
    async fn try_zip_outer_async_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: Copy;
    async fn try_zip_outer_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(&'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_zip_outer_mut_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(Pin<&'a T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayZipOuterWith<T, N> for [T; N]
{
    fn zip_outer_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, mut zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(T, Rhs::Elem)>,
        T: Copy
    {
        self.enumerate_zip_outer_with(rhs, |_, _, x, y| zipper(x, y))
    }
    fn zip_outer_ref_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, mut zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(&'a T, Rhs::Elem)>
    {
        self.enumerate_zip_outer_ref_with(rhs, |_, _, x, y| zipper(x, y))
    }
    fn zip_outer_pin_ref_with<'a, Zip, Rhs, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, mut zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(Pin<&'a T>, Rhs::Elem)>
    {
        self.enumerate_zip_outer_pin_ref_with(rhs, |_, _, x, y| zipper(x, y))
    }
    
    async fn zip_outer_async_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(T, Rhs::Elem)>,
        T: Copy
    {
        self.enumerate_zip_outer_async_with(rhs, |_, _, x, y| zipper(x, y)).await
    }
    async fn zip_outer_ref_async_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(&'a T, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_outer_ref_async_with(rhs, |_, _, x, y| zipper(x, y)).await
    }
    async fn zip_outer_pin_ref_async_with<'a, Zip, Rhs, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(Pin<&'a T>, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_outer_pin_ref_async_with(rhs, |_, _, x, y| zipper(x, y)).await
    }
    
    fn try_zip_outer_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, mut zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(T, Rhs::Elem) -> Result<U, E>,
        T: Copy
    {
        self.try_enumerate_zip_outer_with(rhs, |_, _, x, y| zipper(x, y))
    }
    fn try_zip_outer_ref_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, mut zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(&'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_outer_ref_with(rhs, |_, _, x, y| zipper(x, y))
    }
    fn try_zip_outer_pin_ref_with<'a, Zip, Rhs, U, E, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, mut zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(Pin<&'a T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_outer_pin_ref_with(rhs, |_, _, x, y| zipper(x, y))
    }
    
    async fn try_zip_outer_async_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<U, E>,
        T: Copy
    {
        self.try_enumerate_zip_outer_async_with(rhs, |_, _, x, y| zipper(x, y)).await
    }
    async fn try_zip_outer_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(&'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_outer_ref_async_with(rhs, |_, _, x, y| zipper(x, y)).await
    }
    async fn try_zip_outer_mut_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(Pin<&'a T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_outer_pin_ref_async_with(rhs, |_, _, x, y| zipper(x, y)).await
    }
}