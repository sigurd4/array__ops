use array_trait::Array;

#[const_trait]
pub trait ArgReduce<T, const N: usize>: Array<Item = T>
{
    /// Performs an argument reduction, finding the final righthand operand for which the comparison yields true.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// fn my_argmax<T>(slice: &[T]) -> Option<usize>
    /// where
    ///     T: PartialOrd
    /// {
    ///     slice.argreduce(PartialOrd::gt)
    /// }
    /// 
    /// fn my_argmin<T>(slice: &[T]) -> Option<usize>
    /// where
    ///     T: PartialOrd
    /// {
    ///     slice.argreduce(PartialOrd::lt)
    /// }
    /// 
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// assert_eq!(my_argmax(&x), x.argmax());
    /// assert_eq!(my_argmin(&x), x.argmin());
    /// ```
    fn argreduce<'a, F>(&'a self, reduction: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
        
    /// Performs an argument reduction on the hashed values, finding the final righthand operand for which the comparison yields true.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// fn hasher(str: &&str) -> i32
    /// {
    ///     i32::from_str_radix(str, 10).unwrap()
    /// }
    /// 
    /// fn my_argmax(slice: &[&str]) -> Option<usize>
    /// {
    ///     slice.argreduce_key(PartialOrd::gt, hasher)
    /// }
    /// 
    /// fn my_argmin(slice: &[&str]) -> Option<usize>
    /// {
    ///     slice.argreduce_key(PartialOrd::lt, hasher)
    /// }
    /// 
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// assert_eq!(my_argmax(&x), x.argmax_by_key(hasher));
    /// assert_eq!(my_argmin(&x), x.argmin_by_key(hasher));
    /// ```
    fn argreduce_key<'a, B, FR, FB>(&'a self, reduction: FR, hasher: FB) -> Option<usize>
    where
        FR: FnMut(&B, &B) -> bool /*+ ~const Destruct*/,
        FB: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        T: 'a;
}

impl<T, const N: usize> ArgReduce<T, N> for [T; N]
{
    fn argreduce<'a, F>(&'a self, mut reduction: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> bool /*+ ~const Destruct*/,
        T: 'a
    {
        if N == 0
        {
            return None;
        }
        let mut i = 1;
        let mut j = 0;
        while i < N
        {
            if reduction(&self[i], &self[j])
            {
                j = i;
            }
            i += 1;
        }
        Some(j)
    }
    fn argreduce_key<'a, B, FR, FB>(&'a self, mut reduction: FR, mut hasher: FB) -> Option<usize>
    where
        FR: FnMut(&B, &B) -> bool /*+ ~const Destruct*/,
        FB: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        T: 'a
    {
        if N == 0
        {
            return None;
        }
        let mut j = 0;
        let mut i = 1;
        let mut key = hasher(&self[j]);
        while i < N
        {
            let next_key = hasher(&self[i]);
            if reduction(&next_key, &key)
            {
                j = i;
                key = next_key;
            }
            i += 1;
        }
        Some(j)
    }
}