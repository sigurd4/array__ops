use core::{mem::MaybeUninit, ops::AsyncFn};

use array_trait::Array;
use slice_ops::AsSlice;

use super::ArrayEnumerateVisit;

#[const_trait]
pub trait ArrayInitialize<T, const N: usize>: Array + AsSlice<Item = MaybeUninit<T>>
{
    fn initialize<F>(&mut self, fill: F) -> &mut [T; N]
    where
        F: FnMut(usize) -> T;
    fn rinitialize<F>(&mut self, fill: F) -> &mut [T; N]
    where
        F: FnMut(usize) -> T;
    async fn initialize_async<'a, F>(&'a mut self, fill: F) -> &'a mut [T; N]
    where
        F: AsyncFn(usize) -> T,
        T: 'a;

    fn try_initialize<F, E>(&mut self, fill: F) -> Result<&mut [T; N], E>
    where
        F: FnMut(usize) -> Result<T, E>;
    fn try_rinitialize<F, E>(&mut self, fill: F) -> Result<&mut [T; N], E>
    where
        F: FnMut(usize) -> Result<T, E>;
    async fn try_initialize_async<'a, F, E>(&'a mut self, fill: F) -> Result<&'a mut [T; N], E>
    where
        F: AsyncFn(usize) -> Result<T, E>,
        T: 'a;
}

impl<T, const N: usize> ArrayInitialize<T, N> for [MaybeUninit<T>; N]
{
    fn initialize<F>(&mut self, mut fill: F) -> &mut [T; N]
    where
        F: FnMut(usize) -> T
    {
        self.enumerate_visit_mut(|i, x| {
            x.write(fill(i));
        });
        unsafe {
            (self as *mut Self).cast::<[T; N]>().as_mut_unchecked()
        }
    }
    fn rinitialize<F>(&mut self, mut fill: F) -> &mut [T; N]
    where
        F: FnMut(usize) -> T
    {
        self.enumerate_rvisit_mut(|i, x| {
            x.write(fill(i));
        });
        unsafe {
            (self as *mut Self).cast::<[T; N]>().as_mut_unchecked()
        }
    }
    async fn initialize_async<'a, F>(&'a mut self, fill: F) -> &'a mut [T; N]
    where
        F: AsyncFn(usize) -> T,
        T: 'a
    {
        self.enumerate_visit_mut_async(async |i, x| {
            x.write(fill(i).await);
        }).await;
        unsafe {
            (self as *mut Self).cast::<[T; N]>().as_mut_unchecked()
        }
    }

    fn try_initialize<F, E>(&mut self, mut fill: F) -> Result<&mut [T; N], E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        self.try_enumerate_visit_mut(|i, x| {
            x.write(fill(i)?);
            Ok(())
        })?;
        unsafe {
            Ok((self as *mut Self).cast::<[T; N]>().as_mut_unchecked())
        }
    }
    fn try_rinitialize<F, E>(&mut self, mut fill: F) -> Result<&mut [T; N], E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        self.try_enumerate_rvisit_mut(|i, x| {
            x.write(fill(i)?);
            Ok(())
        })?;
        unsafe {
            Ok((self as *mut Self).cast::<[T; N]>().as_mut_unchecked())
        }
    }
    async fn try_initialize_async<'a, F, E>(&'a mut self, fill: F) -> Result<&'a mut [T; N], E>
    where
        F: AsyncFn(usize) -> Result<T, E>,
        T: 'a
    {
        self.try_enumerate_visit_mut_async(async |i, x| {
            x.write(fill(i).await?);
            Ok(())
        }).await?;
        unsafe {
            Ok((self as *mut Self).cast::<[T; N]>().as_mut_unchecked())
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::ops::*;

    #[test]
    fn it_works()
    {
        let a = ["one", "two"];
        let b = ["three"];
        
        assert_eq!(a.chain(b), ["one", "two", "three"]);
    }
}