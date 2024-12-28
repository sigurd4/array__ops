use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use array_trait::Array;

use super::ArrayEnumerateMap;

#[const_trait]
pub trait ArrayMap<T, const N: usize>: Array<Item = T>
{
    /// Maps all values of an array with a given function.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ops::*;
    /// 
    /// const A: [u8; 4] = [1, 2, 3, 4];
    /// let b = A.map(|b| -(b as i8));
    /// 
    /// assert_eq!(b, [-1, -2, -3, -4]);
    /// ```
    fn map<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(T,)> + ~const Destruct;
    fn map_ref<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a T,)> + ~const Destruct;
    fn map_mut<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a mut T,)> + ~const Destruct;
    fn map_pin_ref<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(Pin<&'a T>,)> + ~const Destruct;
    fn map_pin_mut<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(Pin<&'a mut T>,)> + ~const Destruct;

    fn rmap<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(T,)> + ~const Destruct;
    fn rmap_ref<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a T,)> + ~const Destruct;
    fn rmap_mut<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a mut T,)> + ~const Destruct;
    fn rmap_pin_ref<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(Pin<&'a T>,)> + ~const Destruct;
    fn rmap_pin_mut<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(Pin<&'a mut T>,)> + ~const Destruct;
        
    async fn map_async<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(T,)> + ~const Destruct;
    async fn map_ref_async<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(&'a T,)> + ~const Destruct,
        T: 'a;
    async fn map_mut_async<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(&'a mut T,)> + ~const Destruct,
        T: 'a;
    async fn map_pin_ref_async<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(Pin<&'a T>,)> + ~const Destruct,
        T: 'a;
    async fn map_pin_mut_async<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(Pin<&'a mut T>,)> + ~const Destruct,
        T: 'a;
        
    // TODO: use Result trait
    fn try_map<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(T) -> Result<U, E> + ~const Destruct;
    fn try_map_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_map_mut<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a mut T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_map_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(Pin<&'a T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_map_pin_mut<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(Pin<&'a mut T>) -> Result<U, E> + ~const Destruct,
        T: 'a;

    fn try_rmap<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(T) -> Result<U, E> + ~const Destruct;
    fn try_rmap_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_rmap_mut<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a mut T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_rmap_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(Pin<&'a T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_rmap_pin_mut<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(Pin<&'a mut T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
        
    async fn try_map_async<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(T) -> Result<U, E> + ~const Destruct;
    async fn try_map_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(&'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_map_mut_async<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(&'a mut T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_map_pin_ref_async<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(Pin<&'a T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_map_pin_mut_async<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(Pin<&'a mut T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayMap<T, N> for [T; N]
{
    fn map<Map>(self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(T,)>
    {
        self.enumerate_map(|_, x| mapper(x))
    }
    fn map_ref<'a, Map>(&'a self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a T,)>
    {
        self.enumerate_map_ref(|_, x| mapper(x))
    }
    fn map_mut<'a, Map>(&'a mut self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a mut T,)>
    {
        self.enumerate_map_mut(|_, x| mapper(x))
    }
    fn map_pin_ref<'a, Map>(self: Pin<&'a Self>, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(Pin<&'a T>,)>
    {
        self.enumerate_map_pin_ref(|_, x| mapper(x))
    }
    fn map_pin_mut<'a, Map>(self: Pin<&'a mut Self>, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(Pin<&'a mut T>,)>
    {
        self.enumerate_map_pin_mut(|_, x| mapper(x))
    }
    
    fn rmap<Map>(self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(T,)>
    {
        self.enumerate_rmap(|_, x| mapper(x))
    }
    fn rmap_ref<'a, Map>(&'a self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a T,)>
    {
        self.enumerate_rmap_ref(|_, x| mapper(x))
    }
    fn rmap_mut<'a, Map>(&'a mut self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a mut T,)>
    {
        self.enumerate_rmap_mut(|_, x| mapper(x))
    }
    fn rmap_pin_ref<'a, Map>(self: Pin<&'a Self>, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(Pin<&'a T>,)>
    {
        self.enumerate_rmap_pin_ref(|_, x| mapper(x))
    }
    fn rmap_pin_mut<'a, Map>(self: Pin<&'a mut Self>, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(Pin<&'a mut T>,)>
    {
        self.enumerate_rmap_pin_mut(|_, x| mapper(x))
    }

    async fn map_async<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(T,)>
    {
        self.enumerate_map_async(|_, x| mapper(x)).await
    }
    async fn map_ref_async<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(&'a T,)>,
        T: 'a
    {
        self.enumerate_map_ref_async(|_, x| mapper(x)).await
    }
    async fn map_mut_async<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(&'a mut T,)>,
        T: 'a
    {
        self.enumerate_map_mut_async(|_, x| mapper(x)).await
    }
    async fn map_pin_ref_async<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(Pin<&'a T>,)>
    {
        self.enumerate_map_pin_ref_async(|_, x| mapper(x)).await
    }
    async fn map_pin_mut_async<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(Pin<&'a mut T>,)>
    {
        self.enumerate_map_pin_mut_async(|_, x| mapper(x)).await
    }
    
    fn try_map<Map, U, E>(self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(T) -> Result<U, E>
    {
        self.try_enumerate_map(|_, x| mapper(x))
    }
    fn try_map_ref<'a, Map, U, E>(&'a self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_ref(|_, x| mapper(x))
    }
    fn try_map_mut<'a, Map, U, E>(&'a mut self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a mut T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_mut(|_, x| mapper(x))
    }
    fn try_map_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(Pin<&'a T>) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_pin_ref(|_, x| mapper(x))
    }
    fn try_map_pin_mut<'a, Map, U, E>(self: Pin<&'a mut Self>, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(Pin<&'a mut T>) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_pin_mut(|_, x| mapper(x))
    }
    
    fn try_rmap<Map, U, E>(self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(T) -> Result<U, E>
    {
        self.try_enumerate_rmap(|_, x| mapper(x))
    }
    fn try_rmap_ref<'a, Map, U, E>(&'a self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_rmap_ref(|_, x| mapper(x))
    }
    fn try_rmap_mut<'a, Map, U, E>(&'a mut self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a mut T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_rmap_mut(|_, x| mapper(x))
    }
    fn try_rmap_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(Pin<&'a T>) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_rmap_pin_ref(|_, x| mapper(x))
    }
    fn try_rmap_pin_mut<'a, Map, U, E>(self: Pin<&'a mut Self>, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(Pin<&'a mut T>) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_rmap_pin_mut(|_, x| mapper(x))
    }

    async fn try_map_async<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(T) -> Result<U, E>
    {
        self.try_enumerate_map_async(|_, x| mapper(x)).await
    }
    async fn try_map_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(&'a T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_ref_async(|_, x| mapper(x)).await
    }
    async fn try_map_mut_async<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(&'a mut T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_mut_async(|_, x| mapper(x)).await
    }
    async fn try_map_pin_ref_async<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(Pin<&'a T>) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_pin_ref_async(|_, x| mapper(x)).await
    }
    async fn try_map_pin_mut_async<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(Pin<&'a mut T>) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_pin_mut_async(|_, x| mapper(x)).await
    }
}