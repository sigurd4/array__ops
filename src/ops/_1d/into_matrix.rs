use core::{marker::Destruct, pin::Pin};

use array_trait::Array;
use slice_ops::AsSlice;

use crate::private::guard::PartialEmptyGuard;

#[const_trait]
pub trait ArrayIntoMatrix<T, const N: usize>: Array + AsSlice<Item = T>
{
    fn diagonal_matrix<const H: usize, const W: usize>(self) -> [[T; W]; H]
    where
        T: Default,
        [(); H - N]:,
        [(); W - N]:;

    fn diagonal_or_matrix<const H: usize, const W: usize>(self, default: T) -> [[T; W]; H]
    where
        T: Copy,
        [(); H - N]:,
        [(); W - N]:;
    fn diagonal_or_matrix_ref<'a, const H: usize, const W: usize>(&'a self, default: &'a T) -> [[&'a T; W]; H]
    where
        T: 'a,
        [(); H - N]:,
        [(); W - N]:;
    fn diagonal_or_matrix_pin_ref<'a, const H: usize, const W: usize>(self: Pin<&'a Self>, default: Pin<&'a T>) -> [[Pin<&'a T>; W]; H]
    where
        T: 'a,
        [(); H - N]:,
        [(); W - N]:;

    fn diagonal_or_else_matrix<const H: usize, const W: usize, F>(self, default: F) -> [[T; W]; H]
    where
        F: FnMut(usize, usize) -> T,
        [(); H - N]:,
        [(); W - N]:;
    fn diagonal_or_else_matrix_ref<'a, const H: usize, const W: usize, F>(&'a self, default: F) -> [[&'a T; W]; H]
    where
        F: FnMut(usize, usize) -> &'a T,
        [(); H - N]:,
        [(); W - N]:;
    fn diagonal_or_else_matrix_mut<'a, const H: usize, const W: usize, F>(&'a mut self, default: F) -> [[&'a mut T; W]; H]
    where
        F: FnMut(usize, usize) -> &'a mut T,
        [(); H - N]:,
        [(); W - N]:;
    fn diagonal_or_else_matrix_pin_ref<'a, const H: usize, const W: usize, F>(self: Pin<&'a Self>, default: F) -> [[Pin<&'a T>; W]; H]
    where
        F: FnMut(usize, usize) -> Pin<&'a T>,
        [(); H - N]:,
        [(); W - N]:;
    fn diagonal_or_else_matrix_pin_mut<'a, const H: usize, const W: usize, F>(self: Pin<&'a mut Self>, default: F) -> [[Pin<&'a mut T>; W]; H]
    where
        F: FnMut(usize, usize) -> Pin<&'a mut T>,
        [(); H - N]:,
        [(); W - N]:;

    fn diagonal_matrix_exact(self) -> [[T; N]; N]
    where
        T: Default;
        
    fn diagonal_or_matrix_exact(self, default: T) -> [[T; N]; N]
    where
        T: Copy;
    fn diagonal_or_matrix_exact_ref<'a>(&'a self, default: &'a T) -> [[&'a T; N]; N]
    where
        T: 'a;
    fn diagonal_or_matrix_exact_pin_ref<'a>(self: Pin<&'a Self>, default: Pin<&'a T>) -> [[Pin<&'a T>; N]; N]
    where
        T: 'a;

    fn diagonal_or_else_matrix_exact<F>(self, default: F) -> [[T; N]; N]
    where
        F: FnMut(usize, usize) -> T;
    fn diagonal_or_else_matrix_exact_ref<'a, F>(&'a self, default: F) -> [[&'a T; N]; N]
    where
        F: FnMut(usize, usize) -> &'a T;
    fn diagonal_or_else_matrix_exact_mut<'a, F>(&'a mut self, default: F) -> [[&'a mut T; N]; N]
    where
        F: FnMut(usize, usize) -> &'a mut T;
    fn diagonal_or_else_matrix_exact_pin_ref<'a, F>(self: Pin<&'a Self>, default: F) -> [[Pin<&'a T>; N]; N]
    where
        F: FnMut(usize, usize) -> Pin<&'a T>;
    fn diagonal_or_else_matrix_exact_pin_mut<'a, F>(self: Pin<&'a mut Self>, default: F) -> [[Pin<&'a mut T>; N]; N]
    where
        F: FnMut(usize, usize) -> Pin<&'a mut T>;

    fn diagonal_matrix_truncate<const H: usize, const W: usize>(self) -> [[T; W]; H]
    where
        T: Default + ~const Destruct;

    fn diagonal_or_matrix_truncate<const H: usize, const W: usize>(self, default: T) -> [[T; W]; H]
    where
        T: Copy + ~const Destruct;
    fn diagonal_or_matrix_truncate_ref<'a, const H: usize, const W: usize>(&'a self, default: &'a T) -> [[&'a T; W]; H]
    where
        T: ~const Destruct + 'a;
    fn diagonal_or_matrix_truncate_pin_ref<'a, const H: usize, const W: usize>(self: Pin<&'a Self>, default: Pin<&'a T>) -> [[Pin<&'a T>; W]; H]
    where
        T: ~const Destruct + 'a;

    fn diagonal_or_else_matrix_truncate<const H: usize, const W: usize, F>(self, default: F) -> [[T; W]; H]
    where
        F: FnMut(usize, usize) -> T,
        T: ~const Destruct;
    fn diagonal_or_else_matrix_truncate_ref<'a, const H: usize, const W: usize, F>(&'a self, default: F) -> [[&'a T; W]; H]
    where
        F: FnMut(usize, usize) -> &'a T,
        T: ~const Destruct;
    fn diagonal_or_else_matrix_truncate_mut<'a, const H: usize, const W: usize, F>(&'a mut self, default: F) -> [[&'a mut T; W]; H]
    where
        F: FnMut(usize, usize) -> &'a mut T,
        T: ~const Destruct;
    fn diagonal_or_else_matrix_truncate_pin_ref<'a, const H: usize, const W: usize, F>(self: Pin<&'a Self>, default: F) -> [[Pin<&'a T>; W]; H]
    where
        F: FnMut(usize, usize) -> Pin<&'a T>,
        T: ~const Destruct;
    fn diagonal_or_else_matrix_truncate_pin_mut<'a, const H: usize, const W: usize, F>(self: Pin<&'a mut Self>, default: F) -> [[Pin<&'a mut T>; W]; H]
    where
        F: FnMut(usize, usize) -> Pin<&'a mut T>,
        T: ~const Destruct;

    fn toeplitz_matrix(&self) -> [[T; N]; N]
    where
        T: Copy;
    fn toeplitz_matrix_ref(&self) -> [[&T; N]; N];
    fn toeplitz_matrix_pin_ref(self: Pin<&Self>) -> [[Pin<&T>; N]; N];

    fn hankel_matrix<const M: usize>(&self, r: &[T; M]) -> [[T; M]; N]
    where
        T: Copy;
    fn hankel_matrix_ref<'a, const M: usize>(&'a self, r: &'a [T; M]) -> [[&'a T; M]; N];
    fn hankel_matrix_pin_ref<'a, const M: usize>(self: Pin<&'a Self>, r: Pin<&'a [T; M]>) -> [[Pin<&'a T>; M]; N];
}

impl<T, const N: usize> ArrayIntoMatrix<T, N> for [T; N]
{
    fn diagonal_matrix<const H: usize, const W: usize>(self) -> [[T; W]; H]
    where
        T: Default,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_matrix_truncate(self)
    }

    fn diagonal_or_matrix<const H: usize, const W: usize>(self, default: T) -> [[T; W]; H]
    where
        T: Copy,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }
    fn diagonal_or_matrix_ref<'a, const H: usize, const W: usize>(&'a self, default: &'a T) -> [[&'a T; W]; H]
    where
        T: 'a,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }
    fn diagonal_or_matrix_pin_ref<'a, const H: usize, const W: usize>(self: Pin<&'a Self>, default: Pin<&'a T>) -> [[Pin<&'a T>; W]; H]
    where
        T: 'a,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }

    fn diagonal_or_else_matrix<const H: usize, const W: usize, F>(self, default: F) -> [[T; W]; H]
    where
        F: FnMut(usize, usize) -> T,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_ref<'a, const H: usize, const W: usize, F>(&'a self, default: F) -> [[&'a T; W]; H]
    where
        F: FnMut(usize, usize) -> &'a T,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_mut<'a, const H: usize, const W: usize, F>(&'a mut self, default: F) -> [[&'a mut T; W]; H]
    where
        F: FnMut(usize, usize) -> &'a mut T,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_pin_ref<'a, const H: usize, const W: usize, F>(self: Pin<&'a Self>, default: F) -> [[Pin<&'a T>; W]; H]
    where
        F: FnMut(usize, usize) -> Pin<&'a T>,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_pin_mut<'a, const H: usize, const W: usize, F>(self: Pin<&'a mut Self>, default: F) -> [[Pin<&'a mut T>; W]; H]
    where
        F: FnMut(usize, usize) -> Pin<&'a mut T>,
        [(); H - N]:,
        [(); W - N]:
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }

    fn diagonal_matrix_exact(self) -> [[T; N]; N]
    where
        T: Default
    {
        r#impl::diagonal_matrix_truncate(self)
    }
        
    fn diagonal_or_matrix_exact(self, default: T) -> [[T; N]; N]
    where
        T: Copy
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }
    fn diagonal_or_matrix_exact_ref<'a>(&'a self, default: &'a T) -> [[&'a T; N]; N]
    where
        T: 'a
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }
    fn diagonal_or_matrix_exact_pin_ref<'a>(self: Pin<&'a Self>, default: Pin<&'a T>) -> [[Pin<&'a T>; N]; N]
    where
        T: 'a
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }

    fn diagonal_or_else_matrix_exact<F>(self, default: F) -> [[T; N]; N]
    where
        F: FnMut(usize, usize) -> T
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_exact_ref<'a, F>(&'a self, default: F) -> [[&'a T; N]; N]
    where
        F: FnMut(usize, usize) -> &'a T
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_exact_mut<'a, F>(&'a mut self, default: F) -> [[&'a mut T; N]; N]
    where
        F: FnMut(usize, usize) -> &'a mut T
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_exact_pin_ref<'a, F>(self: Pin<&'a Self>, default: F) -> [[Pin<&'a T>; N]; N]
    where
        F: FnMut(usize, usize) -> Pin<&'a T>
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_exact_pin_mut<'a, F>(self: Pin<&'a mut Self>, default: F) -> [[Pin<&'a mut T>; N]; N]
    where
        F: FnMut(usize, usize) -> Pin<&'a mut T>
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }

    fn diagonal_matrix_truncate<const H: usize, const W: usize>(self) -> [[T; W]; H]
    where
        T: Default
    {
        r#impl::diagonal_matrix_truncate(self)
    }

    fn diagonal_or_matrix_truncate<const H: usize, const W: usize>(self, default: T) -> [[T; W]; H]
    where
        T: Copy
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }
    fn diagonal_or_matrix_truncate_ref<'a, const H: usize, const W: usize>(&'a self, default: &'a T) -> [[&'a T; W]; H]
    where
        T: 'a
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }
    fn diagonal_or_matrix_truncate_pin_ref<'a, const H: usize, const W: usize>(self: Pin<&'a Self>, default: Pin<&'a T>) -> [[Pin<&'a T>; W]; H]
    where
        T: 'a
    {
        r#impl::diagonal_or_matrix_truncate(self, default)
    }

    fn diagonal_or_else_matrix_truncate<const H: usize, const W: usize, F>(self, default: F) -> [[T; W]; H]
    where
        F: FnMut(usize, usize) -> T
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_truncate_ref<'a, const H: usize, const W: usize, F>(&'a self, default: F) -> [[&'a T; W]; H]
    where
        F: FnMut(usize, usize) -> &'a T
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_truncate_mut<'a, const H: usize, const W: usize, F>(&'a mut self, default: F) -> [[&'a mut T; W]; H]
    where
        F: FnMut(usize, usize) -> &'a mut T
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_truncate_pin_ref<'a, const H: usize, const W: usize, F>(self: Pin<&'a Self>, default: F) -> [[Pin<&'a T>; W]; H]
    where
        F: FnMut(usize, usize) -> Pin<&'a T>
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }
    fn diagonal_or_else_matrix_truncate_pin_mut<'a, const H: usize, const W: usize, F>(self: Pin<&'a mut Self>, default: F) -> [[Pin<&'a mut T>; W]; H]
    where
        F: FnMut(usize, usize) -> Pin<&'a mut T>
    {
        r#impl::diagonal_or_else_matrix_truncate(self, default)
    }

    fn toeplitz_matrix(&self) -> [[T; N]; N]
    where
        T: Copy
    {
        r#impl::toeplitz_matrix(self)
    }
    fn toeplitz_matrix_ref(&self) -> [[&T; N]; N]
    {
        r#impl::toeplitz_matrix(&self)
    }
    fn toeplitz_matrix_pin_ref(self: Pin<&Self>) -> [[Pin<&T>; N]; N]
    {
        r#impl::toeplitz_matrix(&self)
    }

    fn hankel_matrix<const M: usize>(&self, r: &[T; M]) -> [[T; M]; N]
    where
        T: Copy
    {
        r#impl::hankel_matrix(self, r)
    }
    fn hankel_matrix_ref<'a, const M: usize>(&'a self, r: &'a [T; M]) -> [[&'a T; M]; N]
    {
        r#impl::hankel_matrix(&self, &r)
    }
    fn hankel_matrix_pin_ref<'a, const M: usize>(self: Pin<&'a Self>, r: Pin<&'a [T; M]>) -> [[Pin<&'a T>; M]; N]
    {
        r#impl::hankel_matrix(&self, &r)
    }
}

mod r#impl
{
    use crate::form::ArrayForm;

    use super::PartialEmptyGuard;

    #[const_trait]
    trait Spec<const N: usize>: ArrayForm<N, Elem: Default>
    {
        fn diagonal<const H: usize, const W: usize>(self) -> [[Self::Elem; W]; H];
    }
    impl<const N: usize, A> Spec<N> for A
    where
        A: ArrayForm<N, Elem: Default>
    {
        default fn diagonal<const H: usize, const W: usize>(self) -> [[Self::Elem; W]; H]
        {
            diagonal_or_else_matrix_truncate(self, |_, _| Default::default())
        }
    }
    impl<const N: usize, A> Spec<N> for A
    where
        A: ArrayForm<N, Elem: Default + Copy>
    {
        fn diagonal<const H: usize, const W: usize>(self) -> [[Self::Elem; W]; H]
        {
            diagonal_or_matrix_truncate(self, Default::default())
        }
    }

    pub(super) fn diagonal_matrix_truncate<A, const N: usize, const H: usize, const W: usize>(array: A) -> [[A::Elem; W]; H]
    where
        A: ArrayForm<N, Elem: Default>
    {
        Spec::diagonal(array)
    }
    
    pub(super) fn diagonal_or_matrix_truncate<A, const N: usize, const H: usize, const W: usize>(array: A, default: A::Elem) -> [[A::Elem; W]; H]
    where
        A: ArrayForm<N, Elem: Copy>
    {
        let mut guard = PartialEmptyGuard::new_left(array);

        let mut dst = [[default; W]; H];

        while guard.more()
        {
            let i = guard.index();
            dst[i][i] = guard.pop();
        }

        if N <= W && N <= H
        {
            guard.done();
        }

        dst
    }

    pub(super) fn diagonal_or_else_matrix_truncate<A, const N: usize, const H: usize, const W: usize, F>(array: A, mut default: F) -> [[A::Elem; W]; H]
    where
        A: ArrayForm<N>,
        F: FnMut(usize, usize) -> A::Elem
    {
        let mut guard = PartialEmptyGuard::new_left(array);

        let dst = crate::from_fn(|i| crate::from_fn(|j| if i == j && guard.more()
            {
                guard.pop()
            }
            else
            {
                default(i, j)
            }
        ));

        if N <= W && N <= H
        {
            guard.done();
        }
    
        dst
    }

    pub(super) fn toeplitz_matrix<const N: usize, A>(array: &A) -> [[A::Elem; N]; N]
    where
        A: ArrayForm<N, Elem: Copy>
    {
        crate::from_fn(|i| crate::from_fn(|j| array.copy_elem(if i >= j {i - j} else {j - i})))
    }

    pub(super) fn hankel_matrix<const N: usize, const M: usize, A, B>(array: &A, r: &B) -> [[A::Elem; M]; N]
    where
        A: ArrayForm<N, Elem: Copy>,
        B: ArrayForm<M, Elem = A::Elem>
    {
        crate::from_fn(|i| crate::from_fn(|j| if i + j < N
        {
            array.copy_elem(i + j)
        }
        else
        {
            r.copy_elem(i + j + 1 - N)
        }))
    }
}