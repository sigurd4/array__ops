use array_trait::Array;

#[const_trait]
pub trait ArrayForm<const N: usize>: ~const private::_ArrayForm<N, _T = <Self as ArrayForm<N>>::T, _Elem = <Self as ArrayForm<N>>::Elem>
{
    type T;
    type Elem;
}
impl<U, const N: usize> const ArrayForm<N> for U
where
    U: ~const private::_ArrayForm<N>
{
    type T = <U as private::_ArrayForm<N>>::_T;
    type Elem = <U as private::_ArrayForm<N>>::_Elem;
}

mod private
{
    use core::{marker::Destruct, mem::MaybeUninit, ops::{Index, RangeBounds}, slice::SliceIndex};

    use crate::{private, ArrayOps};

    #[const_trait]
    pub trait _ArrayForm<const N: usize>
    {
        type _T;
        type _Elem;
        type _MaybeUninit: _ArrayForm<N, _T = MaybeUninit<Self::_T>>;

        fn each_elem(self) -> [Self::_Elem; N];
        unsafe fn read_elem(&self, i: usize) -> Self::_Elem;
        fn copy_elem(&self, i: usize) -> Self::_Elem
        where
            Self::_Elem: Copy;
        unsafe fn drop_elems<R>(&mut self, i: R)
        where
            R: /*~const*/ SliceIndex<[Self::_T], Output = [Self::_T]> + ~const Destruct,
            Self::_Elem: ~const Destruct;
        fn maybe_uninit(self) -> Self::_MaybeUninit;
        fn each_elem_maybe_uninit(self) -> [MaybeUninit<Self::_Elem>; N];
        unsafe fn assume_init(maybe_uninit: Self::_MaybeUninit) -> Self;
        unsafe fn read_assume_init_elem(maybe_uninit: &Self::_MaybeUninit, i: usize) -> Self::_Elem;
        unsafe fn drop_elems_assume_init<R>(maybe_uninit: &mut Self::_MaybeUninit, i: R)
        where
            R: /*~const*/ SliceIndex<[MaybeUninit<Self::_T>], Output = [MaybeUninit<Self::_T>]> + ~const Destruct;
    }

    impl<T, const N: usize> /*const*/ _ArrayForm<N> for [T; N]
    {
        type _T = T;
        type _Elem = T;
        type _MaybeUninit = [MaybeUninit<T>; N];

        fn each_elem(self) -> [Self::_Elem; N]
        {
            self
        }
        unsafe fn read_elem(&self, i: usize) -> Self::_Elem
        {
            core::ptr::read(&self[i])
        }
        fn copy_elem(&self, i: usize) -> Self::_Elem
        where
            Self::_Elem: Copy
        {
            self[i]
        }
        unsafe fn drop_elems<R>(&mut self, i: R)
        where
            R: /*~const*/ SliceIndex<[Self::_T], Output = [Self::_T]>
        {
            core::ptr::drop_in_place(&mut self[i]);
        }
        fn maybe_uninit(self) -> Self::_MaybeUninit
        {
            unsafe {
                private::transmute_unchecked_size(self)
            }
        }
        fn each_elem_maybe_uninit(self) -> [MaybeUninit<Self::_Elem>; N]
        {
            self.each_elem().maybe_uninit()
        }
        unsafe fn assume_init(maybe_uninit: Self::_MaybeUninit) -> Self
        {
            private::transmute_unchecked_size(maybe_uninit)
        }
        unsafe fn read_assume_init_elem(maybe_uninit: &Self::_MaybeUninit, i: usize) -> Self::_Elem
        {
            MaybeUninit::assume_init_read(&maybe_uninit[i])
        }
        unsafe fn drop_elems_assume_init<R>(maybe_uninit: &mut Self::_MaybeUninit, i: R)
        where
            R: /*~const*/ SliceIndex<[MaybeUninit<Self::_T>], Output = [MaybeUninit<Self::_T>]>
        {
            core::ptr::drop_in_place(MaybeUninit::slice_assume_init_mut(&mut maybe_uninit[i]));
        }
    }
    impl<'a, T, const N: usize> const _ArrayForm<N> for &'a [T; N]
    where
        T: 'a
    {
        type _T = T;
        type _Elem = &'a T;
        type _MaybeUninit = &'a [MaybeUninit<T>; N];

        fn each_elem(self) -> [Self::_Elem; N]
        {
            self.each_ref()
        }
        unsafe fn read_elem(&self, i: usize) -> Self::_Elem
        {
            self.copy_elem(i)
        }
        fn copy_elem(&self, i: usize) -> Self::_Elem
        {
            &self[i]
        }
        unsafe fn drop_elems<R>(&mut self, _: R)
        where
            R: /*~const*/ SliceIndex<[Self::_T], Output = [Self::_T]> + ~const Destruct
        {

        }
        fn maybe_uninit(self) -> Self::_MaybeUninit
        {
            unsafe {
                core::mem::transmute(self)
            }
        }
        fn each_elem_maybe_uninit(self) -> [MaybeUninit<Self::_Elem>; N]
        {
            unsafe {
                private::transmute_unchecked_size(self.each_elem())
            }
        }
        unsafe fn assume_init(maybe_uninit: Self::_MaybeUninit) -> Self
        {
            core::mem::transmute(maybe_uninit)
        }
        unsafe fn read_assume_init_elem(maybe_uninit: &Self::_MaybeUninit, i: usize) -> Self::_Elem
        {
           MaybeUninit::assume_init_ref(&maybe_uninit[i])
        }
        unsafe fn drop_elems_assume_init<R>(_: &mut Self::_MaybeUninit, _: R)
        where
            R: /*~const*/ SliceIndex<[MaybeUninit<Self::_T>], Output = [MaybeUninit<Self::_T>]> + ~const Destruct
        {

        }
    }
    impl<'a, T, const N: usize> const _ArrayForm<N> for &'a mut [T; N]
    where
        T: 'a
    {
        type _T = T;
        type _Elem = &'a mut T;
        type _MaybeUninit = &'a mut [MaybeUninit<T>; N];

        fn each_elem(self) -> [Self::_Elem; N]
        {
            self.each_mut()
        }
        unsafe fn read_elem(&self, i: usize) -> Self::_Elem
        {
            (&self[i] as *const T).cast_mut().as_mut_unchecked()
        }
        fn copy_elem(&self, i: usize) -> Self::_Elem
        where
            Self::_Elem: Copy
        {
            unsafe {
                self.read_elem(i)
            }
        }
        unsafe fn drop_elems<R>(&mut self, _: R)
        where
            R: /*~const*/ SliceIndex<[Self::_T], Output = [Self::_T]> + ~const Destruct
        {

        }
        fn maybe_uninit(self) -> Self::_MaybeUninit
        {
            unsafe {
                core::mem::transmute(self)
            }
        }
        fn each_elem_maybe_uninit(self) -> [MaybeUninit<Self::_Elem>; N]
        {
            unsafe {
                private::transmute_unchecked_size(self.each_elem())
            }
        }
        unsafe fn assume_init(maybe_uninit: Self::_MaybeUninit) -> Self
        {
            core::mem::transmute(maybe_uninit)
        }
        unsafe fn read_assume_init_elem(maybe_uninit: &Self::_MaybeUninit, i: usize) -> Self::_Elem
        {
           (MaybeUninit::assume_init_ref(&maybe_uninit[i]) as *const T).cast_mut().as_mut_unchecked()
        }
        unsafe fn drop_elems_assume_init<R>(_: &mut Self::_MaybeUninit, _: R)
        where
            R: /*~const*/ SliceIndex<[MaybeUninit<Self::_T>], Output = [MaybeUninit<Self::_T>]> + ~const Destruct
        {

        }
    }
}