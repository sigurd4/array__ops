use core::pin::Pin;

use array_trait::Array;

use crate::private;

use super::Split;

#[const_trait]
pub trait ArrayChunks<T, const N: usize>: Array<Item = T>
{
    /// Divides an array into chunks, then yielding the rest in a separate array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::ops::*;
    /// 
    /// let array = ["carrot", "potato", "beet", "tomato", "kiwi", "banana", "cherry", "peach", "strawberry", "nine volt batteries"];
    /// 
    /// let ([root_vegetables, technically_berries, stone_fruits], not_for_human_consumption) = array.chunks::<3>();
    /// 
    /// assert_eq!(root_vegetables, ["carrot", "potato", "beet"]);
    /// assert_eq!(technically_berries, ["tomato", "kiwi", "banana"]);
    /// assert_eq!(stone_fruits, ["cherry", "peach", "strawberry"]);
    /// assert_eq!(not_for_human_consumption, ["nine volt batteries"]);
    /// ```
    fn chunks<const M: usize>(self) -> ([[T; M]; N / M], [T; N % M])
    where
        [(); N % M]:,
        [(); N / M]:;
    /// Divides an array-slice into chunks, then yielding the rest in a separate array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::ops::*;
    /// 
    /// let transistors = ["2N3904", "2N2222A", "BC107", "AC127", "OC7", "NKT275", "2SK30A", "2N5458", "J108", "2N7000", "BS170"];
    /// 
    /// let ([silicon_bjts, germanium_bjts, jfets], mosfets) = transistors.chunks_ref::<3>();
    /// 
    /// assert_eq!(silicon_bjts, &["2N3904", "2N2222A", "BC107"]);
    /// assert_eq!(germanium_bjts, &["AC127", "OC7", "NKT275"]);
    /// assert_eq!(jfets, &["2SK30A", "2N5458", "J108"]);
    /// assert_eq!(mosfets, &["2N7000", "BS170"]);
    /// ```
    fn chunks_ref<const M: usize>(&self) -> (&[[T; M]; N / M], &[T; N % M])
    where
        [(); N % M]:,
        [(); N / M]:;
    /// Divides a mutable array-slice into chunks, then yielding the rest in a separate mutable array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::ops::*;
    /// 
    /// let mut array = [0, 1, 0, 1, 0, 1, 6];
    /// 
    /// let (pairs, last) = array.chunks_mut::<2>();
    /// 
    /// for (i, pair) in pairs.into_iter().enumerate()
    /// {
    ///     for number in pair
    ///     {
    ///         *number += i*2;
    ///     }
    /// }
    /// 
    /// assert_eq!(array, [0, 1, 2, 3, 4, 5, 6]);
    /// ```
    fn chunks_mut<const M: usize>(&mut self) -> (&mut [[T; M]; N / M], &mut [T; N % M])
    where
        [(); N % M]:,
        [(); N / M]:;
    fn chunks_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[[T; M]; N / M]>, Pin<&[T; N % M]>)
    where
        [(); N % M]:,
        [(); N / M]:;
    fn chunks_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [[T; M]; N / M]>, Pin<&mut [T; N % M]>)
    where
        [(); N % M]:,
        [(); N / M]:;
    
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate mutable array-slice.
    fn rchunks<const M: usize>(self) -> ([T; N % M], [[T; M]; N / M])
    where
        [(); N % M]:,
        [(); N / M]:;
    /// Divides an array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn rchunks_ref<const M: usize>(&self) -> (&[T; N % M], &[[T; M]; N / M])
    where
        [(); N % M]:,
        [(); N / M]:;
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn rchunks_mut<const M: usize>(&mut self) -> (&mut [T; N % M], &mut [[T; M]; N / M])
    where
        [(); N % M]:,
        [(); N / M]:;
    fn rchunks_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[T; N % M]>, Pin<&[[T; M]; N / M]>)
    where
        [(); N % M]:,
        [(); N / M]:;
    fn rchunks_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [T; N % M]>, Pin<&mut [[T; M]; N / M]>)
    where
        [(); N % M]:,
        [(); N / M]:;
    
    /// Divides an array into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::ops::*;
    /// 
    /// let array = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    /// 
    /// let [lower_half, upper_half] = array.chunks_exact::<5>();
    /// 
    /// assert_eq!(lower_half, [0.0, 0.1, 0.2, 0.3, 0.4]);
    /// assert_eq!(upper_half, [0.5, 0.6, 0.7, 0.8, 0.9]);
    /// ```
    fn chunks_exact<const M: usize>(self) -> [[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    /// Divides an array-slice into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::ops::*;
    /// 
    /// let array = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    /// 
    /// let [lower_half, upper_half] = array.chunks_exact_ref::<5>();
    /// 
    /// assert_eq!(lower_half, &[0.0, 0.1, 0.2, 0.3, 0.4]);
    /// assert_eq!(upper_half, &[0.5, 0.6, 0.7, 0.8, 0.9]);
    /// ```
    fn chunks_exact_ref<const M: usize>(&self) -> &[[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    /// Divides a mutable array-slice into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    fn chunks_exact_mut<const M: usize>(&mut self) -> &mut [[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    fn chunks_exact_pin_ref<const M: usize>(self: Pin<&Self>) -> Pin<&[[T; M]; N / M]>
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    fn chunks_exact_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Pin<&mut [[T; M]; N / M]>
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
}

impl<T, const N: usize> const ArrayChunks<T, N> for [T; N]
{
    fn chunks<const M: usize>(self) -> ([[T; M]; N / M], [T; N % M])
    {
        unsafe {
            private::split_transmute(self)
        }
    }
    fn chunks_ref<const M: usize>(&self) -> (&[[T; M]; N / M], &[T; N % M])
    {
        let (left, right) = self.rsplit_ptr(N % M);
        let chunks = unsafe {
            left.cast::<[[T; M]; N / M]>().as_ref_unchecked()
        };
        let rest = unsafe {
            right.cast::<[T; N % M]>().as_ref_unchecked()
        };
        (chunks, rest)
    }
    fn chunks_mut<const M: usize>(&mut self) -> (&mut [[T; M]; N / M], &mut [T; N % M])
    {
        let (left, right) = self.rsplit_mut_ptr(N % M);
        let chunks = unsafe {
            left.cast::<[[T; M]; N / M]>().as_mut_unchecked()
        };
        let rest = unsafe {
            right.cast::<[T; N % M]>().as_mut_unchecked()
        };
        (chunks, rest)
    }
    fn chunks_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[[T; M]; N / M]>, Pin<&[T; N % M]>)
    where
        [(); N % M]:,
        [(); N / M]:
    {
        let (chunks, rest) = self.get_ref().chunks_ref();
        unsafe {
            (
                Pin::new_unchecked(chunks),
                Pin::new_unchecked(rest)
            )
        }
    }
    fn chunks_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [[T; M]; N / M]>, Pin<&mut [T; N % M]>)
    where
        [(); N % M]:,
        [(); N / M]:
    {
        unsafe {
            let (chunks, rest) = self.get_unchecked_mut().chunks_mut();
            (
                Pin::new_unchecked(chunks),
                Pin::new_unchecked(rest)
            )
        }
    }

    fn rchunks<const M: usize>(self) -> ([T; N % M], [[T; M]; N / M])
    {
        unsafe {
            private::split_transmute(self)
        }
    }
    fn rchunks_ref<const M: usize>(&self) -> (&[T; N % M], &[[T; M]; N / M])
    {
        let (left, right) = self.split_ptr(N % M);
        let rest = unsafe {
            left.cast::<[T; N % M]>().as_ref_unchecked()
        };
        let chunks = unsafe {
            right.cast::<[[T; M]; N / M]>().as_ref_unchecked()
        };
        (rest, chunks)
    }
    fn rchunks_mut<const M: usize>(&mut self) -> (&mut [T; N % M], &mut [[T; M]; N / M])
    {
        let (left, right) = self.split_mut_ptr(N % M);
        let rest = unsafe {
            left.cast::<[T; N % M]>().as_mut_unchecked()
        };
        let chunks = unsafe {
            right.cast::<[[T; M]; N / M]>().as_mut_unchecked()
        };
        (rest, chunks)
    }
    fn rchunks_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[T; N % M]>, Pin<&[[T; M]; N / M]>)
    where
        [(); N % M]:,
        [(); N / M]:
    {
        let (rest, chunks) = self.get_ref().rchunks_ref();
        unsafe {
            (
                Pin::new_unchecked(rest),
                Pin::new_unchecked(chunks)
            )
        }
    }
    fn rchunks_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [T; N % M]>, Pin<&mut [[T; M]; N / M]>)
    where
        [(); N % M]:,
        [(); N / M]:
    {
        unsafe {
            let (rest, chunks) = self.get_unchecked_mut().rchunks_mut();
            (
                Pin::new_unchecked(rest),
                Pin::new_unchecked(chunks)
            )
        }
    }
    
    fn chunks_exact<const M: usize>(self) -> [[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            private::transmute(self)
        }
    }
    fn chunks_exact_ref<const M: usize>(&self) -> &[[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            self.as_ptr().cast::<[[T; M]; N / M]>().as_ref_unchecked()
        }
    }
    fn chunks_exact_mut<const M: usize>(&mut self) -> &mut [[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            self.as_mut_ptr().cast::<[[T; M]; N / M]>().as_mut_unchecked()
        }
    }
    fn chunks_exact_pin_ref<const M: usize>(self: Pin<&Self>) -> Pin<&[[T; M]; N / M]>
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            Pin::new_unchecked(self.get_ref().chunks_exact_ref())
        }
    }
    fn chunks_exact_pin_mut<const M: usize>(self: Pin<&mut Self>) -> Pin<&mut [[T; M]; N / M]>
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        unsafe {
            Pin::new_unchecked(self.get_unchecked_mut().chunks_exact_mut())
        }
    }
}