use core::{marker::Destruct, pin::Pin};

use array_trait::Array;

use crate::private::guard::PartialEmptyGuard;

#[const_trait]
pub trait Fold<T, const N: usize>: Array<Item = T>
{
    fn fold<F, O>(self, default: O, fold: F) -> O
    where
        F: FnMut(O, T) -> O + ~const Destruct;
    fn fold_ref<'a, F, O>(&'a self, default: O, fold: F) -> O
    where
        F: FnMut(O, &'a T) -> O + ~const Destruct,
        T: 'a;
    fn fold_mut<'a, F, O>(&'a mut self, default: O, fold: F) -> O
    where
        F: FnMut(O, &'a mut T) -> O + ~const Destruct,
        T: 'a;
    fn fold_pin_ref<'a, F, O>(self: Pin<&'a Self>, default: O, fold: F) -> O
    where
        F: FnMut(O, Pin<&'a T>) -> O + ~const Destruct,
        T: 'a;
    fn fold_pin_mut<'a, F, O>(self: Pin<&'a mut Self>, default: O, fold: F) -> O
    where
        F: FnMut(O, Pin<&'a mut T>) -> O + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> Fold<T, N> for [T; N]
{
    fn fold<F, O>(self, default: O, fold: F) -> O
    where
        F: FnMut(O, T) -> O
    {
        PartialEmptyGuard::new_left(self).fold(default, fold)
    }
    fn fold_ref<'a, F, O>(&'a self, default: O, fold: F) -> O
    where
        F: FnMut(O, &'a T) -> O,
        T: 'a
    {
        PartialEmptyGuard::new_left(self).fold(default, fold)
    }
    fn fold_mut<'a, F, O>(&'a mut self, default: O, fold: F) -> O
    where
        F: FnMut(O, &'a mut T) -> O,
        T: 'a
    {
        PartialEmptyGuard::new_left(self).fold(default, fold)
    }
    fn fold_pin_ref<'a, F, O>(self: Pin<&'a Self>, default: O, fold: F) -> O
    where
        F: FnMut(O, Pin<&'a T>) -> O,
        T: 'a
    {
        PartialEmptyGuard::new_left(self).fold(default, fold)
    }
    fn fold_pin_mut<'a, F, O>(self: Pin<&'a mut Self>, default: O, fold: F) -> O
    where
        F: FnMut(O, Pin<&'a mut T>) -> O,
        T: 'a
    {
        PartialEmptyGuard::new_left(self).fold(default, fold)
    }
}