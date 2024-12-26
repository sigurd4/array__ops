use core::{ops::AsyncFn, marker::Destruct};

use array_trait::Array;

use crate::{ops::ArrayJoin2D, ArrayForm, Runs2D, TryRuns2D};

use super::{Enumerate, ZipOuterWith};

#[const_trait]
pub trait EnumerateZipOuterWith<T, const N: usize>: Enumerate<T, N> + ZipOuterWith<T, N>
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
}

impl<T, const N: usize> EnumerateZipOuterWith<T, N> for [T; N]
{
    fn enumerate_zip_outer_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, mut zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, T, Rhs::Elem)>,
        T: Copy
    {
        crate::from_fn(|i| crate::from_fn(|j| zipper(i, j, self[i], rhs.copy_elem(j))))
    }
    fn enumerate_zip_outer_ref_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, mut zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut<(usize, usize, &'a T, Rhs::Elem)>
    {
        crate::from_fn(|i| crate::from_fn(|j| zipper(i, j, &self[i], rhs.copy_elem(j))))
    }
    
    async fn enumerate_zip_outer_async_with<Zip, Rhs, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, T, Rhs::Elem)>,
        T: Copy
    {
        self.enumerate_zip_outer_with(rhs, |i, j, x, y| zipper(i, j, x, y)).join_runs_2d().await
    }
    async fn enumerate_zip_outer_ref_async_with<'a, Zip, Rhs, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> [[Zip::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn<(usize, usize, &'a T, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_outer_ref_with(rhs, |i, j, x, y| zipper(i, j, x, y)).join_runs_2d().await
    }
    
    fn try_enumerate_zip_outer_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, mut zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, T, Rhs::Elem) -> Result<U, E>,
        T: Copy
    {
        crate::try_from_fn(|i| crate::try_from_fn(|j| zipper(i, j, self[i], rhs.copy_elem(j))))
    }
    fn try_enumerate_zip_outer_ref_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, mut zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: FnMut(usize, usize, &'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        crate::try_from_fn(|i| crate::try_from_fn(|j| zipper(i, j, &self[i], rhs.copy_elem(j))))
    }
    
    async fn try_enumerate_zip_outer_async_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, T, Rhs::Elem) -> Result<U, E>,
        T: Copy
    {
        self.enumerate_zip_outer_with(rhs, |i, j, x, y| zipper(i, j, x, y)).try_join_runs_2d().await
    }
    async fn try_enumerate_zip_outer_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(usize, usize, &'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.enumerate_zip_outer_ref_with(rhs, |i, j, x, y| zipper(i, j, x, y)).try_join_runs_2d().await
    }
}