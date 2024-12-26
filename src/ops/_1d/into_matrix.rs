use array_trait::Array;

use crate::private::guard::PartialEmptyGuard;

#[const_trait]
pub trait IntoMatrix<T, const N: usize>: Array<Item = T>
{
    fn diagonal_matrix<const H: usize, const W: usize>(self) -> [[T; W]; H]
    where
        T: Default + Copy,
        [(); H - N]:,
        [(); W - N]:;

    fn toeplitz_matrix(&self) -> [[T; N]; N]
    where
        T: Copy;
    fn toeplitz_matrix_ref(&self) -> [[&T; N]; N];

    fn hankel_matrix<const M: usize>(&self, r: &[T; M]) -> [[T; M]; N]
    where
        T: Copy;
    fn hankel_matrix_ref<const M: usize>(&self, r: &[T; M]) -> [[&T; M]; N];
}

impl<T, const N: usize> IntoMatrix<T, N> for [T; N]
{
    fn diagonal_matrix<const H: usize, const W: usize>(self) -> [[T; W]; H]
    where
        T: Default,
        [(); H - N]:,
        [(); W - N]:
    {
        let mut guard = PartialEmptyGuard::new_left(self);
        
        let dst = crate::from_fn(|i| crate::from_fn(|j| if i == j && guard.more()
            {
                guard.pop()
            }
            else
            {
                T::default()
            }
        ));

        guard.done();
    
        dst
    }

    fn toeplitz_matrix(&self) -> [[T; N]; N]
    where
        T: Copy
    {
        crate::from_fn(|i| crate::from_fn(|j| self[if i >= j {i - j} else {j - i}]))
    }
    fn toeplitz_matrix_ref(&self) -> [[&T; N]; N]
    {
        crate::from_fn(|i| crate::from_fn(|j| &self[if i >= j {i - j} else {j - i}]))
    }

    fn hankel_matrix<const M: usize>(&self, r: &[T; M]) -> [[T; M]; N]
    where
        T: Copy
    {
        crate::from_fn(|i| crate::from_fn(|j| if i + j < N
        {
            self[i + j]
        }
        else
        {
            r[i + j + 1 - N]
        }))
    }
    fn hankel_matrix_ref<const M: usize>(&self, r: &[T; M]) -> [[&T; M]; N]
    {
        crate::from_fn(|i| crate::from_fn(|j| if i + j < N
        {
            &self[i + j]
        }
        else
        {
            &r[i + j + 1 - N]
        }))
    }
}