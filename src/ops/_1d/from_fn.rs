use core::{marker::Destruct, mem::MaybeUninit, ops::AsyncFn};

use array_trait::Array;

use crate::private::guard::PartialInitGuard;

use super::ArrayJoin;

#[const_trait]
pub trait FromFn<T, const N: usize>: Array<Item = T>
{
    fn from_fn<F>(fill: F) -> Self
    where
        F: FnMut(usize) -> T + ~const Destruct;
    fn rfrom_fn<F>(fill: F) -> Self
    where
        F: FnMut(usize) -> T + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn from_fn_boxed<F>(fill: F) -> Box<Self>
    where
        F: FnMut(usize) -> T + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn rfrom_fn_boxed<F>(fill: F) -> Box<Self>
    where
        F: FnMut(usize) -> T + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn from_fn_boxed_in<F, A>(fill: F, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T + ~const Destruct,
        A: Allocator;
    #[cfg(feature = "alloc")]
    fn rfrom_fn_boxed_in<F, A>(fill: F, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T + ~const Destruct,
        A: Allocator;
        
    fn try_from_fn<F, E>(fill: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct;
    fn try_rfrom_fn<F, E>(fill: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn try_from_fn_boxed<F, E>(fill: F) -> Result<Box<Self>, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn try_rfrom_fn_boxed<F, E>(fill: F) -> Result<Box<Self>, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn try_from_fn_boxed_in<F, E, A>(fill: F, alloc: A) -> Result<Box<Self, A>, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct,
        A: Allocator;
    #[cfg(feature = "alloc")]
    fn try_rfrom_fn_boxed_in<F, E, A>(fill: F, alloc: A) -> Result<Box<Self, A>, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct,
        A: Allocator;

    async fn from_fn_async<F>(fill: F) -> Self
    where
        F: AsyncFn(usize) -> T + ~const Destruct;
    async fn try_from_fn_async<F, E>(fill: F) -> Result<Self, E>
    where
        F: AsyncFn(usize) -> Result<T, E> + ~const Destruct;
}

impl<T, const N: usize> FromFn<T, N> for [T; N]
{
    fn from_fn<F>(mut fill: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        let mut array = MaybeUninit::uninit_array();
        let mut guard = PartialInitGuard::new_left(&mut array);

        while guard.more()
        {
            guard.push_by_fn(&mut fill)
        }

        guard.done();

        unsafe {
            MaybeUninit::array_assume_init(array)
        }
    }
    fn rfrom_fn<F>(mut fill: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        let mut array = MaybeUninit::uninit_array();
        let mut guard = PartialInitGuard::new_right(&mut array);

        while guard.more()
        {
            guard.push_by_fn(&mut fill)
        }

        guard.done();

        unsafe {
            MaybeUninit::array_assume_init(array)
        }
    }
    #[cfg(feature = "alloc")]
    fn from_fn_boxed<F>(fill: F) -> Box<Self>
    where
        F: FnMut(usize) -> T
    {
        Self::from_fn_boxed_in(fill, Global)
    }
    #[cfg(feature = "alloc")]
    fn rfrom_fn_boxed<F>(fill: F) -> Box<Self>
    where
        F: FnMut(usize) -> T
    {
        Self::rfrom_fn_boxed_in(fill, Global)
    }
    #[cfg(feature = "alloc")]
    fn from_fn_boxed_in<F, A>(mut fill: F, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T,
        A: Allocator
    {
        let mut array = private::boxed_array::new_uninit_in(alloc);
        let mut guard = PartialInitGuard::new_left(&mut *array);

        while guard.more()
        {
            guard.push_by_fn(&mut fill)
        }

        guard.done();

        unsafe {
            private::boxed_array::assume_init(array)
        }
    }
    #[cfg(feature = "alloc")]
    fn rfrom_fn_boxed_in<F, A>(mut fill: F, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T,
        A: Allocator
    {
        let mut array = private::boxed_array::new_uninit_in(alloc);
        let mut guard = PartialInitGuard::new_right(&mut *array);

        while guard.more()
        {
            guard.push_by_fn(&mut fill)
        }

        guard.done();

        unsafe {
            private::boxed_array::assume_init(array)
        }
    }
    
    fn try_from_fn<F, E>(mut fill: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        let mut array = MaybeUninit::uninit_array();
        let mut guard = PartialInitGuard::new_left(&mut array);

        while guard.more()
        {
            guard.try_push_by_fn(&mut fill)?
        }

        guard.done();

        unsafe {
            Ok(MaybeUninit::array_assume_init(array))
        }
    }
    fn try_rfrom_fn<F, E>(mut fill: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        let mut array = MaybeUninit::uninit_array();
        let mut guard = PartialInitGuard::new_right(&mut array);

        while guard.more()
        {
            guard.try_push_by_fn(&mut fill)?
        }

        guard.done();

        unsafe {
            Ok(MaybeUninit::array_assume_init(array))
        }
    }
    #[cfg(feature = "alloc")]
    fn try_from_fn_boxed<F, E>(fill: F) -> Result<Box<Self>, E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        Self::try_from_fn_boxed_in(fill, Global)
    }
    #[cfg(feature = "alloc")]
    fn try_rfrom_fn_boxed<F, E>(fill: F) -> Result<Box<Self>, E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        Self::try_rfrom_fn_boxed_in(fill, Global)
    }
    #[cfg(feature = "alloc")]
    fn try_from_fn_boxed_in<F, E, A>(mut fill: F, alloc: A) -> Result<Box<Self, A>, E>
    where
        F: FnMut(usize) -> Result<T, E>,
        A: Allocator
    {
        let mut array = private::boxed_array::new_uninit_in(alloc);
        let mut guard = PartialInitGuard::new_left(&mut *array);

        while guard.more()
        {
            guard.try_push_by_fn(&mut fill)?
        }

        guard.done();

        unsafe {
            Ok(private::boxed_array::assume_init(array))
        }
    }
    #[cfg(feature = "alloc")]
    fn try_rfrom_fn_boxed_in<F, E, A>(mut fill: F, alloc: A) -> Result<Box<Self, A>, E>
    where
        F: FnMut(usize) -> Result<T, E>,
        A: Allocator
    {
        let mut array = private::boxed_array::new_uninit_in(alloc);
        let mut guard = PartialInitGuard::new_right(&mut *array);

        while guard.more()
        {
            guard.try_push_by_fn(&mut fill)?
        }

        guard.done();

        unsafe {
            Ok(private::boxed_array::assume_init(array))
        }
    }

    async fn from_fn_async<F>(fill: F) -> Self
    where
        F: AsyncFn(usize) -> T
    {
        crate::from_fn(|i| fill(i)).join_runs().await
    }
    async fn try_from_fn_async<F, E>(fill: F) -> Result<Self, E>
    where
        F: AsyncFn(usize) -> Result<T, E>
    {
        crate::from_fn(|i| fill(i)).try_join_runs().await
    }
}