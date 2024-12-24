
use core::{mem::MaybeUninit, alloc::Allocator};

use alloc::{alloc::Global, boxed::Box};

use super::transmute_unchecked_size;

pub fn new_uninit<T, const N: usize>() -> Box<[MaybeUninit<T>; N]>
{
    new_uninit_in(Global)
}
pub fn new_uninit_in<T, A, const N: usize>(alloc: A) -> Box<[MaybeUninit<T>; N], A>
where
    A: Allocator
{
    let boxed = Box::<[T; N], A>::new_uninit_in(alloc);
    unsafe {
        transmute_unchecked_size(boxed)
    }
}

pub unsafe fn assume_init<T, A, const N: usize>(boxed: Box<[MaybeUninit<T>; N], A>) -> Box<[T; N], A>
where
    A: Allocator
{
    transmute_unchecked_size(boxed)
}