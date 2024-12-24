use core::mem::MaybeUninit;

use crate::ArrayForm;

use super::Dir;

pub(crate) struct PartialZipGuard<'a, A, B, U, const D: Dir, const N: usize>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    lhs: A::_MaybeUninit,
    rhs: B::_MaybeUninit,
    dst: &'a mut [MaybeUninit<U>; N],
    i: usize,
    err: bool
}

impl<'a, A, B, U, const N: usize> PartialZipGuard<'a, A, B, U, {Dir::Left}, N>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    pub const fn new_left(lhs: A, rhs: B, dst: &'a mut [MaybeUninit<U>; N]) -> Self
    where
        A: ~const ArrayForm<N>,
        B: ~const ArrayForm<N>
    {
        Self::new(lhs, rhs, dst)
    }
}
impl<'a, A, B, U, const N: usize> PartialZipGuard<'a, A, B, U, {Dir::Right}, N>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    pub const fn new_right(lhs: A, rhs: B, dst: &'a mut [MaybeUninit<U>; N]) -> Self
    where
        A: ~const ArrayForm<N>,
        B: ~const ArrayForm<N>
    {
        Self::new(lhs, rhs, dst)
    }
}

impl<'a, A, B, U, const D: Dir, const N: usize> PartialZipGuard<'a, A, B, U, D, N>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    pub const fn new(lhs: A, rhs: B, dst: &'a mut [MaybeUninit<U>; N]) -> Self
    where
        A: ~const ArrayForm<N>,
        B: ~const ArrayForm<N>
    {
        Self {
            lhs: lhs.maybe_uninit(),
            rhs: rhs.maybe_uninit(),
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

    pub fn zip<F>(&mut self, zipper: F)
    where
        F: FnOnce(A::Elem, B::Elem) -> U
    {
        let f = |j| unsafe {
            let dst = &mut self.dst[j];
            core::ptr::write(dst, MaybeUninit::new(zipper(
                A::read_assume_init_elem(&self.lhs, j),
                B::read_assume_init_elem(&self.rhs, j)
            )))
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
    pub fn try_zip<F, E>(&mut self, zipper: F) -> Result<(), E>
    where
        F: FnOnce(A::Elem, B::Elem) -> Result<U, E>
    {
        assert!(!self.err);
        let f = |j| unsafe {
            let dst = &mut self.dst[j];
            let result = zipper(
                A::read_assume_init_elem(&self.lhs, j),
                B::read_assume_init_elem(&self.rhs, j)
            );
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
                f(self.i)?;
                self.i += 1;
            },
            Dir::Right => {
                assert!(self.i > 0);
                let j = self.i - 1;
                f(j)?;
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

impl<'a, A, B, U, const D: Dir, const N: usize> /*const*/ Drop for PartialZipGuard<'a, A, B, U, D, N>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
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
            A::drop_elems_assume_init(&mut self.lhs, spans.0.clone());
            B::drop_elems_assume_init(&mut self.rhs, spans.0);
        }
    }
}