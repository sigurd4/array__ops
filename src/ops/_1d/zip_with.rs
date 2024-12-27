use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;

use crate::form::ArrayForm;

use super::EnumerateZipWith;

#[const_trait]
pub trait ZipWith<T, const N: usize>: Array<Item = T>
{
    fn zip_with<Zip, Rhs>(self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(T, Rhs::Elem)> + ~const Destruct;
    fn zip_ref_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(&'a T, Rhs::Elem)> + ~const Destruct;
    fn zip_mut_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(&'a mut T, Rhs::Elem)> + ~const Destruct;
    fn zip_pin_ref_with<'a, Zip, Rhs>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(Pin<&'a T>, Rhs::Elem)> + ~const Destruct;
    fn zip_pin_mut_with<'a, Zip, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(Pin<&'a mut T>, Rhs::Elem)> + ~const Destruct;
        
    async fn zip_async_with<Zip, Rhs>(self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(T, Rhs::Elem)> + ~const Destruct;
    async fn zip_ref_async_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(&'a T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn zip_mut_async_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(&'a mut T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn zip_pin_ref_async_with<'a, Zip, Rhs>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(Pin<&'a T>, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn zip_pin_mut_async_with<'a, Zip, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(Pin<&'a mut T>, Rhs::Elem)> + ~const Destruct,
        T: 'a;
        
    fn try_zip_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> Result<U, E> + ~const Destruct;
    fn try_zip_ref_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(&'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_zip_mut_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(&'a mut T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_zip_pin_ref_with<'a, Zip, Rhs, U, E>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(Pin<&'a T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_zip_pin_mut_with<'a, Zip, Rhs, U, E>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(Pin<&'a mut T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
        
    async fn try_zip_async_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<U, E> + ~const Destruct;
    async fn try_zip_ref_async_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(&'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_zip_mut_async_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(&'a mut T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_zip_pin_ref_async_with<'a, Zip, Rhs, U, E>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(Pin<&'a T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_zip_pin_mut_async_with<'a, Zip, Rhs, U, E>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(Pin<&'a mut T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ZipWith<T, N> for [T; N]
{
    fn zip_with<Zip, Rhs>(self, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(T, Rhs::Elem)> + Destruct
    {
        self.enumerate_zip_with(rhs, |_, x, y| zipper(x, y))
    }
    fn zip_ref_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(&'a T, Rhs::Elem)>
    {
        self.enumerate_zip_ref_with(rhs, |_, x, y| zipper(x, y))
    }
    fn zip_mut_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(&'a mut T, Rhs::Elem)>
    {
        self.enumerate_zip_mut_with(rhs, |_, x, y| zipper(x, y))
    }
    fn zip_pin_ref_with<'a, Zip, Rhs>(self: Pin<&'a Self>, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(Pin<&'a T>, Rhs::Elem)>
    {
        self.enumerate_zip_pin_ref_with(rhs, |_, x, y| zipper(x, y))
    }
    fn zip_pin_mut_with<'a, Zip, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(Pin<&'a mut T>, Rhs::Elem)>
    {
        self.enumerate_zip_pin_mut_with(rhs, |_, x, y| zipper(x, y))
    }
    
    async fn zip_async_with<Zip, Rhs>(self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(T, Rhs::Elem)>
    {
        self.enumerate_zip_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    async fn zip_ref_async_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(&'a T, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_ref_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    async fn zip_mut_async_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(&'a mut T, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_mut_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    async fn zip_pin_ref_async_with<'a, Zip, Rhs>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(Pin<&'a T>, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_pin_ref_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    async fn zip_pin_mut_async_with<'a, Zip, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(Pin<&'a mut T>, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_pin_mut_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    
    fn try_zip_with<Zip, Rhs, U, E>(self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> Result<U, E>
    {
        self.try_enumerate_zip_with(rhs, |_, x, y| zipper(x, y))
    }
    fn try_zip_ref_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(&'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_ref_with(rhs, |_, x, y| zipper(x, y))
    }
    fn try_zip_mut_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(&'a mut T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_mut_with(rhs, |_, x, y| zipper(x, y))
    }
    fn try_zip_pin_ref_with<'a, Zip, Rhs, U, E>(self: Pin<&'a Self>, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(Pin<&'a T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_pin_ref_with(rhs, |_, x, y| zipper(x, y))
    }
    fn try_zip_pin_mut_with<'a, Zip, Rhs, U, E>(self: Pin<&'a mut Self>, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(Pin<&'a mut T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_pin_mut_with(rhs, |_, x, y| zipper(x, y))
    }
    
    async fn try_zip_async_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<U, E>
    {
        self.try_enumerate_zip_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    async fn try_zip_ref_async_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(&'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_ref_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    async fn try_zip_mut_async_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(&'a mut T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_mut_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    async fn try_zip_pin_ref_async_with<'a, Zip, Rhs, U, E>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(Pin<&'a T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_pin_ref_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
    async fn try_zip_pin_mut_async_with<'a, Zip, Rhs, U, E>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(Pin<&'a mut T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_pin_mut_async_with(rhs, |_, x, y| zipper(x, y)).await
    }
}