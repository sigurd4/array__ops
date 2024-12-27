use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use crate::form::ArrayForm;

use super::{Enumerate, ZipWith};

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
    fn enumerate_zip_pin_ref_with<'a, Zip, Rhs>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, Pin<&'a T>, Rhs::Elem)> + ~const Destruct;
    fn enumerate_zip_pin_mut_with<'a, Zip, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, Pin<&'a mut T>, Rhs::Elem)> + ~const Destruct;
        
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
    async fn enumerate_zip_pin_ref_async_with<'a, Zip, Rhs>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, Pin<&'a T>, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn enumerate_zip_pin_mut_async_with<'a, Zip, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, Pin<&'a mut T>, Rhs::Elem)> + ~const Destruct,
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
    fn try_enumerate_zip_pin_ref_with<'a, Zip, Rhs, U, E>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, Pin<&'a T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_zip_pin_mut_with<'a, Zip, Rhs, U, E>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
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
    async fn try_enumerate_zip_pin_ref_async_with<'a, Zip, Rhs, U, E>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, Pin<&'a T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_zip_pin_mut_async_with<'a, Zip, Rhs, U, E>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> EnumerateZipWith<T, N> for [T; N]
{
    fn enumerate_zip_with<Zip, Rhs>(self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, T, Rhs::Elem)> + Destruct
    {
        r#impl::enumerate_zip_with(self, rhs, zipper)
    }
    fn enumerate_zip_ref_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, &'a T, Rhs::Elem)>
    {
        r#impl::enumerate_zip_with(self, rhs, zipper)
    }
    fn enumerate_zip_mut_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, &'a mut T, Rhs::Elem)>
    {
        r#impl::enumerate_zip_with(self, rhs, zipper)
    }
    fn enumerate_zip_pin_ref_with<'a, Zip, Rhs>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, Pin<&'a T>, Rhs::Elem)>
    {
        r#impl::enumerate_zip_with(self, rhs, zipper)
    }
    fn enumerate_zip_pin_mut_with<'a, Zip, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, Pin<&'a mut T>, Rhs::Elem)>
    {
        r#impl::enumerate_zip_with(self, rhs, zipper)
    }
    
    async fn enumerate_zip_async_with<Zip, Rhs>(self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, T, Rhs::Elem)>
    {
        r#impl::enumerate_zip_async_with(self, rhs, zipper).await
    }
    async fn enumerate_zip_ref_async_with<'a, Zip, Rhs>(&'a self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, &'a T, Rhs::Elem)>,
        T: 'a
    {
        r#impl::enumerate_zip_async_with(self, rhs, zipper).await
    }
    async fn enumerate_zip_mut_async_with<'a, Zip, Rhs>(&'a mut self, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, &'a mut T, Rhs::Elem)>,
        T: 'a
    {
        r#impl::enumerate_zip_async_with(self, rhs, zipper).await
    }
    async fn enumerate_zip_pin_ref_async_with<'a, Zip, Rhs>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, Pin<&'a T>, Rhs::Elem)>,
        T: 'a
    {
        r#impl::enumerate_zip_async_with(self, rhs, zipper).await
    }
    async fn enumerate_zip_pin_mut_async_with<'a, Zip, Rhs>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, Pin<&'a mut T>, Rhs::Elem)>,
        T: 'a
    {
        r#impl::enumerate_zip_async_with(self, rhs, zipper).await
    }
    
    fn try_enumerate_zip_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, T, Rhs::Elem) -> Result<U, E>
    {
        r#impl::try_enumerate_zip_with(self, rhs, zipper)
    }
    fn try_enumerate_zip_ref_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, &'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_with(self, rhs, zipper)
    }
    fn try_enumerate_zip_mut_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, &'a mut T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_with(self, rhs, zipper)
    }
    fn try_enumerate_zip_pin_ref_with<'a, Zip, Rhs, U, E>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, Pin<&'a T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_with(self, rhs, zipper)
    }
    fn try_enumerate_zip_pin_mut_with<'a, Zip, Rhs, U, E>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_with(self, rhs, zipper)
    }
    
    async fn try_enumerate_zip_async_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, T, Rhs::Elem) -> Result<U, E>
    {
        r#impl::try_enumerate_zip_async_with(self, rhs, zipper).await
    }
    async fn try_enumerate_zip_ref_async_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, &'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_async_with(self, rhs, zipper).await
    }
    async fn try_enumerate_zip_mut_async_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, &'a mut T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_async_with(self, rhs, zipper).await
    }
    async fn try_enumerate_zip_pin_ref_async_with<'a, Zip, Rhs, U, E>(self: Pin<&'a Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, Pin<&'a T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_async_with(self, rhs, zipper).await
    }
    async fn try_enumerate_zip_pin_mut_async_with<'a, Zip, Rhs, U, E>(self: Pin<&'a mut Self>, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, Pin<&'a mut T>, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_zip_async_with(self, rhs, zipper).await
    }
}

mod r#impl
{
    use core::{ops::AsyncFn, mem::MaybeUninit};

    use crate::{form::ArrayForm, ops::ArrayJoin, private::guard::{Dir, PartialZipGuard}};

    pub(super) fn enumerate_zip_with<const N: usize, Zip, Lhs, Rhs>(lhs: Lhs, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, Lhs::Elem, Rhs::Elem)>
    {
        enumerate_dzip_with::<{Dir::Left}, _, _, _, _>(lhs, rhs, zipper)
    }
    pub(super) fn enumerate_rzip_with<const N: usize, Zip, Lhs, Rhs>(lhs: Lhs, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, Lhs::Elem, Rhs::Elem)>
    {
        enumerate_dzip_with::<{Dir::Right}, _, _, _, _>(lhs, rhs, zipper)
    }
    pub(super) async fn enumerate_zip_async_with<const N: usize, Zip, Lhs, Rhs>(lhs: Lhs, rhs: Rhs, zipper: Zip) -> [Zip::Output; N]
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        Zip: AsyncFn<(usize, Lhs::Elem, Rhs::Elem)>
    {
        enumerate_zip_with(lhs, rhs, |i, x, y| zipper(i, x, y)).join_runs().await
    }
    fn enumerate_dzip_with<const D: Dir, const N: usize, Zip, Lhs, Rhs>(lhs: Lhs, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        Zip: FnMut<(usize, Lhs::Elem, Rhs::Elem)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::<_, _, _, D, _>::new(
            lhs,
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

    pub(super) fn try_enumerate_zip_with<const N: usize, Zip, Lhs, Rhs, U, E>(lhs: Lhs, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, Lhs::Elem, Rhs::Elem) -> Result<U, E>
    {
        try_enumerate_dzip_with::<{Dir::Left}, _, _, _, _, _, _>(lhs, rhs, zipper)
    }
    pub(super) fn try_enumerate_rzip_with<const N: usize, Zip, Lhs, Rhs, U, E>(lhs: Lhs, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, Lhs::Elem, Rhs::Elem) -> Result<U, E>
    {
        try_enumerate_dzip_with::<{Dir::Right}, _, _, _, _, _, _>(lhs, rhs, zipper)
    }
    pub(super) async fn try_enumerate_zip_async_with<const N: usize, Zip, Lhs, Rhs, U, E>(lhs: Lhs, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(usize, Lhs::Elem, Rhs::Elem) -> Result<U, E>
    {
        enumerate_zip_with(lhs, rhs, |i, x, y| zipper(i, x, y)).try_join_runs().await
    }
    fn try_enumerate_dzip_with<const D: Dir, const N: usize, Zip, Lhs, Rhs, U, E>(lhs: Lhs, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Lhs: ArrayForm<N>,
        Rhs: ArrayForm<N>,
        Zip: FnMut(usize, Lhs::Elem, Rhs::Elem) -> Result<U, E>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            lhs,
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
}