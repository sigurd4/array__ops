use array_trait::Array;

use crate::private;

#[const_trait]
pub trait Reformulate<T, const N: usize>: Array<Item = T>
{
    fn reformulate<const M: usize>(self) -> [T; M]
    where
        [(); M - N]:,
        [(); N - M]:;
    
    fn reformulate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); M - N]:,
        [(); N - M]:;
        
    fn reformulate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); M - N]:,
        [(); N - M]:;
        
    fn try_reformulate<const M: usize>(self) -> Result<[T; M], [T; N]>;
    
    fn try_reformulate_ref<const M: usize>(&self) -> Option<&[T; M]>;
        
    fn try_reformulate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>;
}

impl<T, const N: usize> Reformulate<T, N> for [T; N]
{
    fn reformulate<const M: usize>(self) -> [T; M]
    where
        [(); M - N]:,
        [(); N - M]:
    {
        unsafe {
            self.try_reformulate().unwrap_unchecked()
        }
    }
    
    fn reformulate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); M - N]:,
        [(); N - M]:
    {
        unsafe {
            self.try_reformulate_ref().unwrap_unchecked()
        }
    }
        
    fn reformulate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); M - N]:,
        [(); N - M]:
    {
        unsafe {
            self.try_reformulate_mut().unwrap_unchecked()
        }
    }
    
    fn try_reformulate<const M: usize>(self) -> Result<[T; M], Self>
    {
        if N != M
        {
            return Err(self)
        }
        unsafe {
            Ok(private::transmute_unchecked_size(self))
        }
    }
    
    fn try_reformulate_ref<const M: usize>(&self) -> Option<&[T; M]>
    {
        if N != M
        {
            return None
        }
        unsafe {
            Some(self.as_ptr().cast::<[T; M]>().as_ref_unchecked())
        }
    }
        
    fn try_reformulate_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>
    {
        if N != M
        {
            return None
        }
        unsafe {
            Some(self.as_mut_ptr().cast::<[T; M]>().as_mut_unchecked())
        }
    }
}