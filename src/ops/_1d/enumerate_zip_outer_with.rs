use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use crate::form::ArrayForm;

use super::{ArrayEnumerate, ArrayZipOuterWith};

#[const_trait]
pub trait ArrayEnumerateZipOuterWith<T, const N: usize>: ArrayEnumerate<T, N> + ArrayZipOuterWith<T, N>
{
    fn enumerate_zip_outer_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, T, Rhs::Elem)> + ~const Destruct,
        T: Copy;
    fn enumerate_zip_outer_ref_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, &'a T, Rhs::Elem)> + ~const Destruct;
    fn enumerate_zip_outer_pin_ref_with<'a, Zip, Rhs, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, Pin<&'a T>, Rhs::Elem)> + ~const Destruct;

    async fn enumerate_zip_outer_async_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, T, Rhs::Elem)> + ~const Destruct,
        T: Copy;
    async fn enumerate_zip_outer_ref_async_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, &'a T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn enumerate_zip_outer_pin_ref_async_with<'a, Zip, Rhs, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, Pin<&'a T>, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    
    fn try_enumerate_zip_outer_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: Copy;
    fn try_enumerate_zip_outer_ref_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, &'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_zip_outer_pin_ref_with<'a, Zip, Rhs, U, E, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, Pin<&'a T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    
    async fn try_enumerate_zip_outer_async_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: Copy;
    async fn try_enumerate_zip_outer_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, &'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_zip_outer_pin_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, Pin<&'a T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayEnumerateZipOuterWith<T, N> for [T; N]
{
    fn enumerate_zip_outer_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, T, Rhs::Elem)>,
        T: Copy
    {
        r#impl::enumerate_zip_outer_with(self, rhs, zipper)
    }
    fn enumerate_zip_outer_ref_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, &'a T, Rhs::Elem)>
    {
        r#impl::enumerate_zip_outer_with(&self, rhs, zipper)
    }
    fn enumerate_zip_outer_pin_ref_with<'a, Zip, Rhs, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, Pin<&'a T>, Rhs::Elem)>
    {
        r#impl::enumerate_zip_outer_with(&self, rhs, zipper)
    }
    
    async fn enumerate_zip_outer_async_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, T, Rhs::Elem)>,
        T: Copy
    {
        r#impl::enumerate_zip_outer_async_with(self, rhs, zipper).await
    }
    async fn enumerate_zip_outer_ref_async_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, &'a T, Rhs::Elem)>,
        T: 'a
    {
        r#impl::enumerate_zip_outer_async_with(&self, rhs, zipper).await
    }
    async fn enumerate_zip_outer_pin_ref_async_with<'a, Zip, Rhs, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, Pin<&'a T>, Rhs::Elem)>,
        T: 'a
    {
        r#impl::enumerate_zip_outer_async_with(&self, rhs, zipper).await
    }
    
    fn try_enumerate_zip_outer_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, T, Rhs::Elem) -> Result<U, E>,
        T: Copy
    {
        r#impl::try_enumerate_zip_outer_with(self, rhs, zipper)
    }
    fn try_enumerate_zip_outer_ref_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, &'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_outer_with(&self, rhs, zipper)
    }
    fn try_enumerate_zip_outer_pin_ref_with<'a, Zip, Rhs, U, E, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, Pin<&'a T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_outer_with(&self, rhs, zipper)
    }
    
    async fn try_enumerate_zip_outer_async_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, T, Rhs::Elem) -> Result<U, E>,
        T: Copy
    {
        r#impl::try_enumerate_zip_outer_async_with(self, rhs, zipper).await
    }
    async fn try_enumerate_zip_outer_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, &'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_outer_async_with(&self, rhs, zipper).await
    }
    async fn try_enumerate_zip_outer_pin_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(self: Pin<&'a Self>, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, Pin<&'a T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_outer_async_with(&self, rhs, zipper).await
    }
}

mod r#impl
{
    use core::ops::AsyncFn;

    use crate::{form::ArrayForm, ops::ArrayJoin2D};

    pub(super) async fn enumerate_zip_outer_async_with<Zip, Lhs, Rhs, const N: usize, const M: usize>(lhs: &Lhs, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Lhs: ArrayForm<N, Elem: Copy>,
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, Lhs::Elem, Rhs::Elem)>
    {
        enumerate_zip_outer_with(lhs, rhs, |i, j, x, y| zipper(i, j, x, y)).join_runs_2d().await
    }
    pub(super) fn enumerate_zip_outer_with<Zip, Lhs, Rhs, const N: usize, const M: usize>(lhs: &Lhs, rhs: &Rhs, mut zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Lhs: ArrayForm<N, Elem: Copy>,
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, Lhs::Elem, Rhs::Elem)>
    {
        crate::from_fn(|i| crate::from_fn(|j| zipper(i, j, lhs.copy_elem(i), rhs.copy_elem(j))))
    }
    pub(super) async fn try_enumerate_zip_outer_async_with<Zip, Lhs, Rhs, const N: usize, const M: usize, U, E>(lhs: &Lhs, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Lhs: ArrayForm<N, Elem: Copy>,
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, Lhs::Elem, Rhs::Elem) -> Result<U, E>
    {
        enumerate_zip_outer_with(lhs, rhs, |i, j, x, y| zipper(i, j, x, y)).try_join_runs_2d().await
    }
    pub(super) fn try_enumerate_zip_outer_with<Zip, Lhs, Rhs, const N: usize, const M: usize, U, E>(lhs: &Lhs, rhs: &Rhs, mut zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Lhs: ArrayForm<N, Elem: Copy>,
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, Lhs::Elem, Rhs::Elem) -> Result<U, E>
    {
        crate::try_from_fn(|i| crate::try_from_fn(|j| zipper(i, j, lhs.copy_elem(i), rhs.copy_elem(j))))
    }
}