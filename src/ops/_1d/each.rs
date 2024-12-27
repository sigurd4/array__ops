use core::pin::Pin;

use array_trait::Array;

use super::Map;

#[const_trait]
pub trait Each<T, const N: usize>: Array<Item = T>
{
    fn each_ref(&self) -> [&T; N];
    fn each_mut(&mut self) -> [&mut T; N];
    fn each_pin_ref(self: Pin<&Self>) -> [Pin<&T>; N];
    fn each_pin_mut(self: Pin<&mut Self>) -> [Pin<&mut T>; N];
}

impl<T, const N: usize> Each<T, N> for [T; N]
{
    fn each_ref(&self) -> [&T; N]
    {
        self.map_ref(|x| x)
    }
    fn each_mut(&mut self) -> [&mut T; N]
    {
        self.map_mut(|x| x)
    }
    fn each_pin_ref(self: Pin<&Self>) -> [Pin<&T>; N]
    {
        self.map_pin_ref(|x| x)
    }
    fn each_pin_mut(self: Pin<&mut Self>) -> [Pin<&mut T>; N]
    {
        self.map_pin_mut(|x| x)
    }
}