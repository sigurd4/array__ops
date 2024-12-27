use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use crate::ops::ArrayFlatten;

use super::{Enumerate, EnumerateMap, Flatmap};

#[const_trait]
pub trait EnumerateFlatmap<T, const N: usize>: Enumerate<T, N> + Flatmap<T, N>
{
    fn enumerate_flatmap<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, T), Output = [U; M]> + ~const Destruct,
        [(); N*M]:;
    fn enumerate_flatmap_ref<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, &'a T), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn enumerate_flatmap_mut<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, &'a mut T), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn enumerate_flatmap_pin_ref<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, Pin<&'a T>), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn enumerate_flatmap_pin_mut<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, Pin<&'a mut T>), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;

    fn enumerate_rflatmap<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, T), Output = [U; M]> + ~const Destruct,
        [(); N*M]:;
    fn enumerate_rflatmap_ref<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, &'a T), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn enumerate_rflatmap_mut<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, &'a mut T), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn enumerate_rflatmap_pin_ref<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, Pin<&'a T>), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn enumerate_rflatmap_pin_mut<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, Pin<&'a mut T>), Output = [U; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;

    async fn enumerate_flatmap_async<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, T) -> [U; M] + ~const Destruct,
        [(); N*M]:;
    async fn enumerate_flatmap_ref_async<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, &'a T) -> [U; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn enumerate_flatmap_mut_async<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, &'a mut T) -> [U; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn enumerate_flatmap_pin_ref_async<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, Pin<&'a T>) -> [U; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn enumerate_flatmap_pin_mut_async<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, Pin<&'a mut T>) -> [U; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
        
    fn try_enumerate_flatmap<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, T) -> Result<[U; M], E> + ~const Destruct;
    fn try_enumerate_flatmap_ref<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, &'a T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_flatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_flatmap_pin_ref<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, Pin<&'a T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_flatmap_pin_mut<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, Pin<&'a mut T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;

    fn try_enumerate_rflatmap<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, T) -> Result<[U; M], E> + ~const Destruct;
    fn try_enumerate_rflatmap_ref<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, &'a T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rflatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rflatmap_pin_ref<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, Pin<&'a T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rflatmap_pin_mut<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, Pin<&'a mut T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
        
    async fn try_enumerate_flatmap_async<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, T) -> Result<[U; M], E> + ~const Destruct,
        [(); N*M]:;
    async fn try_enumerate_flatmap_ref_async<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, &'a T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn try_enumerate_flatmap_mut_async<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, &'a mut T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn try_enumerate_flatmap_pin_ref_async<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, Pin<&'a T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn try_enumerate_flatmap_pin_mut_async<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, Pin<&'a mut T>) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
}

impl<T, const N: usize> EnumerateFlatmap<T, N> for [T; N]
{
    fn enumerate_flatmap<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, T), Output = [U; M]>,
        [(); N*M]:
    {
        self.enumerate_map(mapper).flatten()
    }
    fn enumerate_flatmap_ref<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, &'a T), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_map_ref(mapper).flatten()
    }
    fn enumerate_flatmap_mut<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, &'a mut T), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_map_mut(mapper).flatten()
    }
    fn enumerate_flatmap_pin_ref<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, Pin<&'a T>), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_map_pin_ref(mapper).flatten()
    }
    fn enumerate_flatmap_pin_mut<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, Pin<&'a mut T>), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_map_pin_mut(mapper).flatten()
    }

    fn enumerate_rflatmap<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, T), Output = [U; M]>,
        [(); N*M]:
    {
        self.enumerate_rmap(mapper).flatten()
    }
    fn enumerate_rflatmap_ref<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, &'a T), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_rmap_ref(mapper).flatten()
    }
    fn enumerate_rflatmap_mut<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, &'a mut T), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_rmap_mut(mapper).flatten()
    }
    fn enumerate_rflatmap_pin_ref<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, Pin<&'a T>), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_rmap_pin_ref(mapper).flatten()
    }
    fn enumerate_rflatmap_pin_mut<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: FnMut<(usize, Pin<&'a mut T>), Output = [U; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_rmap_pin_mut(mapper).flatten()
    }
    
    async fn enumerate_flatmap_async<Map, U, const M: usize>(self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, T) -> [U; M],
        [(); N*M]:
    {
        self.enumerate_map_async(mapper).await.flatten()
    }
    async fn enumerate_flatmap_ref_async<'a, Map, U, const M: usize>(&'a self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, &'a T) -> [U; M],
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_map_ref_async(mapper).await.flatten()
    }
    async fn enumerate_flatmap_mut_async<'a, Map, U, const M: usize>(&'a mut self, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, &'a mut T) -> [U; M],
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_map_mut_async(mapper).await.flatten()
    }
    async fn enumerate_flatmap_pin_ref_async<'a, Map, U, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, Pin<&'a T>) -> [U; M],
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_map_pin_ref_async(mapper).await.flatten()
    }
    async fn enumerate_flatmap_pin_mut_async<'a, Map, U, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> [U; N*M]
    where
        Map: AsyncFn(usize, Pin<&'a mut T>) -> [U; M],
        T: 'a,
        [(); N*M]:
    {
        self.enumerate_map_pin_mut_async(mapper).await.flatten()
    }
    
    fn try_enumerate_flatmap<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, T) -> Result<[U; M], E>
    {
        self.try_enumerate_map(mapper).map(ArrayFlatten::flatten)
    }
    fn try_enumerate_flatmap_ref<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, &'a T) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_map_ref(mapper).map(ArrayFlatten::flatten)
    }
    fn try_enumerate_flatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_map_mut(mapper).map(ArrayFlatten::flatten)
    }
    fn try_enumerate_flatmap_pin_ref<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, Pin<&'a T>) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_map_pin_ref(mapper).map(ArrayFlatten::flatten)
    }
    fn try_enumerate_flatmap_pin_mut<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, Pin<&'a mut T>) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_map_pin_mut(mapper).map(ArrayFlatten::flatten)
    }

    fn try_enumerate_rflatmap<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, T) -> Result<[U; M], E>
    {
        self.try_enumerate_rmap(mapper).map(ArrayFlatten::flatten)
    }
    fn try_enumerate_rflatmap_ref<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, &'a T) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_rmap_ref(mapper).map(ArrayFlatten::flatten)
    }
    fn try_enumerate_rflatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_rmap_mut(mapper).map(ArrayFlatten::flatten)
    }
    fn try_enumerate_rflatmap_pin_ref<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, Pin<&'a T>) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_rmap_pin_ref(mapper).map(ArrayFlatten::flatten)
    }
    fn try_enumerate_rflatmap_pin_mut<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(usize, Pin<&'a mut T>) -> Result<[U; M], E>,
        T: 'a
    {
        self.try_enumerate_rmap_pin_mut(mapper).map(ArrayFlatten::flatten)
    }
    
    async fn try_enumerate_flatmap_async<Map, U, E, const M: usize>(self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, T) -> Result<[U; M], E>,
        [(); N*M]:
    {
        self.try_enumerate_map_async(mapper).await.map(ArrayFlatten::flatten)
    }
    async fn try_enumerate_flatmap_ref_async<'a, Map, U, E, const M: usize>(&'a self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, &'a T) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        self.try_enumerate_map_ref_async(mapper).await.map(ArrayFlatten::flatten)
    }
    async fn try_enumerate_flatmap_mut_async<'a, Map, U, E, const M: usize>(&'a mut self, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, &'a mut T) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        self.try_enumerate_map_mut_async(mapper).await.map(ArrayFlatten::flatten)
    }
    async fn try_enumerate_flatmap_pin_ref_async<'a, Map, U, E, const M: usize>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, Pin<&'a T>) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        self.try_enumerate_map_pin_ref_async(mapper).await.map(ArrayFlatten::flatten)
    }
    async fn try_enumerate_flatmap_pin_mut_async<'a, Map, U, E, const M: usize>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(usize, Pin<&'a mut T>) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        self.try_enumerate_map_pin_mut_async(mapper).await.map(ArrayFlatten::flatten)
    }
}