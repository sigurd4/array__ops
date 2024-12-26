use crate::form::ArrayForm;

use super::Dir;


pub(crate) struct PartialEmptyGuard<A, const D: Dir, const N: usize>
where
    A: ArrayForm<N>
{
    src: A::_MaybeUninit,
    i: usize
}

impl<A, const N: usize> PartialEmptyGuard<A, {Dir::Left}, N>
where
    A: ArrayForm<N>
{
    pub const fn new_left(src: A) -> Self
    where
        A: ~const ArrayForm<N>
    {
        Self::new(src)
    }
}
impl<A, const N: usize> PartialEmptyGuard<A, {Dir::Right}, N>
where
    A: ArrayForm<N>
{
    pub const fn new_right(src: A) -> Self
    where
        A: ~const ArrayForm<N>
    {
        Self::new(src)
    }
}

impl<A, const D: Dir, const N: usize> PartialEmptyGuard<A, D, N>
where
    A: ArrayForm<N>
{
    pub const fn new(src: A) -> Self
    where
        A: ~const ArrayForm<N>
    {
        Self {
            src: src.maybe_uninit(),
            i: match D
            {
                Dir::Left => 0,
                Dir::Right => N,
            }
        }
    }

    pub const fn index(&self) -> usize
    {
        self.i
    }
    
    pub const fn more(&self) -> bool
    {
        match D
        {
            Dir::Left => self.i < N,
            Dir::Right => self.i > 0,
        }
    }

    pub fn pop(&mut self) -> A::Elem
    {
        let f = |j| unsafe {
            A::read_assume_init_elem(&self.src, j)
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

    pub fn reduce<F>(mut self, mut reduce: F) -> Option<A::Elem>
    where
        F: FnMut(A::Elem, A::Elem) -> A::Elem
    {
        let mut value = None;
        if self.more()
        {
            let value = value.insert(self.pop());
            while self.more()
            {
                unsafe {
                    core::ptr::write(value, reduce(core::ptr::read(value), self.pop()));
                }
            }
        }
        self.done();

        value
    }

    pub fn fold<U, F>(mut self, mut default: U, mut fold: F) -> U
    where
        F: FnMut(U, A::Elem) -> U
    {
        while self.more()
        {
            unsafe {
                core::ptr::write(&mut default, fold(core::ptr::read(&mut default), self.pop()));
            }
        }
        self.done();

        default
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

impl<A, const D: Dir, const N: usize> /*const*/ Drop for PartialEmptyGuard<A, D, N>
where
    A: ArrayForm<N>
{
    fn drop(&mut self)
    {
        let span = match D
        {
            Dir::Left => self.i..N,
            Dir::Right => 0..self.i
        };
        unsafe {
            A::drop_elems_assume_init(&mut self.src, span);
        }
    }
}