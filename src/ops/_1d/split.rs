use core::pin::Pin;

use array_trait::Array;

use crate::private;

#[const_trait]
pub trait Split<T, const N: usize>: Array<Item = T>
{
    fn split_len(n: usize) -> (usize, usize);
    fn rsplit_len(n: usize) -> (usize, usize);
        
    fn split_ptr(&self, n: usize) -> (*const T, *const T);
    fn split_mut_ptr(&mut self, n: usize) -> (*mut T, *mut T);

    fn rsplit_ptr(&self, n: usize) -> (*const T, *const T);
    fn rsplit_mut_ptr(&mut self, n: usize) -> (*mut T, *mut T);
    
    /// Splits an array at a chosen index.
    fn split_array2<const M: usize>(self) -> ([T; M], [T; N - M])
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as array-slices.
    fn split_array_ref2<const M: usize>(&self) -> (&[T; M], &[T; N - M])
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as mutable array-slices.
    fn split_array_mut2<const M: usize>(&mut self) -> (&mut [T; M], &mut [T; N - M])
    where
        [(); N - M]:;
    fn split_array_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[T; M]>, Pin<&[T; N - M]>)
    where
        [(); N - M]:;
    fn split_array_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [T; M]>, Pin<&mut [T; N - M]>)
    where
        [(); N - M]:;
    
    /// Splits an array at a chosen index, where the index goes from right to left.
    fn rsplit_array<const M: usize>(self) -> ([T; N - M], [T; M])
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as array-slices, where the index goes from right to left.
    fn rsplit_array_ref<const M: usize>(&self) -> (&[T; N - M], &[T; M])
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as mutable array-slices, where the index goes from right to left.
    fn rsplit_array_mut<const M: usize>(&mut self) -> (&mut [T; N - M], &mut [T; M])
    where
        [(); N - M]:;
    fn rsplit_array_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[T; N - M]>, Pin<&[T; M]>)
    where
        [(); N - M]:;
    fn rsplit_array_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [T; N - M]>, Pin<&mut [T; M]>)
    where
        [(); N - M]:;
}

impl<T, const N: usize> const Split<T, N> for [T; N]
{
    fn split_len(mid: usize) -> (usize, usize)
    {
        slice_ops::split_len(N, mid)
    }
    fn rsplit_len(mid: usize) -> (usize, usize)
    {
        slice_ops::rsplit_len(N, mid)
    }
    
    fn split_ptr(&self, mid: usize) -> (*const T, *const T)
    {
        let ptr = self.as_ptr();
        (ptr, unsafe {ptr.add(slice_ops::split_len(N, mid).0)})
    }
    fn split_mut_ptr(&mut self, mid: usize) -> (*mut T, *mut T)
    {
        let ptr = self.as_mut_ptr();
        (ptr, unsafe {ptr.add(slice_ops::split_len(N, mid).0)})
    }

    fn rsplit_ptr(&self, mid: usize) -> (*const T, *const T)
    {
        let ptr = self.as_ptr();
        (ptr, unsafe {ptr.add(slice_ops::rsplit_len(N, mid).0)})
    }
    fn rsplit_mut_ptr(&mut self, mid: usize) -> (*mut T, *mut T)
    {
        let ptr = self.as_mut_ptr();
        (ptr, unsafe {ptr.add(slice_ops::rsplit_len(N, mid).0)})
    }
    
    fn split_array2<const M: usize>(self) -> ([T; M], [T; N - M])
    where
        [(); N - M]:
    {
        unsafe {
            private::split_transmute(self)
        }
    }
    fn split_array_ref2<const M: usize>(&self) -> (&[T; M], &[T; N - M])
    where
        [(); N - M]:
    {
        let (ptr_left, ptr_right) = self.split_ptr(M);
        unsafe {(
            ptr_left.cast::<[T; M]>().as_ref_unchecked(),
            ptr_right.cast::<[T; N - M]>().as_ref_unchecked()
        )}
    }
    fn split_array_mut2<const M: usize>(&mut self) -> (&mut [T; M], &mut [T; N - M])
    where
        [(); N - M]:
    {
        let (ptr_left, ptr_right) = self.split_mut_ptr(M);
        unsafe {(
            ptr_left.cast::<[T; M]>().as_mut_unchecked(),
            ptr_right.cast::<[T; N - M]>().as_mut_unchecked()
        )}
    }
    fn split_array_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[T; M]>, Pin<&[T; N - M]>)
    where
        [(); N - M]:
    {
        let (left, right) = self.get_ref().split_array_ref2();
        unsafe {(
            Pin::new_unchecked(left),
            Pin::new_unchecked(right)
        )}
    }
    fn split_array_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [T; M]>, Pin<&mut [T; N - M]>)
    where
        [(); N - M]:
    {
        let (left, right) = unsafe {
            self.get_unchecked_mut().split_array_mut2()
        };
        unsafe {(
            Pin::new_unchecked(left),
            Pin::new_unchecked(right)
        )}
    }
    
    fn rsplit_array<const M: usize>(self) -> ([T; N - M], [T; M])
    where
        [(); N - M]:
    {
        unsafe {
            private::split_transmute(self)
        }
    }
    fn rsplit_array_ref<const M: usize>(&self) -> (&[T; N - M], &[T; M])
    where
        [(); N - M]:
    {
        let (ptr_left, ptr_right) = self.rsplit_ptr(M);
        unsafe {(
            ptr_left.cast::<[T; N - M]>().as_ref_unchecked(),
            ptr_right.cast::<[T; M]>().as_ref_unchecked()
        )}
    }
    fn rsplit_array_mut<const M: usize>(&mut self) -> (&mut [T; N - M], &mut [T; M])
    where
        [(); N - M]:
    {
        let (ptr_left, ptr_right) = self.rsplit_mut_ptr(M);
        unsafe {(
            ptr_left.cast::<[T; N - M]>().as_mut_unchecked(),
            ptr_right.cast::<[T; M]>().as_mut_unchecked()
        )}
    }
    fn rsplit_array_pin_ref<const M: usize>(self: Pin<&Self>) -> (Pin<&[T; N - M]>, Pin<&[T; M]>)
    where
        [(); N - M]:
    {
        let (left, right) = Split::rsplit_array_ref(self.get_ref());
        unsafe {(
            Pin::new_unchecked(left),
            Pin::new_unchecked(right)
        )}
    }
    fn rsplit_array_pin_mut<const M: usize>(self: Pin<&mut Self>) -> (Pin<&mut [T; N - M]>, Pin<&mut [T; M]>)
    where
        [(); N - M]:
    {
        let (left, right) = unsafe {
            Split::rsplit_array_mut(self.get_unchecked_mut())
        };
        unsafe {(
            Pin::new_unchecked(left),
            Pin::new_unchecked(right)
        )}
    }
}