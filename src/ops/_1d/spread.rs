use array_trait::Array;
use slice_ops::Padded;

use crate::{private, MutForm};

#[const_trait]
pub trait ArraySpread<T, const N: usize>: Array<Item = T>
{
    /// Distributes items of an array equally across a given width, then provides the rest as a separate array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = ["ping 1", "pong 1", "ping 2", "pong 2", "ping 3", "pong 3", "uhh..."];
    /// 
    /// let ([ping, pong], rest) = array.spread::<2>();
    /// 
    /// assert_eq!(ping, ["ping 1", "ping 2", "ping 3"]);
    /// assert_eq!(pong, ["pong 1", "pong 2", "pong 3"]);
    /// assert_eq!(rest, ["uhh..."]);
    /// ```
    fn spread<const M: usize>(self) -> ([[T; N / M]; M], [T; N % M])
    where
        [(); M - 1]:,
        [(); N / M]:,
        [(); N % M]:;

    /// Distributes items of an array-slice equally across a given width, then provides the rest as a separate array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    fn spread_ref<const M: usize>(&self) -> ([&[Padded<T, M>; N / M]; M], &[T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:;
    
    /// Distributes items of a mutable array-slice equally across a given width, then provides the rest as a separate mutable array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let mut array = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20"];
    /// 
    /// let (threes, _) = array.spread_mut::<3>();
    /// 
    /// for fizz in threes.into_iter().last().unwrap()
    /// {
    ///     **fizz = "fizz";
    /// }
    /// 
    /// let (fives, _) = array.spread_mut::<5>();
    /// 
    /// for buzz in fives.into_iter().last().unwrap()
    /// {
    ///     **buzz = "buzz";
    /// }
    /// 
    /// let (fifteens, _) = array.spread_mut::<15>();
    /// 
    /// for fizzbuzz in fifteens.into_iter().last().unwrap()
    /// {
    ///     **fizzbuzz = "fizzbuzz";
    /// }
    /// 
    /// assert_eq!(array, ["1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14", "fizzbuzz", "16", "17", "fizz", "19", "buzz"]);
    /// 
    /// ```
    fn spread_mut<const M: usize>(&mut self) -> ([&mut [Padded<T, M>; N / M]; M], &mut [T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:;
    
    /// Distributes items of an array equally across a given width, then provides the leftmost rest as a separate array.
    fn rspread<const M: usize>(self) -> ([T; N % M], [[T; N / M]; M])
    where
        [(); M - 1]:,
        [(); N / M]:,
        [(); N % M]:,
        T: Copy;

    /// Distributes items of an array-slice equally across a given width, then provides the leftmost rest as a separate array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    /// 
    /// let (zero, [odd, even]) = array.rspread_ref::<2>();
    /// 
    /// assert_eq!(*zero, [0]);
    /// assert_eq!(odd.each_ref().map(|padding| **padding), [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    /// assert_eq!(even.each_ref().map(|padding| **padding), [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    /// ```
    fn rspread_ref<const M: usize>(&self) -> (&[T; N % M], [&[Padded<T, M>; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:;
    /// Distributes items of a mutable array-slice equally across a given width, then provides the leftmost rest as a separate mutable array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array__ops::*;
    /// 
    /// let mut array = ["the", "beat", "goes", "1", "2", "3", "4", "5", "6", "7", "8"];
    /// 
    /// let (start, [boots, n, cats, and]) = array.rspread_mut::<4>();
    /// 
    /// for boots in boots
    /// {
    ///     **boots = "boots";
    /// }
    /// for n in n
    /// {
    ///     **n = "n";
    /// }
    /// for cats in cats
    /// {
    ///     **cats = "cats";
    /// }
    /// for and in and
    /// {
    ///     **and = "and";
    /// }
    /// 
    /// assert_eq!(array, ["the", "beat", "goes", "boots", "n", "cats", "and", "boots", "n", "cats", "and"]);
    /// ```
    fn rspread_mut<const M: usize>(&mut self) -> (&mut [T; N % M], [&mut [Padded<T, M>; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:;
    
    /// Distributes items of an array equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = *b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
    /// 
    /// let [lower_case, upper_case] = array.spread_exact::<2>();
    /// 
    /// assert_eq!(lower_case, *b"abcdefghijklmnopqrstuvwxyz");
    /// assert_eq!(upper_case, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// ```
    fn spread_exact<const M: usize>(self) -> [[T; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:,
        [(); N / M]:;
    
    /// Distributes items of an array-slice equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array__ops::*;
    /// 
    /// let statement = ["s", "he", "be", "lie", "ve", "d"];
    /// 
    /// let [interpretation2, interpretation1] = statement.spread_exact_ref::<2>();
    /// 
    /// assert_eq!(interpretation1, &["he", "lie", "d"]);
    /// assert_eq!(interpretation2, &["s", "be", "ve"]);
    /// ```
    fn spread_exact_ref<const M: usize>(&self) -> [&[Padded<T, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:;

    /// Distributes items of a mutable array-slice equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array__ops::*;
    /// 
    /// let mut array = *b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
    /// 
    /// let [lower_case, upper_case] = array.spread_exact_mut::<2>();
    /// 
    /// assert_eq!(lower_case.each_ref().map(|padding| padding.borrow()), b"abcdefghijklmnopqrstuvwxyz".each_ref());
    /// assert_eq!(upper_case.each_ref().map(|padding| padding.borrow()), b"ABCDEFGHIJKLMNOPQRSTUVWXYZ".each_ref());
    /// 
    /// for c in upper_case
    /// {
    ///     **c = b'_';
    /// }
    /// 
    /// assert_eq!(array, *b"a_b_c_d_e_f_g_h_i_j_k_l_m_n_o_p_q_r_s_t_u_v_w_x_y_z_")
    /// ```
    fn spread_exact_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:;
}

impl<T, const N: usize> const ArraySpread<T, N> for [T; N]
{
    fn spread<const M: usize>(self) -> ([[T; N / M]; M], [T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:,
        [(); N / M]:
    {
        let split = self.chunk();
    
        let spread_t = unsafe {core::ptr::read(&split.0 as *const [[T; _]; _])};
        let rest = unsafe {core::ptr::read(&split.1 as *const [T; _])};
        core::mem::forget(split);
    
        (spread_t.transpose(), rest)
    }
    fn spread_ref<const M: usize>(&self) -> ([&[Padded<T, M>; N / M]; M], &[T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:
    {
        let (left, right) = self.rsplit_ptr(N % M);
    
        unsafe {(
            crate::from_fn(|i| &*left.add(i).cast()),
            &*right.cast()
        )}
    }
    fn spread_mut<const M: usize>(&mut self) -> ([&mut [Padded<T, M>; N / M]; M], &mut [T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:
    {
        let (left, right) = self.rsplit_mut_ptr(N % M);
    
        unsafe {(
            crate::from_fn(|i| &mut *left.add(i).cast()),
            &mut *right.cast()
        )}
    }
    
    fn rspread<const M: usize>(self) -> ([T; N % M], [[T; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:,
        [(); N / M]:
    {
        let split = self.rchunks();
        
        let start = unsafe {core::ptr::read(&split.0 as *const [T; _])};
        let spread_t = unsafe {core::ptr::read(&split.1 as *const [[T; _]; _])};
        core::mem::forget(split);
    
        (start, spread_t.transpose())
    }
    fn rspread_ref<const M: usize>(&self) -> (&[T; N % M], [&[Padded<T, M>; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:
    {
        let (left, right) = self.split_ptr(N % M);
    
        unsafe {(
            &*left.cast(),
            crate::from_fn(|i| &*right.add(i).cast())
        )}
    }
    fn rspread_mut<const M: usize>(&mut self) -> (&mut [T; N % M], [&mut [Padded<T, M>; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:
    {
        let (left, right) = self.split_mut_ptr(N % M);
    
        unsafe {(
            &mut *left.cast(),
            crate::from_fn(|i| &mut *right.add(i).cast())
        )}
    }
    fn spread_exact<const M: usize>(self) -> [[T; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        let spread_t: [[T; M]; N / M] = unsafe {
            private::transmute_unchecked_size(self)
        };
        spread_t.transpose()
    }
    fn spread_exact_ref<const M: usize>(&self) -> [&[Padded<T, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:
    {
        let ptr = self as *const T;
        
        crate::from_fn(|i| unsafe {&*ptr.add(i).cast()})
    }
    fn spread_exact_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:
    {
        let ptr = self as *mut T;
        
        crate::from_fn(|i| unsafe {&mut *ptr.add(i).cast()})
    }
}