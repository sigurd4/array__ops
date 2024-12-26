use array_trait::Array;

#[const_trait]
pub trait ArrayPermute<T, const N: usize>: Array<T, N>
{
    /// Performs the bit-reverse permutation. Length must be a power of 2.
    /// 
    /// # Example
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.bit_rev_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
    /// ```
    fn bit_rev_permutation(&mut self)
    where
        [(); slice_ops::is_power_of(N, 2) as usize - 1]:;
    fn digit_rev_permutation<const R: usize>(&mut self)
    where
        [(); slice_ops::is_power_of(N, R) as usize - 1]:;

    /// Performs the grey code permutation. Length must be a power of 2.
    /// 
    /// # Example
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.grey_code_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100])
    /// ```
    fn grey_code_permutation(&mut self)
    where
        [(); N.is_power_of_two() as usize - 1]:;
}

impl<T, const N: usize> const ArrayPermute<T, N> for [T; N]
{
    fn bit_rev_permutation(&mut self)
    where
        [(); slice_ops::is_power_of(N, 2) as usize - 1]:
    {
        self.digit_rev_permutation::<2>()
    }
    fn digit_rev_permutation<const R: usize>(&mut self)
    where
        [(); slice_ops::is_power_of(N, R) as usize - 1]:
    {
        if N <= R
        {
            return;
        }
    
        let mut i = 1;
        let mut j = N/R + 1;
        while i < N - 1
        {
            if i < j - 1
            {
                unsafe {
                    core::ptr::swap_nonoverlapping(self.as_mut_ptr().add(i), self.as_mut_ptr().add(j - 1), 1);
                }
            }
            let mut k = N/R;
            while k*(R - 1) < j
            {
                j -= k*(R - 1);
                k /= R;
            }
            j += k;
            i += 1;
        }
    }

    fn grey_code_permutation(&mut self)
    where
        [(); N.is_power_of_two() as usize - 1]:
    {
        let mut i = 0;
        while i < N
        {
            let mut j = i ^ (i >> 1);
            while j < i
            {
                j = j ^ (j >> 1);
            }
            if j != i
            {
                unsafe {
                    core::ptr::swap_nonoverlapping(self.as_mut_ptr().add(i), self.as_mut_ptr().add(j), 1);
                }
            }
            i += 1;
        }
    }
}