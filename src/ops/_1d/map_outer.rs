use core::{ops::AsyncFn, marker::Destruct};

use array_trait::Array;

use super::EnumerateMapOuter;

#[const_trait]
pub trait MapOuter<T, const N: usize>: Array<Item = T>
{
    fn map_outer<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(T, T)> + ~const Destruct,
        T: Copy;
    fn map_outer_ref<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(&'a T, &'a T)> + ~const Destruct;
        
    async fn map_outer_async<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(T, T)> + ~const Destruct,
        T: Copy;
    async fn map_outer_ref_async<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(&'a T, &'a T)> + ~const Destruct,
        T: 'a;
        
    fn try_map_outer<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(T, T) -> Result<U, E> + ~const Destruct,
        T: Copy;
    fn try_map_outer_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(&'a T, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
        
    async fn try_map_outer_async<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(T, T) -> Result<U, E> + ~const Destruct,
        T: Copy;
    async fn try_map_outer_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(&'a T, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> MapOuter<T, N> for [T; N]
{
    fn map_outer<Map>(&self, mut mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(T, T)> + Destruct,
        T: Copy
    {
        self.enumerate_map_outer(|_, _, x, y| mapper(x, y))
    }
    fn map_outer_ref<'a, Map>(&'a self, mut mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(&'a T, &'a T)>
    {
        self.enumerate_map_outer_ref(|_, _, x, y| mapper(x, y))
    }
    
    async fn map_outer_async<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(T, T)>,
        T: Copy
    {
        self.enumerate_map_outer_async(|_, _, x, y| mapper(x, y)).await
    }
    async fn map_outer_ref_async<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(&'a T, &'a T)>,
        T: 'a
    {
        self.enumerate_map_outer_ref_async(|_, _, x, y| mapper(x, y)).await
    }
    
    fn try_map_outer<Map, U, E>(&self, mut mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(T, T) -> Result<U, E>,
        T: Copy
    {
        self.try_enumerate_map_outer(|_, _, x, y| mapper(x, y))
    }
    fn try_map_outer_ref<'a, Map, U, E>(&'a self, mut mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(&'a T, &'a T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_outer_ref(|_, _, x, y| mapper(x, y))
    }
    
    async fn try_map_outer_async<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(T, T) -> Result<U, E>,
        T: Copy
    {
        self.try_enumerate_map_outer_async(|_, _, x, y| mapper(x, y)).await
    }
    async fn try_map_outer_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(&'a T, &'a T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_map_outer_ref_async(|_, _, x, y| mapper(x, y)).await
    }
}