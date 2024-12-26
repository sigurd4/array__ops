use array_trait::Array;

#[const_trait]
pub trait ArrayFlatten<T, const M: usize, const N: usize>: Array<Item = [T; N]>
{
    fn flatten(self) -> [T; M*N];
    fn flatten_ref(&self) -> &[T; M*N];
    fn flatten_mut(&mut self) -> &mut [T; M*N];
}

impl<T, const M: usize, const N: usize> const ArrayFlatten<T, M, N> for [[T; N]; M]
{
    fn flatten(self) -> [T; M*N]
    {
        let flattened = unsafe {
            self.as_ptr().cast().read()
        };
        core::mem::forget(self);
        flattened
    }
    fn flatten_ref(&self) -> &[T; M*N]
    {
        unsafe {
            self.as_ptr().cast().as_ref_unchecked()
        }
    }
    fn flatten_mut(&mut self) -> &mut [T; M*N]
    {
        unsafe {
            self.as_mut_ptr().cast().as_mut_unchecked()
        }
    }
}