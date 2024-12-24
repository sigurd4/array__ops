use core::mem::MaybeUninit;

use crate::ArrayForm;

use super::Dir;

pub(crate) struct PartialMapGuard<'a, A, U, const D: Dir, const N: usize>
where
    A: ArrayForm<N>
{
    src: A::_MaybeUninit,
    dst: &'a mut [MaybeUninit<U>; N],
    i: usize,
    err: bool
}

impl<'a, A, U, const N: usize> PartialMapGuard<'a, A, U, {Dir::Left}, N>
where
    A: ArrayForm<N>
{
    pub const fn new_left(src: A, dst: &'a mut [MaybeUninit<U>; N]) -> Self
    where
        A: ~const ArrayForm<N>
    {
        Self::new(src, dst)
    }
}
impl<'a, A, U, const N: usize> PartialMapGuard<'a, A, U, {Dir::Right}, N>
where
    A: ArrayForm<N>
{
    pub const fn new_right(src: A, dst: &'a mut [MaybeUninit<U>; N]) -> Self
    where
        A: ~const ArrayForm<N>
    {
        Self::new(src, dst)
    }
}

impl<'a, A, U, const D: Dir, const N: usize> PartialMapGuard<'a, A, U, D, N>
where
    A: ArrayForm<N>
{
    pub const fn new(src: A, dst: &'a mut [MaybeUninit<U>; N]) -> Self
    where
        A: ~const ArrayForm<N>
    {
        Self {
            src: src.maybe_uninit(),
            dst,
            i: match D
            {
                Dir::Left => 0,
                Dir::Right => N,
            },
            err: false
        }
    }

    pub const fn more(&self) -> bool
    {
        match D
        {
            Dir::Left => self.i < N,
            Dir::Right => self.i > 0,
        }
    }

    pub fn map<F>(&mut self, mapper: F)
    where
        F: FnOnce(A::Elem) -> U
    {
        let f = |j| unsafe {
            let dst = &mut self.dst[j];
            core::ptr::write(dst, MaybeUninit::new(mapper(A::read_assume_init_elem(&mut self.src, j))))
        };
        match D
        {
            Dir::Left => {
                assert!(self.i < N);
                f(self.i);
                self.i += 1;
            },
            Dir::Right => {
                assert!(self.i > 0);
                let j = self.i - 1;
                f(j);
                self.i = j;
            }
        }
    }
    pub fn try_map<F, E>(&mut self, mapper: F) -> Result<(), E>
    where
        F: FnOnce(A::Elem) -> Result<U, E>
    {
        assert!(!self.err);
        let f = |j| unsafe {
            let dst = &mut self.dst[j];
            let result = mapper(A::read_assume_init_elem(&mut self.src, j));
            match result
            {
                Err(error) => {
                    self.err = true;
                    Err(error)
                }
                Ok(value) => {
                    core::ptr::write(dst, MaybeUninit::new(value));
                    Ok(())
                }
            }
        };
        match D
        {
            Dir::Left => {
                assert!(self.i < N);
                f(self.i);
                self.i += 1;
            },
            Dir::Right => {
                assert!(self.i > 0);
                let j = self.i - 1;
                f(j);
                self.i = j;
            }
        }
        Ok(())
    }

    pub const fn done(self)
    {
        match D
        {
            Dir::Left => assert!(self.i == N),
            Dir::Right => assert!(self.i == 0)
        }
        core::mem::forget(self)
    }
}

impl<'a, A, U, const D: Dir, const N: usize> /*const*/ Drop for PartialMapGuard<'a, A, U, D, N>
where
    A: ArrayForm<N>
{
    fn drop(&mut self)
    {
        let spans = match D
        {
            Dir::Left => (
                self.i + self.err as usize..N,
                0..self.i
            ),
            Dir::Right => (
                0..self.i - self.err as usize,
                self.i..N
            )
        };
        unsafe {
            core::ptr::drop_in_place(MaybeUninit::slice_assume_init_mut(&mut self.dst[spans.1]));
            A::drop_elems_assume_init(&mut self.src, spans.0);
        }
    }
}