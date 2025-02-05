use core::ops::{MulAssign, Sub};

use array_trait::Array;
use slice_ops::AsSlice;

use crate::form::ArrayForm;

// TODO: Add mul_cross_all in MxN matrix
#[const_trait]
pub trait ArrayMulCross<T, const N: usize>: Array + AsSlice<Item = T>
{
    /// Computes the general cross-product of the two arrays (as if vectors, in the mathematical sense).
    /// 
    /// # Example
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use array__ops::ops::*;
    /// 
    /// const U: [f64; 3] = [1.0, 0.0, 0.0];
    /// const V: [f64; 3] = [0.0, 1.0, 0.0];
    /// 
    /// let w = U.mul_cross([&V]);
    /// 
    /// assert_eq!(w, [0.0, 0.0, 1.0]);
    /// ```
    fn mul_cross<Rhs>(&self, rhs: [&Rhs; N - 2]) -> [<T as Sub>::Output; N]
    where
        T: MulAssign<Rhs::Elem> + Sub + Copy,
        Rhs: ArrayForm<N, Elem: Copy>;

    async fn mul_cross_async<Rhs>(&self, rhs: [&Rhs; N - 2]) -> [<T as Sub>::Output; N]
    where
        T: MulAssign<Rhs::Elem> + Sub + Copy,
        Rhs: ArrayForm<N, Elem: Copy>;
}

impl<T, const N: usize> ArrayMulCross<T, N> for [T; N]
{
    fn mul_cross<Rhs>(&self, rhs: [&Rhs; N - 2]) -> [<T as Sub>::Output; N]
    where
        T: MulAssign<Rhs::Elem> + Sub + Copy,
        Rhs: ArrayForm<N, Elem: Copy>
    {
        crate::from_fn(|i| {
            let mut m_p = self[(i + 1) % N];
            let mut m_m = self[(i + (N - 1)) % N];
    
            let mut n = 2;
            while n < N
            {
                m_p *= rhs[n - 2].copy_elem((i + n) % N);
                m_m *= rhs[n - 2].copy_elem((i + (N - n)) % N);
                
                n += 1;
            }
    
            m_p - m_m
        })
    }
    
    async fn mul_cross_async<Rhs>(&self, rhs: [&Rhs; N - 2]) -> [<T as Sub>::Output; N]
    where
        T: MulAssign<Rhs::Elem> + Sub + Copy,
        Rhs: ArrayForm<N, Elem: Copy>
    {
        crate::from_fn_async(async |i| {
            let mut m_p = self[(i + 1) % N];
            let mut m_m = self[(i + (N - 1)) % N];
    
            let mut n = 2;
            while n < N
            {
                m_p *= rhs[n - 2].copy_elem((i + n) % N);
                m_m *= rhs[n - 2].copy_elem((i + (N - n)) % N);
                
                n += 1;
            }
    
            m_p - m_m
        }).await
    }
}