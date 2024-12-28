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
    #[allow(unused)]
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
                Dir::Right => const {N - 1},
            },
            j: 1
        }
    }
    
    pub const fn more(&self) -> bool
    {
        self.j < N && (self.j < const {N - 1} || {
            match D
            {
                Dir::Left => self.i + self.j < N,
                Dir::Right => self.i >= self.j
            }
        })
    }

    fn incr(&mut self, skip: bool)
    {
        let mut jj = self.j;
        if skip
        {
            jj += jj
        }
        match D
        {
            Dir::Left => {
                if self.i + jj >= N
                {
                    self.i = 0;
                    self.j *= 2
                }
                else
                {
                    self.i += self.j
                }
            },
            Dir::Right => {
                if self.i < jj
                {
                    self.i = const {N - 1};
                    self.j *= 2
                }
                else
                {
                    self.i -= self.j
                }
            }
        }
    }

    fn get(&mut self) -> *mut T
    {
        let f = #[inline] |this: &Self, i| {
            unsafe {
                (MaybeUninit::assume_init_ref(&this.src.get_unchecked(i)) as *const T).cast_mut().as_mut_unchecked()
            }
        };
        
        let dst = f(self, self.i);
        self.incr(false);
        dst
    }
    fn pop(&mut self) -> T
    {
        let f = #[inline] |this: &Self, i| {
            unsafe {
                MaybeUninit::assume_init_read(&this.src.get_unchecked(i))
            }
        };
        
        let dst = f(self, self.i);
        self.incr(true);
        dst
    }

    pub fn reduce<F>(mut self, mut reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T
    {
        if N <= 0
        {
            return None
        }
        while self.more()
        {
            let dst = self.get();
            let value = self.pop();
            unsafe {
                core::ptr::write(dst, reduce(core::ptr::read(dst), value));
            }
        }
        let value = unsafe {
            MaybeUninit::assume_init_read(&self.src.get_unchecked(self.i))
        };
        self.done();

        Some(value)
    }

    pub const fn done(self)
    {
        debug_assert!(self.j >= N);
        match D
        {
            Dir::Left => debug_assert!(self.i == 0),
            Dir::Right => debug_assert!(self.i == N - 1)
        }
        core::mem::forget(self)
    }
}

impl<T, const D: Dir, const N: usize> Drop for PartialDivideAndConquerGuard<T, D, N>
{
    fn drop(&mut self)
    {
        let end = (self.i + self.j).min(N);
        let less = |this: &Self| match D
        {
            Dir::Left => this.i >= this.j,
            Dir::Right => this.i < N - this.j
        };
        let decr = |this: &mut Self| match D
        {
            Dir::Left => this.i -= this.j,
            Dir::Right => this.i += this.j
        };
        let kill = |this: &mut Self| if let Some(x) = this.src.get_mut(this.i)
        {
            unsafe {
                core::ptr::drop_in_place(x);
            }
        };
        while less(self)
        {
            kill(self);
            decr(self);
        }
        let less = |this: &Self| match D
        {
            Dir::Left => this.i > end + this.j,
            Dir::Right => this.i < end - this.j
        };
        while less(self)
        {
            kill(self);
            decr(self);
        }
    }
}