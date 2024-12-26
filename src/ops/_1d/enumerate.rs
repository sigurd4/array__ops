use array_trait::Array;

use super::EnumerateMap;

#[const_trait]
pub trait Enumerate<T, const N: usize>: Array<Item = T>
{
    fn enumerate(self) -> [(usize, T); N];
    fn enumerate_ref(&self) -> [(usize, &T); N];
    fn enumerate_mut(&mut self) -> [(usize, &mut T); N];
}

impl<T, const N: usize> Enumerate<T, N> for [T; N]
{
    fn enumerate(self) -> [(usize, T); N]
    {
        self.enumerate_map(|i, x| (i, x))
    }
    fn enumerate_ref(&self) -> [(usize, &T); N]
    {
        self.enumerate_map_ref(|i, x| (i, x))
    }
    fn enumerate_mut(&mut self) -> [(usize, &mut T); N]
    {
        self.enumerate_map_mut(|i, x| (i, x))
    }
}