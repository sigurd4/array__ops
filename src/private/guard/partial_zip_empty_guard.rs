use crate::ArrayForm;

use super::Dir;


pub(crate) struct PartialZipEmptyGuard<A, B, const D: Dir, const N: usize>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    lhs: A::_MaybeUninit,
    rhs: B::_MaybeUninit,
    i: usize
}

impl<A, B, const N: usize> PartialZipEmptyGuard<A, B, {Dir::Left}, N>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    pub const fn new_left(lhs: A, rhs: B) -> Self
    where
        A: ~const ArrayForm<N>,
        B: ~const ArrayForm<N>
    {
        Self::new(lhs, rhs)
    }
}
impl<A, B, const N: usize> PartialZipEmptyGuard<A, B, {Dir::Right}, N>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    pub const fn new_right(lhs: A, rhs: B) -> Self
    where
        A: ~const ArrayForm<N>,
        B: ~const ArrayForm<N>
    {
        Self::new(lhs, rhs)
    }
}

impl<A, B, const D: Dir, const N: usize> PartialZipEmptyGuard<A, B, D, N>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    pub const fn new(lhs: A, rhs: B) -> Self
    where
        A: ~const ArrayForm<N>,
        B: ~const ArrayForm<N>
    {
        Self {
            lhs: lhs.maybe_uninit(),
            rhs: rhs.maybe_uninit(),
            i: match D
            {
                Dir::Left => 0,
                Dir::Right => N,
            }
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

    pub fn pop(&mut self) -> (A::Elem, B::Elem)
    {
        let f = |j| unsafe {
            (
                A::read_assume_init_elem(&self.lhs, j),
                B::read_assume_init_elem(&self.rhs, j)
            )
        };
        match D
        {
            Dir::Left => {
                assert!(self.i < N);
                let value = f(self.i);
                self.i += 1;
                value
            },
            Dir::Right => {
                assert!(self.i > 0);
                self.i -= 1;
                f(self.i)
            }
        }
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

impl<A, B, const D: Dir, const N: usize> /*const*/ Drop for PartialZipEmptyGuard<A, B, D, N>
where
    A: ArrayForm<N>,
    B: ArrayForm<N>
{
    fn drop(&mut self)
    {
        let span = match D
        {
            Dir::Left => self.i..N,
            Dir::Right => 0..self.i
        };
        unsafe {
            B::drop_elems_assume_init(&mut self.rhs, span.clone());
            A::drop_elems_assume_init(&mut self.lhs, span);
        }
    }
}