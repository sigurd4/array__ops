use core::{ops::AsyncFn, marker::Destruct};

use array_trait::Array;

use crate::{ops::ArrayJoin2D, Runs2D, TryRuns2D};

use super::{Enumerate, EnumerateZipOuterWith, MapOuter};

#[const_trait]
pub trait EnumerateMapOuter<T, const N: usize>: Enumerate<T, N> + MapOuter<T, N>
{
    fn enumerate_map_outer<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(usize, usize, T, T)> + ~const Destruct,
        T: Copy;
    fn enumerate_map_outer_ref<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(usize, usize, &'a T, &'a T)> + ~const Destruct;
        
    async fn enumerate_map_outer_async<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, T, T)> + ~const Destruct,
        T: Copy;
    async fn enumerate_map_outer_ref_async<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, &'a T, &'a T)> + ~const Destruct,
        T: 'a;
        
    fn try_enumerate_map_outer<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(usize, usize, T, T) -> Result<U, E> + ~const Destruct,
        T: Copy;
    fn try_enumerate_map_outer_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(usize, usize, &'a T, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
        
    async fn try_enumerate_map_outer_async<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, T, T) -> Result<U, E> + ~const Destruct,
        T: Copy;
    async fn try_enumerate_map_outer_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, &'a T, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> EnumerateMapOuter<T, N> for [T; N]
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
    
    async fn enumerate_map_outer_async<Map>(&self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, T, T)>,
        T: Copy
    {
        self.enumerate_map_outer(|i, j, x, y| mapper(i, j, x, y)).join_runs_2d().await
    }
    async fn enumerate_map_outer_ref_async<'a, Map>(&'a self, mapper: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(usize, usize, &'a T, &'a T)>,
        T: 'a
    {
        self.enumerate_map_outer_ref(|i, j, x, y| mapper(i, j, x, y)).join_runs_2d().await
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
    
    async fn try_enumerate_map_outer_async<Map, U, E>(&self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, T, T) -> Result<U, E>,
        T: Copy
    {
        self.enumerate_map_outer(|i, j, x, y| mapper(i, j, x, y)).try_join_runs_2d().await
    }
    async fn try_enumerate_map_outer_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(usize, usize, &'a T, &'a T) -> Result<U, E>,
        T: 'a
    {
        self.enumerate_map_outer_ref(|i, j, x, y| mapper(i, j, x, y)).try_join_runs_2d().await
    }
}