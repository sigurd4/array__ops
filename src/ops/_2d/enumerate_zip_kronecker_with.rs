use core::marker::Destruct;

use array_trait::Array;
use slice_ops::AsSlice;

use crate::form::ArrayForm;

#[const_trait]
pub trait ArrayEnumerateZipKroneckerWith<T, const M: usize, const N: usize>: Array + AsSlice<Item = [T; N]>
{
    fn enumerate_zip_kronecker_with<Rhs, const H: usize, const W: usize, F>(&self, rhs: &Rhs, zipper: F) -> [[F::Output; N*W]; M*H]
    where
        T: Copy,
        Rhs: ArrayForm<H, Elem: ArrayForm<W, Elem: Copy>>,
        F: FnMut<(usize, usize, usize, usize, T, <Rhs::Elem as ArrayForm<W>>::Elem)> + ~const Destruct;
}

impl<T, const M: usize, const N: usize> ArrayEnumerateZipKroneckerWith<T, M, N> for [[T; N]; M]
{
    fn enumerate_zip_kronecker_with<Rhs, const H: usize, const W: usize, F>(&self, rhs: &Rhs, zipper: F) -> [[F::Output; N*W]; M*H]
    where
        T: Copy,
        Rhs: ArrayForm<H, Elem: ArrayForm<W, Elem: Copy>>,
        F: FnMut<(usize, usize, usize, usize, T, <Rhs::Elem as ArrayForm<W>>::Elem)>
    {
        r#impl::enumerate_zip_kronecker_with(self, rhs, zipper)
    }
}

mod r#impl
{
    use crate::{form::ArrayForm, private::{self, guard::Dir}};

    pub(super) fn enumerate_zip_kronecker_with<const MA: usize, const NA: usize, const MB: usize, const NB: usize, A, B, F, U>(
        lhs: &A,
        rhs: &B,
        zipper: F
    ) -> [[U; NA*NB]; MA*MB]
    where
        A: ArrayForm<MA, Elem: ArrayForm<NA, Elem: Copy>>,
        B: ArrayForm<MB, Elem: ArrayForm<NB, Elem: Copy>>,
        F: FnMut(usize, usize, usize, usize, <A::Elem as ArrayForm<NA>>::Elem, <B::Elem as ArrayForm<NB>>::Elem) -> U
    {
        enumerate_dzip_kronecker_with::<{Dir::Left}, {Dir::Left}, _, _, _, _, _, _, _, _>(lhs, rhs, zipper)
    }
    fn enumerate_dzip_kronecker_with<const DV: Dir, const DH: Dir, const MA: usize, const NA: usize, const MB: usize, const NB: usize, A, B, F, U>(
        lhs: &A,
        rhs: &B,
        mut zipper: F
    ) -> [[U; NA*NB]; MA*MB]
    where
        A: ArrayForm<MA, Elem: ArrayForm<NA, Elem: Copy>>,
        B: ArrayForm<MB, Elem: ArrayForm<NB, Elem: Copy>>,
        F: FnMut(usize, usize, usize, usize, <A::Elem as ArrayForm<NA>>::Elem, <B::Elem as ArrayForm<NB>>::Elem) -> U
    {
        // Copies the whole A matrix, then performs the zipping in-place in the new matrix.
        // Doesn't make a massive improvement in asm-complexity, but may be slightly better memory-wise (not sure)
        // Sadly, this does not guard the destructors of the data properly, and needs to be wrapped in a new guard-type
        #[allow(unused)]
        fn same_size_lhs<const DV: Dir, const DH: Dir, const MA: usize, const NA: usize, const MB: usize, const NB: usize, A, B, F, U>(
            lhs: &A,
            rhs: &B,
            mut zipper: F
        ) -> [[U; NA*NB]; MA*MB]
        where
            A: ArrayForm<MA, Elem: ArrayForm<NA, Elem: Copy>>,
            B: ArrayForm<MB, Elem: ArrayForm<NB, Elem: Copy>>,
            F: FnMut(usize, usize, usize, usize, <A::Elem as ArrayForm<NA>>::Elem, <B::Elem as ArrayForm<NB>>::Elem) -> U
        {
            let lhs: [[<A::Elem as ArrayForm<NA>>::Elem; NA]; MA] = unsafe {
                // This is ok, because A will be Copy if its elements are Copy
                core::ptr::read(lhs).each_elem_2d()
            };
            let mut m = unsafe {
                private::transmute::<_, [[U; NA*NB]; MA*MB]>([[lhs; NB]; MB])
            };
            let dst = unsafe {
                (&mut m as *mut [[U; NA*NB]; MA*MB]).cast::<[[[[<A::Elem as ArrayForm<NA>>::Elem; NA]; MA]; NB]; MB]>().as_mut_unchecked()
            };

            let start = #[inline] |d, n| match d
            {
                Dir::Left => 0,
                Dir::Right => n,
            };

            let more = #[inline] |d, i, n| match d
            {
                Dir::Left => i < n,
                Dir::Right => i > 0
            };

            let incr = #[inline] |d, f, i: &mut usize| if d == f
            {
                match d
                {
                    Dir::Left => *i += 1,
                    Dir::Right => *i -= 1
                }
            };
    
            let mut h = start(DV, MB);
            while more(DV, h, MB)
            {
                incr(DV, Dir::Right, &mut h);
                let mut w = start(DH, NB);
                while more(DH, w, NB)
                {
                    incr(DH, Dir::Right, &mut w);
                    let rhs = rhs.copy_elem_2d(h, w);
                    let mut i = start(DV, MA);
                    while more(DV, i, MA)
                    {
                        incr(DV, Dir::Right, &mut i);
                        let mut j = start(DH, NA);
                        while more(DH, j, NA)
                        {
                            incr(DH, Dir::Right, &mut j);
                            let src = &mut dst[h][w][i][j];
                            let dst = (src as *mut <A::Elem as ArrayForm<NA>>::Elem).cast::<U>();
                            unsafe {
                                core::ptr::write(dst, zipper(h, w, i, j, core::ptr::read(src), rhs));
                            }
                            incr(DH, Dir::Left, &mut j);
                        }
                        incr(DV, Dir::Left, &mut i);
                    }
                    incr(DH, Dir::Left, &mut w);
                }
                incr(DV, Dir::Left, &mut h);
            }
    
            m
        }

        // Alternative method that may possibly use less stack memory
        // Only works if elements of A and U have the same layout
        if const {private::fits::<<A::Elem as ArrayForm<NA>>::Elem, U>()}

            // TODO: remove this when same_size_lhs handles destructors properly
            && const {!core::mem::needs_drop::<<A::Elem as ArrayForm<NA>>::Elem>()}
            && const {!core::mem::needs_drop::<U>()}
        {
            return same_size_lhs::<{DV}, {DH}, _, _, _, _, _, _, _, _>(lhs, rhs, zipper)
        }

        let mut fill = |r, c| {
            let h = r % MA;
            let w = c % NA;
            let i = r / MA;
            let j = c / NA;
            zipper(h, w, i, j, lhs.copy_elem_2d(h, w), rhs.copy_elem_2d(i, j))
        };
        match (DH, DV)
        {
            (Dir::Left, Dir::Left) => crate::from_fn(|r| crate::from_fn(|c| fill(r, c))),
            (Dir::Left, Dir::Right) => crate::rfrom_fn(|r| crate::from_fn(|c| fill(r, c))),
            (Dir::Right, Dir::Left) => crate::from_fn(|r| crate::rfrom_fn(|c| fill(r, c))),
            (Dir::Right, Dir::Right) => crate::rfrom_fn(|r| crate::rfrom_fn(|c| fill(r, c))),
        }
    }
}