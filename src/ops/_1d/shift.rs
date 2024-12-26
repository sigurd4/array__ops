use array_trait::Array;

use crate::MutForm;

#[const_trait]
pub trait ArrayShift<T, const N: usize>: Array<Item = T>
{
    fn shift_many_left<const M: usize, I>(&mut self, items: I)
    where
        I: MutForm<[T; M]>;
    fn shift_many_right<const M: usize, I>(&mut self, items: I)
    where
        I: MutForm<[T; M]>;
    
    fn shift_left<I>(&mut self, item: I)
    where
        I: MutForm<T>;
    fn shift_right<I>(&mut self, item: I)
    where
        I: MutForm<T>;
}

impl<T, const N: usize> const ArrayShift<T, N> for [T; N]
{
    fn shift_many_left<const M: usize, I>(&mut self, mut items: I)
    where
        I: MutForm<[T; M]>
    {
        if M == N
        {
            core::mem::swap(self, items.get_mut());
        }
        else
        {
            let dst = self.as_mut_ptr();
            let src = items.get_mut().as_mut_ptr();
            if M < N
            {
                let shift = || unsafe {
                };
                if I::IS_MUT
                {
                    let buffer = unsafe {
                        dst.cast::<[T; M]>().read()
                    };
                    unsafe {
                        core::ptr::copy(
                            dst.add(M),
                            dst,
                            N - M
                        );
                        core::ptr::copy_nonoverlapping(
                            shift,
                            dst.add(N - M),
                            M
                        );
                        core::ptr::write(items, buffer);
                    }
                }
                else
                {
                    unsafe {
                        core::ptr::drop_in_place(
                            &mut self[0..M]
                        );
                        core::ptr::copy(
                            dst.add(M),
                            dst,
                            N - M
                        );
                        core::ptr::copy_nonoverlapping(
                            shift,
                            dst.add(N - M),
                            M
                        );
                    }
                    core::mem::forget(items);
                }
            }
            else
            {
                if I::IS_MUT
                {
                    let buffer = unsafe {
                        src.add(M - N).cast::<[T; N]>().read()
                    };
                    unsafe {
                        core::ptr::copy(
                            src,
                            src.add(N),
                            M - N
                        );
                        core::ptr::copy_nonoverlapping(
                            dst,
                            src,
                            N
                        );
                        core::ptr::write(self, buffer);
                    }
                }
                else
                {
                    unsafe {
                        core::ptr::drop_in_place(self);
                        core::ptr::copy_nonoverlapping(
                            src.add(M - N),
                            dst,
                            N
                        );
                        core::ptr::drop_in_place(
                            &mut items.get_mut()[0..M - N]
                        );
                    }
                    core::mem::forget(items);
                }
            }
        }
    }
    fn shift_many_right<const M: usize, I>(&mut self, mut items: I)
    where
        I: MutForm<[T; M]>
    {
        if M == N
        {
            core::mem::swap(self, items.get_mut());
        }
        else
        {
            let dst = self.as_mut_ptr();
            let src = items.get_mut().as_mut_ptr();
            if M < N
            {
                let shift = || unsafe {
                };
                if I::IS_MUT
                {
                    let buffer = unsafe {
                        dst.add(N - M).cast::<[T; M]>().read()
                    };
                    unsafe {
                        core::ptr::copy(
                            dst,
                            dst.add(M),
                            N - M
                        );
                        core::ptr::copy_nonoverlapping(
                            shift,
                            dst,
                            M
                        );
                        core::ptr::write(items, buffer);
                    }
                }
                else
                {
                    unsafe {
                        core::ptr::drop_in_place(
                            &mut self[N - M..N]
                        );
                        core::ptr::copy(
                            dst,
                            dst.add(M),
                            N - M
                        );
                        core::ptr::copy_nonoverlapping(
                            shift,
                            dst,
                            M
                        );
                    }
                    core::mem::forget(items);
                }
            }
            else
            {
                if I::IS_MUT
                {
                    let buffer = unsafe {
                        src.cast::<[T; N]>().read()
                    };
                    unsafe {
                        core::ptr::copy(
                            src.add(N),
                            src,
                            M - N
                        );
                        core::ptr::copy_nonoverlapping(
                            dst,
                            src.add(M - N),
                            N
                        );
                        core::ptr::write(self, buffer);
                    }
                }
                else
                {
                    unsafe {
                        core::ptr::drop_in_place(self);
                        core::ptr::copy_nonoverlapping(
                            src,
                            dst,
                            N
                        );
                        core::ptr::drop_in_place(
                            &mut items.get_mut()[M - N..N]
                        );
                    }
                    core::mem::forget(items);
                }
            }
        }
    }
    
    fn shift_left<I>(&mut self, item: I)
    where
        I: MutForm<T>
    {
        self.shift_many_left([item]);
    }
    fn shift_right<I>(&mut self, item: I)
    where
        I: MutForm<T>
    {
        self.shift_many_right([item]);
    }
}