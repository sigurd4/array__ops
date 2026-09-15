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
    #[allow(unused)]
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
        assert!(self.more());
        match D
        {
            Dir::Left => self.i,
            Dir::Right => self.i - 1
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

    pub fn pop(&mut self) -> A::Elem
    {
        assert!(self.more());
        let f = |j| unsafe {
            A::read_assume_init_elem(&self.src, j)
        };
        match D
        {
            Dir::Left => {
                let value = f(self.i);
                self.i += 1;
                value
            },
            Dir::Right => {
                self.i -= 1;
                f(self.i)
            }
        }
    }

    pub fn map_reduce<F1, F2>(mut self, mut mapper: F1, mut reduce: F2) -> Option<F1::Output>
    where
        F1: FnMut<(A::Elem,)>,
        F2: FnMut(F1::Output, F1::Output) -> F1::Output
    {
        let mut value = None;
        if self.more()
        {
            let value = value.insert(mapper(self.pop()));
            while self.more()
            {
                unsafe {
                    core::ptr::write(value, reduce(core::ptr::read(value), mapper(self.pop())));
                }
            }
        }
        self.done();

        value
    }

    pub fn map_fold<U, F1, F2>(mut self, mut mapper: F1, mut default: U, mut fold: F2) -> U
    where
        F1: FnMut<(A::Elem,)>,
        F2: FnMut(U, F1::Output) -> U
    {
        while self.more()
        {
            unsafe {
                core::ptr::write(&mut default, fold(core::ptr::read(&default), mapper(self.pop())));
            }
        }
        self.done();

        default
    }

    pub fn reduce<F>(self, reduce: F) -> Option<A::Elem>
    where
        F: FnMut(A::Elem, A::Elem) -> A::Elem
    {
        self.map_reduce(|z| z, reduce)
    }

    pub fn fold<U, F>(self, default: U, fold: F) -> U
    where
        F: FnMut(U, A::Elem) -> U
    {
        self.map_fold(|z| z, default, fold)
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