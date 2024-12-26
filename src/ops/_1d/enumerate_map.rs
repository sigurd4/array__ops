use core::{marker::Destruct, mem::MaybeUninit, ops::AsyncFn};

use crate::{private::guard::PartialMapGuard, Runs, TryRuns};

use super::{ArrayJoin, Enumerate, Map};

#[const_trait]
pub trait EnumerateMap<T, const N: usize>: Enumerate<T, N> + Map<T, N>
{
    fn enumerate_map<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, T)> + ~const Destruct;
    fn enumerate_map_ref<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a T)> + ~const Destruct;
    fn enumerate_map_mut<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a mut T)> + ~const Destruct;

    fn enumerate_rmap<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, T)> + ~const Destruct;
    fn enumerate_rmap_ref<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a T)> + ~const Destruct;
    fn enumerate_rmap_mut<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a mut T)> + ~const Destruct;
        
    async fn enumerate_map_async<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, T)> + ~const Destruct;
    async fn enumerate_map_ref_async<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, &'a T)> + ~const Destruct,
        T: 'a;
    async fn enumerate_map_mut_async<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, &'a mut T)> + ~const Destruct,
        T: 'a;
        
    // TODO: use Result trait
    fn try_enumerate_map<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, T) -> Result<U, E> + ~const Destruct;
    fn try_enumerate_map_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_map_mut<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<U, E> + ~const Destruct,
        T: 'a;

    fn try_enumerate_rmap<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, T) -> Result<U, E> + ~const Destruct;
    fn try_enumerate_rmap_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rmap_mut<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<U, E> + ~const Destruct,
        T: 'a;
        
    async fn try_enumerate_map_async<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, T) -> Result<U, E> + ~const Destruct;
    async fn try_enumerate_map_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_map_mut_async<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, &'a mut T) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> EnumerateMap<T, N> for [T; N]
{
    fn enumerate_map<Map>(self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, T)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialMapGuard::new_left(
            self,
            &mut dst
        );

        while guard.more()
        {
            guard.enumerate_map(&mut mapper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    fn enumerate_map_ref<'a, Map>(&'a self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a T)>
    {
        crate::from_fn(|i| mapper(i, &self[i]))
    }
    fn enumerate_map_mut<'a, Map>(&'a mut self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a mut T)>
    {
        crate::from_fn(|i| unsafe {
            mapper(i, (&mut self[i] as *mut T).as_mut_unchecked())
        })
    }

    fn enumerate_rmap<Map>(self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, T)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialMapGuard::new_right(
            self,
            &mut dst
        );

        while guard.more()
        {
            guard.enumerate_map(&mut mapper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    fn enumerate_rmap_ref<'a, Map>(&'a self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a T)>
    {
        crate::rfrom_fn(|i| mapper(i, &self[i]))
    }
    fn enumerate_rmap_mut<'a, Map>(&'a mut self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a mut T)>
    {
        crate::rfrom_fn(|i| unsafe {
            mapper(i, (&mut self[i] as *mut T).as_mut_unchecked())
        })
    }
        
    async fn enumerate_map_async<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, T)>
    {
        self.enumerate_map(|i, x| mapper(i, x)).join_runs().await
    }
    async fn enumerate_map_ref_async<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, &'a T)>,
        T: 'a
    {
        self.enumerate_map_ref(|i, x| mapper(i, x)).join_runs().await
    }
    async fn enumerate_map_mut_async<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, &'a mut T)>,
        T: 'a
    {
        self.enumerate_map_mut(|i, x| mapper(i, x)).join_runs().await
    }
        
    // TODO: use Result trait
    fn try_enumerate_map<Map, U, E>(self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, T) -> Result<U, E>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialMapGuard::new_left(
            self,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_enumerate_map(&mut mapper)
            {
                result = Err(error);
                break
            }
        }
        guard.done();
    
        result.map(|()| unsafe {
            MaybeUninit::array_assume_init(dst)
        })
    }
    fn try_enumerate_map_ref<'a, Map, U, E>(&'a self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a T) -> Result<U, E>,
        T: 'a
    {
        crate::try_from_fn(|i| mapper(i, &self[i]))
    }
    fn try_enumerate_map_mut<'a, Map, U, E>(&'a mut self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<U, E>,
        T: 'a
    {
        crate::try_from_fn(|i| unsafe {
            mapper(i, (&mut self[i] as *mut T).as_mut_unchecked())
        })
    }

    fn try_enumerate_rmap<Map, U, E>(self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, T) -> Result<U, E>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialMapGuard::new_right(
            self,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_enumerate_map(&mut mapper)
            {
                result = Err(error);
                break
            }
        }
        guard.done();
    
        result.map(|()| unsafe {
            MaybeUninit::array_assume_init(dst)
        })
    }
    fn try_enumerate_rmap_ref<'a, Map, U, E>(&'a self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a T) -> Result<U, E>,
        T: 'a
    {
        crate::try_rfrom_fn(|i| mapper(i, &self[i]))
    }
    fn try_enumerate_rmap_mut<'a, Map, U, E>(&'a mut self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<U, E>,
        T: 'a
    {
        crate::try_rfrom_fn(|i| unsafe {
            mapper(i, (&mut self[i] as *mut T).as_mut_unchecked())
        })
    }
        
    async fn try_enumerate_map_async<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, T) -> Result<U, E>
    {
        self.enumerate_map(|i, x| mapper(i, x)).try_join_runs().await
    }
    async fn try_enumerate_map_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, &'a T) -> Result<U, E>,
        T: 'a
    {
        self.enumerate_map_ref(|i, x| mapper(i, x)).try_join_runs().await
    }
    async fn try_enumerate_map_mut_async<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, &'a mut T) -> Result<U, E>,
        T: 'a
    {
        self.enumerate_map_mut(|i, x| mapper(i, x)).try_join_runs().await
    }
}