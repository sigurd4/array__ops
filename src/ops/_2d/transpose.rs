use core::mem::MaybeUninit;

use array_trait::Array;

use crate::private;

use super::ArrayTransposeAssign;

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

impl<T, const M: usize, const N: usize> const ArrayTranspose<T, M, N> for [[T; N]; M]
{
    fn transpose(self) -> [[T; M]; N]
    {
        // Transposes in-place
        // Even though the matrices have different dimensions, they have equal size, which makes this trick possible.

        let mut transposed = unsafe {
            private::transmute(self)
        };

        if N == M
        {
            let square = unsafe {
                (&mut transposed as *mut [[T; M]; N]).cast::<[[T; N]; N]>().as_mut_unchecked()
            };
            square.transpose_assign();
            return transposed
        }

        let ptr = transposed[0].as_mut_ptr();

        let m = unsafe {
            core::slice::from_raw_parts_mut(ptr, M * N)
        };
        let mut visited = [[false; N]; M];
        let visited = unsafe {
            core::slice::from_raw_parts_mut(visited[0].as_mut_ptr(), M * N)
        };
        let mut c = 0;
        while c < M * N
        {
            if !visited[c]
            {
                let mut a = c;
                loop
                {
                    a = if a == N * M - 1
                    {
                        N * M - 1
                    }
                    else
                    {
                        (M * a) % (N * M - 1)
                    };
                    visited[a] = true;
                    if a == c
                    {
                        break
                    }
                    unsafe {
                        m.swap_unchecked(a, c);
                    }
                }
            }
            c += 1
        }

        transposed
    }
}

#[cfg(test)]
mod test
{
    use super::ArrayTranspose;

    #[test]
    fn transpose()
    {
        let a = [
            [1, 2, 3],
            [4, 5, 6]
        ];
        let a_t = a.transpose();

        println!("{:?}", a_t);

        assert_eq!(
            a_t,
            [
                [1, 4],
                [2, 5],
                [3, 6]
            ]
        );
    }
}