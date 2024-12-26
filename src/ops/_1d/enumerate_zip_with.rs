use core::{marker::Destruct, mem::MaybeUninit, ops::AsyncFn};

use array_trait::Array;

use crate::{private::guard::PartialZipGuard, ArrayForm, Runs, TryRuns};

use super::{ArrayJoin, Enumerate, ZipWith};

#[const_trait]
pub trait EnumerateZipWith<T, const N: usize>: Enumerate<T, N> + ZipWith<T, N>
{
    fn enumerate_zip_with<Zip, Rhs>(self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, T, Rhs::Elem)> + ~const Destruct;
    fn enumerate_zip_ref_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, &'a T, Rhs::Elem)> + ~const Destruct;
    fn enumerate_zip_mut_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, &'a mut T, Rhs::Elem)> + ~const Destruct;
        
    async fn enumerate_zip_async_with<Zip, Rhs>(self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, T, Rhs::Elem)> + ~const Destruct;
    async fn enumerate_zip_ref_async_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, &'a T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn enumerate_zip_mut_async_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, &'a mut T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
        
    fn try_enumerate_zip_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, T, Rhs::Elem) -> Result<U, E> + ~const Destruct;
    fn try_enumerate_zip_ref_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, &'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_zip_mut_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, &'a mut T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
        
    async fn try_enumerate_zip_async_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, T, Rhs::Elem) -> Result<U, E> + ~const Destruct;
    async fn try_enumerate_zip_ref_async_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, &'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_zip_mut_async_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, &'a mut T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> EnumerateZipWith<T, N> for [T; N]
{
    fn enumerate_zip_with<Zip, Rhs>(self, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, T, Rhs::Elem)> + Destruct
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        while guard.more()
        {
            guard.enumerate_zip(&mut zipper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    fn enumerate_zip_ref_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, &'a T, Rhs::Elem)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        while guard.more()
        {
            guard.enumerate_zip(&mut zipper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    fn enumerate_zip_mut_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, &'a mut T, Rhs::Elem)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        while guard.more()
        {
            guard.enumerate_zip(&mut zipper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    
    async fn enumerate_zip_async_with<Zip, Rhs>(self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, T, Rhs::Elem)>
    {
        self.enumerate_zip_with(rhs, |i, x, y| zipper(i, x, y)).join_runs().await
    }
    async fn enumerate_zip_ref_async_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, &'a T, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_ref_with(rhs, |i, x, y| zipper(i, x, y)).join_runs().await
    }
    async fn enumerate_zip_mut_async_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, &'a mut T, Rhs::Elem)>,
        T: 'a
    {
        self.enumerate_zip_mut_with(rhs, |i, x, y| zipper(i, x, y)).join_runs().await
    }
    
    fn try_enumerate_zip_with<Zip, Rhs, U, E>(self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, T, Rhs::Elem) -> Result<U, E>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_enumerate_zip(&mut zipper)
            {
                result = Err(error);
                break
            }
        }
        guard.done();
    
        result.map(|()| unsafe {
            MaybeUninit::array_assume_init(dst)
        })
    }
    fn try_enumerate_zip_ref_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, &'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_enumerate_zip(&mut zipper)
            {
                result = Err(error);
                break
            }
        }
        guard.done();
    
        result.map(|()| unsafe {
            MaybeUninit::array_assume_init(dst)
        })
    }
    fn try_enumerate_zip_mut_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, &'a mut T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_enumerate_zip(&mut zipper)
            {
                result = Err(error);
                break
            }
        }
        guard.done();
    
        result.map(|()| unsafe {
            MaybeUninit::array_assume_init(dst)
        })
    }
    
    async fn try_enumerate_zip_async_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, T, Rhs::Elem) -> Result<U, E>
    {
        self.enumerate_zip_with(rhs, |i, x, y| zipper(i, x, y)).try_join_runs().await
    }
    async fn try_enumerate_zip_ref_async_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, &'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.enumerate_zip_ref_with(rhs, |i, x, y| zipper(i, x, y)).try_join_runs().await
    }
    async fn try_enumerate_zip_mut_async_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, &'a mut T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        self.enumerate_zip_mut_with(rhs, |i, x, y| zipper(i, x, y)).try_join_runs().await
    }
}