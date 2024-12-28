use core::{ops::AsyncFn, marker::Destruct};

use array_trait::Array;

use super::ArrayEnumerateMapAssign;

#[const_trait]
pub trait ArrayMapAssign<T, const N: usize>: Array<Item = T>
{
    fn map_assign<Map>(&mut self, mapper: Map)
    where
        Map: FnMut(T) -> T + ~const Destruct;
        
    async fn map_assign_async<Map>(&mut self, mapper: Map)
    where
        Map: AsyncFn(T) -> T + ~const Destruct;
        
    fn try_map_assign<Map, E>(&mut self, mapper: Map) -> Result<(), E>
    where
        Map: FnMut(T) -> Result<T, E> + ~const Destruct;
        
    async fn try_map_assign_async<Map, E>(&mut self, mapper: Map) -> Result<(), E>
    where
        Map: AsyncFn(T) -> Result<T, E> + ~const Destruct;
}

impl<T, const N: usize> ArrayMapAssign<T, N> for [T; N]
{
    fn map_assign<Map>(&mut self, mut mapper: Map)
    where
        Map: FnMut(T) -> T
    {
        self.enumerate_map_assign(|_, x| mapper(x))
    }
    
    async fn map_assign_async<Map>(&mut self, mapper: Map)
    where
        Map: AsyncFn(T) -> T + Destruct
    {
        self.enumerate_map_assign_async(|_, x| mapper(x)).await
    }
    
    fn try_map_assign<Map, E>(&mut self, mut mapper: Map) -> Result<(), E>
    where
        Map: FnMut(T) -> Result<T, E>
    {
        self.try_enumerate_map_assign(|_, x| mapper(x))
    }
    
    async fn try_map_assign_async<Map, E>(&mut self, mapper: Map) -> Result<(), E>
    where
        Map: AsyncFn(T) -> Result<T, E>
    {
        self.try_enumerate_map_assign_async(|_, x| mapper(x)).await
    }
}