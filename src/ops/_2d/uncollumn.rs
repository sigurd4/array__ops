use array_trait::Array;

#[const_trait]
pub trait ArrayUncollumn<T, const N: usize>: Array<Item = [T; 1]>
{
    fn uncollumn(self) -> [T; N];
    fn uncollumn_ref(&self) -> &[T; N];
    fn uncollumn_mut(&mut self) -> &mut [T; N];
}

impl<T, const N: usize> ArrayUncollumn<T, N> for [[T; 1]; N]
{
    fn uncollumn(self) -> [T; N]
    {
        let uncollumn = unsafe {
            self.as_ptr().cast().read()
        };
        core::mem::forget(self);
        uncollumn
    }

    fn uncollumn_ref(&self) -> &[T; N]
    {
        unsafe {
            self.as_ptr().cast().as_ref_unchecked()
        }
    }

    fn uncollumn_mut(&mut self) -> &mut [T; N]
    {
        unsafe {
            self.as_mut_ptr().cast().as_mut_unchecked()
        }
    }
}