use array_trait::Array;

use crate::form::MutForm;

#[const_trait]
pub trait ArrayShift<T, const N: usize>: Array<Item = T>
{
    fn shift_many_left<'a, const M: usize, I>(&mut self, items: I)
    where
        I: ~const MutForm<'a, [T; M]>,
        T: 'a;
    fn shift_many_right<'a, const M: usize, I>(&mut self, items: I)
    where
        I: ~const MutForm<'a, [T; M]>,
        T: 'a;
    
    fn shift_left<'a, I>(&mut self, item: I)
    where
        I: ~const MutForm<'a, T>,
        T: 'a;
    fn shift_right<'a, I>(&mut self, item: I)
    where
        I: ~const MutForm<'a, T>,
        T: 'a;
}

impl<T, const N: usize> ArrayShift<T, N> for [T; N]
{
    fn shift_many_left<'a, const M: usize, I>(&mut self, mut items: I)
    where
        I: MutForm<'a, [T; M]>,
        T: 'a
    {
        let items_mut = items.as_mut();
        if M == N
        {
            core::mem::swap(
                self,
                unsafe {
                    items_mut.as_mut_ptr().cast::<[T; N]>().as_mut_unchecked()
                }
            );
        }
        else
        {
            let dst = self.as_mut_ptr();
            let src = items_mut.as_mut_ptr();
            if M < N
            {
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
                            src,
                            dst.add(N - M),
                            M
                        );
                        core::ptr::write(items_mut, buffer);
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
                            src,
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
                            &mut items_mut[0..M - N]
                        );
                    }
                    core::mem::forget(items);
                }
            }
        }
    }
    fn shift_many_right<'a, const M: usize, I>(&mut self, mut items: I)
    where
        I: MutForm<'a, [T; M]>,
        T: 'a
    {
        let items_mut = items.as_mut();
        if M == N
        {
            core::mem::swap(
                self,
                unsafe {
                    items_mut.as_mut_ptr().cast::<[T; N]>().as_mut_unchecked()
                }
            );
        }
        else
        {
            let dst = self.as_mut_ptr();
            let src = items_mut.as_mut_ptr();
            if M < N
            {
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
                            src,
                            dst,
                            M
                        );
                        core::ptr::write(items_mut, buffer);
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
                            src,
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
                            &mut items_mut[M - N..N]
                        );
                    }
                    core::mem::forget(items);
                }
            }
        }
    }
    
    fn shift_left<'a, I>(&mut self, mut item: I)
    where
        I: MutForm<'a, T>,
        T: 'a
    {
        if I::IS_MUT
        {
            self.shift_many_left(core::array::from_mut(item.as_mut()));
        }
        else
        {
            self.shift_many_left(unsafe {
                [item.read()]
            });
        }
    }
    fn shift_right<'a, I>(&mut self, mut item: I)
    where
        I: MutForm<'a, T>,
        T: 'a
    {
        if I::IS_MUT
        {
            self.shift_many_right(core::array::from_mut(item.as_mut()));
        }
        else
        {
            self.shift_many_right(unsafe {
                [item.read()]
            });
        }
    }
}