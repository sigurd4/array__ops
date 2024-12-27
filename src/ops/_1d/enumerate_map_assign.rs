use core::{ops::AsyncFn, marker::Destruct};

use super::{Enumerate, EnumerateVisit, MapAssign};

#[const_trait]
pub trait EnumerateMapAssign<T, const N: usize>: Enumerate<T, N> + MapAssign<T, N>
{
    fn enumerate_map_assign<Map>(&mut self, mapper: Map)
    where
        Map: FnMut(usize, T) -> T + ~const Destruct;
        
    async fn enumerate_map_assign_async<Map>(&mut self, mapper: Map)
    where
        Map: AsyncFn(usize, T) -> T + ~const Destruct;
        
    fn try_enumerate_map_assign<Map, E>(&mut self, mapper: Map) -> Result<(), E>
    where
        Map: FnMut(usize, T) -> Result<T, E> + ~const Destruct;
        
    async fn try_enumerate_map_assign_async<Map, E>(&mut self, mapper: Map) -> Result<(), E>
    where
        Map: AsyncFn(usize, T) -> Result<T, E> + ~const Destruct;
}

impl<T, const N: usize> EnumerateMapAssign<T, N> for [T; N]
{
    fn enumerate_map_assign<Map>(&mut self, mut mapper: Map)
    where
        Map: FnMut(usize, T) -> T
    {
        self.enumerate_visit_mut(|i, x| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, mapper(i, value))
        })
    }
    
    async fn enumerate_map_assign_async<Map>(&mut self, mapper: Map)
    where
        Map: AsyncFn(usize, T) -> T + Destruct
    {
        self.enumerate_visit_mut_async(async |i, x| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, mapper(i, value).await)
        }).await
    }
    
    fn try_enumerate_map_assign<Map, E>(&mut self, mut mapper: Map) -> Result<(), E>
    where
        Map: FnMut(usize, T) -> Result<T, E>
    {
        self.try_enumerate_visit_mut(|i, x| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, mapper(i, value)?);
            Ok(())
        })
    }
    
    async fn try_enumerate_map_assign_async<Map, E>(&mut self, mapper: Map) -> Result<(), E>
    where
        Map: AsyncFn(usize, T) -> Result<T, E>
    {
        self.try_enumerate_visit_mut_async(async |i, x| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, mapper(i, value).await?);
            Ok(())
        }).await
    }
}