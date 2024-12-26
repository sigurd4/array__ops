use core::{ops::AsyncFn, marker::Destruct};

use array_trait::Array;

use crate::form::ArrayForm;

#[const_trait]
pub trait ZipAssignWith<T, const N: usize>: Array<Item = T>
{
    fn zip_assign_with<Rhs, Zip>(&mut self, rhs: Rhs, map: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> T + ~const Destruct;
        
    async fn zip_assign_async_with<Rhs, Zip>(&mut self, rhs: Rhs, map: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> T + ~const Destruct;
        
    fn try_zip_assign_with<Rhs, Zip, E>(&mut self, rhs: Rhs, map: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> Result<T, E> + ~const Destruct;
        
    async fn try_zip_assign_async_with<Rhs, Zip, E>(&mut self, rhs: Rhs, map: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<T, E> + ~const Destruct;
}

impl<T, const N: usize> ZipAssignWith<T, N> for [T; N]
{
    fn zip_assign_with<Rhs, Zip>(&mut self, rhs: Rhs, mut zip: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> T
    {
        self.visit_mut_with(rhs, |x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(value, y))
        });
    }
    
    async fn zip_assign_async_with<Rhs, Zip>(&mut self, rhs: Rhs, zip: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> T
    {
        self.visit_mut_async_with(rhs, async |x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(value, y).await)
        }).await
    }
    
    fn try_zip_assign_with<Rhs, Zip, E>(&mut self, rhs: Rhs, mut zip: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> Result<T, E>
    {
        self.try_visit_mut_with(rhs, |x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(value, y)?);
            Ok(())
        })
    }
    
    async fn try_zip_assign_async_with<Rhs, Zip, E>(&mut self, rhs: Rhs, zip: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<T, E>
    {
        self.try_visit_mut_async_with(rhs, async |x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(value, y).await?);
            Ok(())
        }).await
    }
}