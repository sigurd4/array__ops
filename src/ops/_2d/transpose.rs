use array_trait::Array;

#[const_trait]
pub trait ArrayTranspose<T, const M: usize, const N: usize>: Array<Item = [T; N]>
{
    /// Transposes a two-dimensional array (as if it were a matrix)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let matrix: [[u8; 5]; 3] = [
    ///     [1,   2,  3,  4,  5],
    ///     [6,   7,  8,  9, 10],
    ///     [11, 12, 13, 14, 15]
    /// ];
    /// 
    /// assert_eq!(matrix.transpose(), [
    ///     [1,  6, 11],
    ///     [2,  7, 12],
    ///     [3,  8, 13],
    ///     [4,  9, 14],
    ///     [5, 10, 15]
    /// ]);
    /// ```
    fn transpose(self) -> [[T; M]; N];
}

impl<T, const M: usize, const N: usize> ArrayTranspose<T, M, N> for [[T; N]; M]
{
    fn transpose(self) -> [[T; M]; N]
    {
        let transposed = crate::from_fn(|i| crate::from_fn(|j| unsafe {
            core::ptr::read(&self[j][i])
        }));
        core::mem::forget(self);
        transposed
    }
}