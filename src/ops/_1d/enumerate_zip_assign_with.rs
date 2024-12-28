use core::{ops::AsyncFn, marker::Destruct};

use crate::form::ArrayForm;

use super::{ArrayEnumerate, ArrayEnumerateMeet, ArrayZipAssignWith};

#[const_trait]
pub trait ArrayEnumerateZipAssignWith<T, const N: usize>: ArrayEnumerate<T, N> + ArrayZipAssignWith<T, N>
{
    fn enumerate_zip_assign_with<Rhs, Zip>(&mut self, rhs: Rhs, map: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, T, Rhs::Elem) -> T + ~const Destruct;
        
    async fn enumerate_zip_assign_async_with<Rhs, Zip>(&mut self, rhs: Rhs, map: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, T, Rhs::Elem) -> T + ~const Destruct;
        
    fn try_enumerate_zip_assign_with<Rhs, Zip, E>(&mut self, rhs: Rhs, map: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, T, Rhs::Elem) -> Result<T, E> + ~const Destruct;
        
    async fn try_enumerate_zip_assign_async_with<Rhs, Zip, E>(&mut self, rhs: Rhs, map: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, T, Rhs::Elem) -> Result<T, E> + ~const Destruct;
}

impl<T, const N: usize> ArrayEnumerateZipAssignWith<T, N> for [T; N]
{
    fn enumerate_zip_assign_with<Rhs, Zip>(&mut self, rhs: Rhs, mut zip: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, T, Rhs::Elem) -> T
    {
        self.enumerate_meet_each_mut(rhs, |i, x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(i, value, y))
        });
    }
    
    async fn enumerate_zip_assign_async_with<Rhs, Zip>(&mut self, rhs: Rhs, zip: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, T, Rhs::Elem) -> T
    {
        self.enumerate_meet_each_mut_async(rhs, async |i, x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(i, value, y).await)
        }).await
    }
    
    fn try_enumerate_zip_assign_with<Rhs, Zip, E>(&mut self, rhs: Rhs, mut zip: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, T, Rhs::Elem) -> Result<T, E>
    {
        self.try_enumerate_meet_each_mut(rhs, |i, x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(i, value, y)?);
            Ok(())
        })
    }
    
    async fn try_enumerate_zip_assign_async_with<Rhs, Zip, E>(&mut self, rhs: Rhs, zip: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, T, Rhs::Elem) -> Result<T, E>
    {
        self.try_enumerate_meet_each_mut_async(rhs, async |i, x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(i, value, y).await?);
            Ok(())
        }).await
    }
}