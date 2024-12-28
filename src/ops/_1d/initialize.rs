use core::{mem::MaybeUninit, ops::AsyncFn};

use array_trait::Array;

use crate::private;

use super::EnumerateVisit;

#[const_trait]
pub trait Initialize<T, const N: usize>: Array<Item = MaybeUninit<T>>
{
    fn initialize<F>(self: &mut Self, fill: F) -> &mut [T; N]
    where
        F: FnMut(usize) -> T;
    fn rinitialize<F>(self: &mut Self, fill: F) -> &mut [T; N]
    where
        F: FnMut(usize) -> T;
    async fn initialize_async<'a, F>(self: &'a mut Self, fill: F) -> &'a mut [T; N]
    where
        F: AsyncFn(usize) -> T,
        T: 'a;

    fn try_initialize<F, E>(self: &mut Self, fill: F) -> Result<&mut [T; N], E>
    where
        F: FnMut(usize) -> Result<T, E>;
    fn try_rinitialize<F, E>(self: &mut Self, fill: F) -> Result<&mut [T; N], E>
    where
        F: FnMut(usize) -> Result<T, E>;
    async fn try_initialize_async<'a, F, E>(self: &'a mut Self, fill: F) -> Result<&'a mut [T; N], E>
    where
        F: AsyncFn(usize) -> Result<T, E>,
        T: 'a;
}

impl<T, const N: usize> Initialize<T, N> for [MaybeUninit<T>; N]
{
    fn initialize<F>(self: &mut Self, mut fill: F) -> &mut [T; N]
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
    fn rinitialize<F>(self: &mut Self, mut fill: F) -> &mut [T; N]
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
    async fn initialize_async<'a, F>(self: &'a mut Self, fill: F) -> &'a mut [T; N]
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

    fn try_initialize<F, E>(self: &mut Self, mut fill: F) -> Result<&mut [T; N], E>
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
    fn try_rinitialize<F, E>(self: &mut Self, mut fill: F) -> Result<&mut [T; N], E>
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
    async fn try_initialize_async<'a, F, E>(self: &'a mut Self, fill: F) -> Result<&'a mut [T; N], E>
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