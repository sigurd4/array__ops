use core::marker::{ConstParamTy, Destruct};

pub(crate) struct PartiallyInitGuard<'a, T, const D: Dir>
{
    dst: &'a mut [MaybeUninit<T>],
    i: usize
}

impl<'a, T> PartiallyInitGuard<'a, T, {Dir::Left}>
{
    pub const fn new_left(dst: &'a mut [MaybeUninit<T>]) -> Self
    {
        Self::new(dst)
    }
}
impl<'a, T> PartiallyInitGuard<'a, T, {Dir::Right}>
{
    pub const fn new_right(dst: &'a mut [MaybeUninit<T>]) -> Self
    {
        Self::new(dst)
    }
}

impl<'a, T, const D: Dir> PartiallyInitGuard<'a, T, D>
{
    pub const fn new(dst: &'a mut [MaybeUninit<T>]) -> Self
    {
        Self {
            i: match D
            {
                Dir::Left => 0,
                Dir::Right => dst.len(),
            },
            dst
        }
    }

    pub const fn more(&self) -> bool
    {
        match D
        {
            Dir::Left => self.i < self.dst.len(),
            Dir::Right => self.i > 0,
        }
    }

    pub const fn push(&mut self, value: T)
    {
        match D
        {
            Dir::Left => {
                debug_assert!(self.i < self.dst.len());
                unsafe {
                    let dst = &mut self.dst[self.i];
                    core::ptr::write(dst, MaybeUninit::new(value))
                };
                self.i += 1;
            },
            Dir::Right => {
                debug_assert!(self.i > 0);
                let j = self.i - 1;
                unsafe {
                    let dst = &mut self.dst[j];
                    core::ptr::write(dst, MaybeUninit::new(value))
                };
                self.i = j;
            }
        }
    }
    pub /*const*/ fn push_by_fn<F>(&mut self, value: F)
    where
        F: FnOnce(usize) -> T
    {
        match D
        {
            Dir::Left => {
                debug_assert!(self.i < self.dst.len());
                unsafe {
                    let dst = &mut self.dst[self.i];
                    core::ptr::write(dst, MaybeUninit::new(value(self.i)))
                };
                self.i += 1;
            },
            Dir::Right => {
                debug_assert!(self.i > 0);
                let j = self.i - 1;
                unsafe {
                    let dst = &mut self.dst[j];
                    core::ptr::write(dst, MaybeUninit::new(value(j)))
                };
                self.i = j;
            }
        }
    }

    pub const fn done(self)
    {
        match D
        {
            Dir::Left => debug_assert!(self.i == self.dst.len()),
            Dir::Right => debug_assert!(self.i == 0)
        }
        core::mem::forget(self)
    }
}

impl<'a, T, const D: Dir> /*const*/ Drop for PartiallyInitGuard<'a, T, D>
where
    [T]: /*~const*/ Destruct
{
    fn drop(&mut self)
    {
        let target = match D
        {
            Dir::Left => &mut self.dst[..self.i],
            Dir::Right => &mut self.dst[self.i..]
        };
        unsafe {
            let target = core::mem::transmute::<_, &mut [T]>(target);
            core::ptr::drop_in_place(target);
        }
    }
}

#[cfg(feature = "alloc")]
pub mod boxed_array
{
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
}

#[derive(ConstParamTy, PartialEq, Eq)]
pub(crate) enum Dir
{
    Left,
    Right
}

#[repr(C)]
pub(crate) struct Pair<L, R>
{
    pub left: L,
    pub right: R
}

impl<L, R> Pair<L, R>
{
    pub const fn new(left: L, right: R) -> Self
    {
        Self {left, right}
    }

    pub const fn unpack(self) -> (L, R)
    {
        unsafe {
            let mut left_right: (L, R) = uninit();

            core::ptr::copy_nonoverlapping(&self.left as *const L, &mut left_right.0 as *mut L, 1);
            core::ptr::copy_nonoverlapping(&self.right as *const R, &mut left_right.1 as *mut R, 1);

            core::mem::forget(self);

            left_right
        }
    }

    pub const fn pack(left_right: (L, R)) -> Self
    {
        unsafe {
            let mut pair: Self = uninit();

            core::ptr::copy_nonoverlapping(&left_right.0 as *const L, &mut pair.left as *mut L, 1);
            core::ptr::copy_nonoverlapping(&left_right.1 as *const R, &mut pair.right as *mut R, 1);

            core::mem::forget(left_right);

            pair
        }
    }
    
    pub const fn unpack_mandrop(self) -> (ManuallyDrop<L>, ManuallyDrop<R>)
    {
        unsafe {
            let mut left_right: (ManuallyDrop<L>, ManuallyDrop<R>) = uninit();

            core::ptr::copy_nonoverlapping(&self.left as *const L, (&mut left_right.0 as *mut ManuallyDrop<L>).cast(), 1);
            core::ptr::copy_nonoverlapping(&self.right as *const R, (&mut left_right.1 as *mut ManuallyDrop<R>).cast(), 1);

            core::mem::forget(self);

            left_right
        }
    }
}

impl<L, R> From<(L, R)> for Pair<L, R>
{
    fn from(left_right: (L, R)) -> Self
    {
        Self::pack(left_right)
    }
}

impl<L, R> Into<(L, R)> for Pair<L, R>
{
    fn into(self) -> (L, R)
    {
        self.unpack()
    }
}

use core::mem::{ManuallyDrop, MaybeUninit};

/*impl<T, const P: &'static [usize]> NotTuple for PartitionedArray<T, P>
where
[(); crate::sum_len::<{P}>()]: {}*/

#[deprecated]
pub(crate) const unsafe fn uninit<T>() -> T
{
    MaybeUninit::assume_init(MaybeUninit::uninit())
}

pub(crate) const unsafe fn split_transmute<A, B, C>(a: A) -> (B, C)
{
    transmute_unchecked_size::<_, Pair<_, _>>(a).unpack()
}

pub(crate) const unsafe fn merge_transmute<A, B, C>(a: A, b: B) -> C
{
    transmute_unchecked_size(Pair::new(a, b))
}

pub(crate) const unsafe fn overlap_swap_transmute<A, B>(a: A, b: B) -> (B, A)
{
    split_transmute(Pair::new(a, b))
}

pub(crate) const unsafe fn transmute_unchecked_size<A, B>(from: A) -> B
{
    /*#[cfg(test)]
    if core::mem::size_of::<A>() != core::mem::size_of::<B>() && core::mem::align_of::<A>() != core::mem::align_of::<B>()
    {
        panic!("Cannot transmute due to unequal size or alignment")
    }*/
    
    let b = unsafe {core::mem::transmute_copy(&from)};
    core::mem::forget(from);
    b

    //core::ptr::read(core::mem::transmute(&ManuallyDrop::new(from)))
    
    /*union Transmutation<A, B>
    {
        a: ManuallyDrop<A>,
        b: ManuallyDrop<B>
    }

    unsafe {ManuallyDrop::into_inner(Transmutation {a: ManuallyDrop::new(a)}.b)}*/
}