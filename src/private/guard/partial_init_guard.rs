use core::mem::MaybeUninit;

use super::Dir;


pub(crate) struct PartialInitGuard<'a, T, const D: Dir, const N: usize>
{
    dst: &'a mut [MaybeUninit<T>; N],
    i: usize
}

impl<'a, T, const N: usize> PartialInitGuard<'a, T, {Dir::Left}, N>
{
    pub const fn new_left(dst: &'a mut [MaybeUninit<T>; N]) -> Self
    {
        Self::new(dst)
    }
}
impl<'a, T, const N: usize> PartialInitGuard<'a, T, {Dir::Right}, N>
{
    pub const fn new_right(dst: &'a mut [MaybeUninit<T>; N]) -> Self
    {
        Self::new(dst)
    }
}

impl<'a, T, const D: Dir, const N: usize> PartialInitGuard<'a, T, D, N>
{
    pub const fn new(dst: &'a mut [MaybeUninit<T>; N]) -> Self
    {
        Self {
            dst,
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

    #[allow(unused)]
    pub /*const*/ fn push(&mut self, value: T)
    {
        self.push_by_fn(|_| value);
    }
    pub /*const*/ fn push_by_fn<F>(&mut self, value: F)
    where
        F: FnOnce(usize) -> T
    {
        let f = |j| unsafe {
            let dst = &mut self.dst[j];
            core::ptr::write(dst, MaybeUninit::new(value(j)))
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
    pub /*const*/ fn try_push_by_fn<F, E>(&mut self, value: F) -> Result<(), E>
    where
        F: FnOnce(usize) -> Result<T, E>
    {
        let f = |j| unsafe {
            let dst = &mut self.dst[j];
            let result = value(j);
            match result
            {
                Err(error) => Err(error),
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

impl<'a, T, const D: Dir, const N: usize> /*const*/ Drop for PartialInitGuard<'a, T, D, N>
{
    fn drop(&mut self)
    {
        let span = match D
        {
            Dir::Left => 0..self.i,
            Dir::Right => self.i..N
        };
        unsafe {
            core::ptr::drop_in_place(MaybeUninit::slice_assume_init_mut(&mut self.dst[span]));
        }
    }
}