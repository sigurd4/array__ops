use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use super::{ArrayEnumerate, ArrayEnumerateZipOuterWith, ArrayMapOuter};

#[const_trait]
pub trait ArrayEnumerateMapOuter<T, const N: usize>: ArrayEnumerate<T, N> + ArrayMapOuter<T, N>
{
    fn enumerate_map_outer<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(usize, usize, T, T)> + ~const Destruct,
        T: Copy;
    fn enumerate_map_outer_ref<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(usize, usize, &'a T, &'a T)> + ~const Destruct;
    fn enumerate_map_outer_pin_ref<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(usize, usize, Pin<&'a T>, Pin<&'a T>)> + ~const Destruct;
        
    async fn enumerate_map_outer_async<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, T, T)> + ~const Destruct,
        T: Copy;
    async fn enumerate_map_outer_ref_async<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, &'a T, &'a T)> + ~const Destruct,
        T: 'a;
    async fn enumerate_map_outer_pin_ref_async<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, Pin<&'a T>, Pin<&'a T>)> + ~const Destruct,
        T: 'a;
        
    fn try_enumerate_map_outer<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(usize, usize, T, T) -> Result<U, E> + ~const Destruct,
        T: Copy;
    fn try_enumerate_map_outer_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(usize, usize, &'a T, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_map_outer_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(usize, usize, Pin<&'a T>, Pin<&'a T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
        
    async fn try_enumerate_map_outer_async<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, T, T) -> Result<U, E> + ~const Destruct,
        T: Copy;
    async fn try_enumerate_map_outer_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, &'a T, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_map_outer_pin_ref_async<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, Pin<&'a T>, Pin<&'a T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayEnumerateMapOuter<T, N> for [T; N]
{
    fn enumerate_map_outer<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(usize, usize, T, T)> + Destruct,
        T: Copy
    {
        self.enumerate_zip_outer_with(self, mapper)
    }
    fn enumerate_map_outer_ref<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(usize, usize, &'a T, &'a T)>
    {
        self.enumerate_zip_outer_ref_with(&self, mapper)
    }
    fn enumerate_map_outer_pin_ref<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(usize, usize, Pin<&'a T>, Pin<&'a T>)>
    {
        self.enumerate_zip_outer_pin_ref_with(&self, mapper)
    }
    
    async fn enumerate_map_outer_async<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, T, T)>,
        T: Copy
    {
        self.enumerate_zip_outer_async_with(self, mapper).await
    }
    async fn enumerate_map_outer_ref_async<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, &'a T, &'a T)>,
        T: 'a
    {
        self.enumerate_zip_outer_ref_async_with(&self, mapper).await
    }
    async fn enumerate_map_outer_pin_ref_async<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, Pin<&'a T>, Pin<&'a T>)>,
        T: 'a
    {
        self.enumerate_zip_outer_pin_ref_async_with(&self, mapper).await
    }
    
    fn try_enumerate_map_outer<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(usize, usize, T, T) -> Result<U, E>,
        T: Copy
    {
        self.try_enumerate_zip_outer_with(self, mapper)
    }
    fn try_enumerate_map_outer_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(usize, usize, &'a T, &'a T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_outer_ref_with(&self, mapper)
    }
    fn try_enumerate_map_outer_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(usize, usize, Pin<&'a T>, Pin<&'a T>) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_outer_pin_ref_with(&self, mapper)
    }
    
    async fn try_enumerate_map_outer_async<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, T, T) -> Result<U, E>,
        T: Copy
    {
        self.try_enumerate_zip_outer_async_with(self, mapper).await
    }
    async fn try_enumerate_map_outer_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, &'a T, &'a T) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_outer_ref_async_with(&self, mapper).await
    }
    async fn try_enumerate_map_outer_pin_ref_async<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, Pin<&'a T>, Pin<&'a T>) -> Result<U, E>,
        T: 'a
    {
        self.try_enumerate_zip_outer_pin_ref_async_with(&self, mapper).await
    }
}