use crate::ops::FromFn;

use core::{marker::Destruct, ops::AsyncFn};

pub const fn from_fn<T, const N: usize, F>(fill: F) -> [T; N]
where
    F: FnMut(usize) -> T + ~const Destruct,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::from_fn(fill)
}
pub const fn rfrom_fn<T, const N: usize, F>(fill: F) -> [T; N]
where
    F: FnMut(usize) -> T + ~const Destruct,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::rfrom_fn(fill)
}
#[cfg(feature = "alloc")]
pub const fn from_fn_boxed<T, const N: usize, F>(fill: F) -> Box<[T; N]>
where
    F: FnMut(usize) -> T + ~const Destruct,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::from_fn_boxed(fill)
}
#[cfg(feature = "alloc")]
pub const fn rfrom_fn_boxed<T, const N: usize, F>(fill: F) -> Box<[T; N]>
where
    F: FnMut(usize) -> T + ~const Destruct,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::rfrom_fn_boxed(fill)
}
#[cfg(feature = "alloc")]
pub const fn from_fn_boxed_in<T, const N: usize, F, A>(fill: F, alloc: A) -> Box<[T; N], A>
where
    F: FnMut(usize) -> T + ~const Destruct,
    A: Allocator,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::from_fn_boxed_in(fill, alloc)
}
#[cfg(feature = "alloc")]
pub const fn rfrom_fn_boxed_in<T, const N: usize, F, A>(mut fill: F, alloc: A) -> Box<[T; N], A>
where
    F: FnMut(usize) -> T + ~const Destruct,
    A: Allocator,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::rfrom_fn_boxed_in(fill, alloc)
}

pub const fn try_from_fn<T, const N: usize, F, E>(fill: F) -> Result<[T; N], E>
where
    F: FnMut(usize) -> Result<T, E> + ~const Destruct,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::try_from_fn(fill)
}
pub const fn try_rfrom_fn<T, const N: usize, F, E>(fill: F) -> Result<[T; N], E>
where
    F: FnMut(usize) -> Result<T, E> + ~const Destruct,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::try_rfrom_fn(fill)
}
#[cfg(feature = "alloc")]
pub const fn try_from_fn_boxed<T, const N: usize, F, E>(fill: F) -> Result<Box<[T; N]>, E>
where
    F: FnMut(usize) -> Result<T, E> + ~const Destruct,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::try_from_fn_boxed(fill)
}
#[cfg(feature = "alloc")]
pub const fn try_rfrom_fn_boxed<T, const N: usize, F, E>(fill: F) -> Result<Box<[T; N]>, E>
where
    F: FnMut(usize) -> Result<T, E> + ~const Destruct,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::try_rfrom_fn_boxed(fill)
}
#[cfg(feature = "alloc")]
pub const fn try_from_fn_boxed_in<T, const N: usize, F, E, A>(fill: F, alloc: A) -> Result<Box<[T; N], A>, E>
where
    F: FnMut(usize) -> Result<T, E> + ~const Destruct,
    A: Allocator,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::try_from_fn_boxed_in(fill, alloc)
}
#[cfg(feature = "alloc")]
pub const fn try_rfrom_fn_boxed_in<T, const N: usize, F, E, A>(mut fill: F, alloc: A) -> Result<Box<[T; N], A>, E>
where
    F: FnMut(usize) -> Result<T, E> + ~const Destruct,
    A: Allocator,
    [T; N]: ~const FromFn<T, N>
{
    <[T; N]>::try_rfrom_fn_boxed_in(fill, alloc)
}

pub async fn from_fn_async<T, const N: usize, F>(fill: F) -> [T; N]
where
    F: AsyncFn(usize) -> T
{
    <[T; N]>::from_fn_async(fill).await
}
pub async fn try_from_fn_async<T, const N: usize, F, E>(fill: F) -> Result<[T; N], E>
where
    F: AsyncFn(usize) -> Result<T, E>
{
    <[T; N]>::try_from_fn_async(fill).await
}