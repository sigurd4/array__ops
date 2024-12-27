use array_trait::Array;

#[const_trait]
pub trait ArrayTransposeAssign<T, const N: usize>: Array<Item = [T; N]>
{
    /// Transposes a square matrix in-place.
    /// 
    /// # Examples
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// let mut a = [
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ];
    /// 
    /// a.transpose_assign();
    /// 
    /// assert_eq!(a, [
    ///     [1, 4, 7],
    ///     [2, 5, 8],
    ///     [3, 6, 9]
    /// ]);
    /// ```
    fn transpose_assign(&mut self);
}

impl<T, const N: usize> const ArrayTransposeAssign<T, N> for [[T; N]; N]
{
    fn transpose_assign(&mut self)
    {
        let mut r = 0;
        while r < N
        {
            let row = unsafe {
                (&mut self[r] as *mut [T; N]).as_mut_unchecked()
            };
            let mut c = r + 1;
            while c < N
            {
                core::mem::swap(
                    &mut row[c],
                    &mut self[c][r]
                );
                c += 1;
            }
            r += 1;
        }
    }
}