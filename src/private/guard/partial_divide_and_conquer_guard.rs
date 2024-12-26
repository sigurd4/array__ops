use core::mem::MaybeUninit;

use crate::form::ArrayForm;

use super::Dir;

pub(crate) struct PartialDivideAndConquerGuard<T, const D: Dir, const N: usize>
{
    src: [MaybeUninit<T>; N],
    i: usize,
    j: usize
}

impl<T, const N: usize> PartialDivideAndConquerGuard<T, {Dir::Left}, N>
{
    pub const fn new_left<A>(src: A) -> Self
    where
        A: ~const ArrayForm<N, Elem = T>
    {
        Self::new(src)
    }
}
impl<T, const N: usize> PartialDivideAndConquerGuard<T, {Dir::Right}, N>
{
    pub const fn new_right<A>(src: A) -> Self
    where
        A: ~const ArrayForm<N, Elem = T>
    {
        Self::new(src)
    }
}

impl<T, const D: Dir, const N: usize> PartialDivideAndConquerGuard<T, D, N>
{
    pub const fn new<A>(src: A) -> Self
    where
        A: ~const ArrayForm<N, Elem = T>
    {
        Self {
            src: src.each_elem_maybe_uninit(),
            i: match D
            {
                Dir::Left => 0,
                Dir::Right => N - 1,
            },
            j: 1
        }
    }
    
    pub const fn more(&self) -> bool
    {
        self.j != 0 && self.j < N
    }

    pub fn pop(&mut self) -> (&mut T, T)
    {
        let f1 = |i| {
            unsafe {
                (MaybeUninit::assume_init_ref(&self.src[i]) as *const T).cast_mut().as_mut_unchecked()
            }
        };
        let f2 = |i| {
            unsafe {
                MaybeUninit::assume_init_read(&self.src[i])
            }
        };
        loop
        {
            assert!(self.i < N);
            assert!(self.j < N);
            return match D
            {
                Dir::Left => {
                    let dst = f1(self.i);
                    self.i += self.j;
                    if self.i > N
                    {
                        self.i = 0;
                        self.j *= 2;
                        continue
                    }
                    let value = f2(self.i);
                    if self.i > N
                    {
                        self.i = 0;
                        self.j *= 2;
                    }
                    (dst, value)
                },
                Dir::Right => {
                    let dst = f1(self.i);
                    if self.i < self.j
                    {
                        self.i = N;
                        self.j *= 2;
                        continue
                    }
                    self.i -= self.j;
                    let value = f2(self.i);
                    if self.i < self.j
                    {
                        self.i = N;
                        self.j *= 2;
                    }
                    self.i -= self.j;
                    (dst, value)
                }
            }
        }
    }

    pub fn reduce<F>(mut self, mut reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T
    {
        while self.more()
        {
            let (dst, value) = self.pop();
            unsafe {
                core::ptr::write(dst, reduce(core::ptr::read(dst), value));
            }
        }
        let value = if N > 0
        {
            self.j = 0;
            unsafe {
                Some(MaybeUninit::assume_init_read(&self.src[0]))
            }
        }
        else
        {
            None
        };
        self.done();

        value
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

impl<T, const D: Dir, const N: usize> /*const*/ Drop for PartialDivideAndConquerGuard<T, D, N>
{
    fn drop(&mut self)
    {
        if N > 0 && self.j > 0
        {
            let i0 = self.i;
            while match D
            {
                Dir::Left => self.i >= self.j,
                Dir::Right => self.i + self.j < N
            }
            {
                match D
                {
                    Dir::Left => self.i -= self.j,
                    Dir::Right => self.i += self.j
                }
                unsafe {
                    core::ptr::drop_in_place(&mut self.src[self.i]);
                }
            }
            self.j /= 2;

            if self.j > 0
            {
                self.i = match D
                {
                    Dir::Left => 0,
                    Dir::Right => N - 1
                };
                while match D
                {
                    Dir::Left => self.i >= i0 + self.j,
                    Dir::Right => self.i + self.j <= i0
                }
                {
                    match D
                    {
                        Dir::Left => self.i -= self.j,
                        Dir::Right => self.i += self.j
                    }
                    unsafe {
                        core::ptr::drop_in_place(&mut self.src[self.i]);
                    }
                }
            }
        }
    }
}