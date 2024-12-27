use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;

use super::EnumerateFlatmap;

#[const_trait]
pub trait Flatmap<T, const N: usize>: Array<Item = T>
{
    /// Maps all values of an array with a given function.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// const A: [u8; 4] = [1, 2, 3, 4];
    /// let b = A.map(|b| -(b as i8));
    /// 
    /// assert_eq!(b, [-1, -2, -3, -4]);
    /// ```
    fn flatmap<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(T,), Output = [U; M]> + ~const Destruct,
        [(); N*M]:;
    fn flatmap_ref<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(&'a T,), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn flatmap_mut<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(&'a mut T,), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn flatmap_pin_ref<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(Pin<&'a T>,), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn flatmap_pin_mut<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(Pin<&'a mut T>,), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;

    fn rflatmap<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(T,), Output = [U; M]> + ~const Destruct,
        [(); N*M]:;
    fn rflatmap_ref<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(&'a T,), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn rflatmap_mut<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(&'a mut T,), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn rflatmap_pin_ref<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(Pin<&'a T>,), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn rflatmap_pin_mut<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(Pin<&'a mut T>,), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;

    async fn flatmap_async<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(T) -> [U; M] + ~const Destruct,
        [(); N*M]:;
    async fn flatmap_ref_async<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(&'a T) -> [U; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn flatmap_mut_async<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(&'a mut T) -> [U; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn flatmap_pin_ref_async<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(Pin<&'a T>) -> [U; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn flatmap_pin_mut_async<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(Pin<&'a mut T>) -> [U; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
        
    fn try_flatmap<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(T) -> Result<[U; M], E> + ~const Destruct;
    fn try_flatmap_ref<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_flatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a mut T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_flatmap_pin_ref<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(Pin<&'a T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_flatmap_pin_mut<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(Pin<&'a mut T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;

    fn try_rflatmap<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(T) -> Result<[U; M], E> + ~const Destruct;
    fn try_rflatmap_ref<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_rflatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a mut T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_rflatmap_pin_ref<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(Pin<&'a T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_rflatmap_pin_mut<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(Pin<&'a mut T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
        
    async fn try_flatmap_async<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(T) -> Result<[U; M], E> + ~const Destruct,
        [(); N*M]:;
    async fn try_flatmap_ref_async<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(&'a T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn try_flatmap_mut_async<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(&'a mut T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn try_flatmap_pin_ref_async<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(Pin<&'a T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn try_flatmap_pin_mut_async<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(Pin<&'a mut T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
}

impl<T, const N: usize> Flatmap<T, N> for [T; N]
{
    fn flatmap<Map, U, const M: usize>(self, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(T,), Output = [U; M]> + Destruct,
        [(); N*M]:
    {
        self.enumerate_flatmap(|_, x| mapper(x))
    }
    fn flatmap_ref<'a, Map, U, const M: usize>(&'a self, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(&'a T,), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_flatmap_ref(|_, x| mapper(x))
    }
    fn flatmap_mut<'a, Map, U, const M: usize>(&'a mut self, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(&'a mut T,), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_flatmap_mut(|_, x| mapper(x))
    }
    fn flatmap_pin_ref<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(Pin<&'a T>,), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_flatmap_pin_ref(|_, x| mapper(x))
    }
    fn flatmap_pin_mut<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(Pin<&'a mut T>,), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_flatmap_pin_mut(|_, x| mapper(x))
    }

    fn rflatmap<Map, U, const M: usize>(self, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(T,), Output = [U; M]> + Destruct,
        [(); N*M]:
    {
        self.enumerate_rflatmap(|_, x| mapper(x))
    }
    fn rflatmap_ref<'a, Map, U, const M: usize>(&'a self, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(&'a T,), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_rflatmap_ref(|_, x| mapper(x))
    }
    fn rflatmap_mut<'a, Map, U, const M: usize>(&'a mut self, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(&'a mut T,), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_rflatmap_mut(|_, x| mapper(x))
    }
    fn rflatmap_pin_ref<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(Pin<&'a T>,), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_rflatmap_pin_ref(|_, x| mapper(x))
    }
    fn rflatmap_pin_mut<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mut mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(Pin<&'a mut T>,), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_rflatmap_pin_mut(|_, x| mapper(x))
    }
    
    async fn flatmap_async<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(T) -> [U; M],
        [(); N*M]:
    {
        self.enumerate_flatmap_async(|_, x| mapper(x)).await
    }
    async fn flatmap_ref_async<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(&'a T) -> [U; M],
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_flatmap_ref_async(|_, x| mapper(x)).await
    }
    async fn flatmap_mut_async<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(&'a mut T) -> [U; M],
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_flatmap_mut_async(|_, x| mapper(x)).await
    }
    async fn flatmap_pin_ref_async<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(Pin<&'a T>) -> [U; M],
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_flatmap_pin_ref_async(|_, x| mapper(x)).await
    }
    async fn flatmap_pin_mut_async<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(Pin<&'a mut T>) -> [U; M],
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_flatmap_pin_mut_async(|_, x| mapper(x)).await
    }
    
    fn try_flatmap<Map, U, E, const M: usize>(self, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(T) -> Result<[U; M], E>
    {
        self.try_enumerate_flatmap(|_, x| mapper(x))
    }
    fn try_flatmap_ref<'a, Map, U, E, const M: usize>(&'a self, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a T) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_flatmap_ref(|_, x| mapper(x))
    }
    fn try_flatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a mut T) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_flatmap_mut(|_, x| mapper(x))
    }
    fn try_flatmap_pin_ref<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(Pin<&'a T>) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_flatmap_pin_ref(|_, x| mapper(x))
    }
    fn try_flatmap_pin_mut<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(Pin<&'a mut T>) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_flatmap_pin_mut(|_, x| mapper(x))
    }

    fn try_rflatmap<Map, U, E, const M: usize>(self, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(T) -> Result<[U; M], E>
    {
        self.try_enumerate_rflatmap(|_, x| mapper(x))
    }
    fn try_rflatmap_ref<'a, Map, U, E, const M: usize>(&'a self, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a T) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_rflatmap_ref(|_, x| mapper(x))
    }
    fn try_rflatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a mut T) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_rflatmap_mut(|_, x| mapper(x))
    }
    fn try_rflatmap_pin_ref<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(Pin<&'a T>) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_rflatmap_pin_ref(|_, x| mapper(x))
    }
    fn try_rflatmap_pin_mut<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mut mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(Pin<&'a mut T>) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_rflatmap_pin_mut(|_, x| mapper(x))
    }
    
    async fn try_flatmap_async<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(T) -> Result<[U; M], E>,
        [(); N*M]:
    {
        self.try_enumerate_flatmap_async(|_, x| mapper(x)).await
    }
    async fn try_flatmap_ref_async<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(&'a T) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        self.try_enumerate_flatmap_ref_async(|_, x| mapper(x)).await
    }
    async fn try_flatmap_mut_async<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(&'a mut T) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        self.try_enumerate_flatmap_mut_async(|_, x| mapper(x)).await
    }
    async fn try_flatmap_pin_ref_async<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(Pin<&'a T>) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        self.try_enumerate_flatmap_pin_ref_async(|_, x| mapper(x)).await
    }
    async fn try_flatmap_pin_mut_async<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(Pin<&'a mut T>) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        self.try_enumerate_flatmap_pin_mut_async(|_, x| mapper(x)).await
    }
}