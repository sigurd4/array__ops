use core::{marker::Destruct, ops::AsyncFn, pin::Pin};

use super::{ArrayJoin, ArrayEnumerate, ArrayMap};

#[const_trait]
pub trait ArrayEnumerateMap<T, const N: usize>: ArrayEnumerate<T, N> + ArrayMap<T, N>
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
    fn enumerate_map_pin_ref<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, Pin<&'a T>)> + ~const Destruct;
    fn enumerate_map_pin_mut<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, Pin<&'a mut T>)> + ~const Destruct;

    fn enumerate_rmap<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, T)> + ~const Destruct;
    fn enumerate_rmap_ref<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a T)> + ~const Destruct;
    fn enumerate_rmap_mut<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a mut T)> + ~const Destruct;
    fn enumerate_rmap_pin_ref<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, Pin<&'a T>)> + ~const Destruct;
    fn enumerate_rmap_pin_mut<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, Pin<&'a mut T>)> + ~const Destruct;
        
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
    async fn enumerate_map_pin_ref_async<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, Pin<&'a T>)> + ~const Destruct,
        T: 'a;
    async fn enumerate_map_pin_mut_async<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, Pin<&'a mut T>)> + ~const Destruct,
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
    fn try_enumerate_map_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, Pin<&'a T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_map_pin_mut<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, Pin<&'a mut T>) -> Result<U, E> + ~const Destruct,
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
    fn try_enumerate_rmap_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, Pin<&'a T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_enumerate_rmap_pin_mut<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, Pin<&'a mut T>) -> Result<U, E> + ~const Destruct,
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
    async fn try_enumerate_map_pin_ref_async<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, Pin<&'a T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_enumerate_map_pin_mut_async<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, Pin<&'a mut T>) -> Result<U, E> + ~const Destruct,
        T: 'a;
}

impl<T, const N: usize> ArrayEnumerateMap<T, N> for [T; N]
{
    fn enumerate_map<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, T)>
    {
        r#impl::enumerate_map(self, mapper)
    }
    fn enumerate_map_ref<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a T)>
    {
        r#impl::enumerate_map(self, mapper)
    }
    fn enumerate_map_mut<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a mut T)>
    {
        r#impl::enumerate_map(self, mapper)
    }
    fn enumerate_map_pin_ref<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, Pin<&'a T>)>
    {
        r#impl::enumerate_map(self, mapper)
    }
    fn enumerate_map_pin_mut<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, Pin<&'a mut T>)>
    {
        r#impl::enumerate_map(self, mapper)
    }

    fn enumerate_rmap<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, T)>
    {
        r#impl::enumerate_rmap(self, mapper)
    }
    fn enumerate_rmap_ref<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a T)>
    {
        r#impl::enumerate_rmap(self, mapper)
    }
    fn enumerate_rmap_mut<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, &'a mut T)>
    {
        r#impl::enumerate_rmap(self, mapper)
    }
    fn enumerate_rmap_pin_ref<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, Pin<&'a T>)>
    {
        r#impl::enumerate_rmap(self, mapper)
    }
    fn enumerate_rmap_pin_mut<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(usize, Pin<&'a mut T>)>
    {
        r#impl::enumerate_rmap(self, mapper)
    }
        
    async fn enumerate_map_async<Map>(self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, T)>
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map(|i, x| mapper(i, x)).join_runs().await
    }
    async fn enumerate_map_ref_async<'a, Map>(&'a self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, &'a T)>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_ref(|i, x| mapper(i, x)).join_runs().await
    }
    async fn enumerate_map_mut_async<'a, Map>(&'a mut self, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, &'a mut T)>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_mut(|i, x| mapper(i, x)).join_runs().await
    }
    async fn enumerate_map_pin_ref_async<'a, Map>(self: Pin<&'a Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, Pin<&'a T>)>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_pin_ref(|i, x| mapper(i, x)).join_runs().await
    }
    async fn enumerate_map_pin_mut_async<'a, Map>(self: Pin<&'a mut Self>, mapper: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(usize, Pin<&'a mut T>)>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_pin_mut(|i, x| mapper(i, x)).join_runs().await
    }
        
    // TODO: use Result trait
    fn try_enumerate_map<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, T) -> Result<U, E>
    {
        r#impl::try_enumerate_map(self, mapper)
    }
    fn try_enumerate_map_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a T) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_map(self, mapper)
    }
    fn try_enumerate_map_mut<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_map(self, mapper)
    }
    fn try_enumerate_map_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, Pin<&'a T>) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_map(self, mapper)
    }
    fn try_enumerate_map_pin_mut<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, Pin<&'a mut T>) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_map(self, mapper)
    }

    fn try_enumerate_rmap<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, T) -> Result<U, E>
    {
        r#impl::try_enumerate_rmap(self, mapper)
    }
    fn try_enumerate_rmap_ref<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a T) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_rmap(self, mapper)
    }
    fn try_enumerate_rmap_mut<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, &'a mut T) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_rmap(self, mapper)
    }
    fn try_enumerate_rmap_pin_ref<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, Pin<&'a T>) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_rmap(self, mapper)
    }
    fn try_enumerate_rmap_pin_mut<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(usize, Pin<&'a mut T>) -> Result<U, E>,
        T: 'a
    {
        r#impl::try_enumerate_rmap(self, mapper)
    }
        
    async fn try_enumerate_map_async<Map, U, E>(self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, T) -> Result<U, E>
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map(|i, x| mapper(i, x)).try_join_runs().await
    }
    async fn try_enumerate_map_ref_async<'a, Map, U, E>(&'a self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, &'a T) -> Result<U, E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_ref(|i, x| mapper(i, x)).try_join_runs().await
    }
    async fn try_enumerate_map_mut_async<'a, Map, U, E>(&'a mut self, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, &'a mut T) -> Result<U, E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_mut(|i, x| mapper(i, x)).try_join_runs().await
    }
    async fn try_enumerate_map_pin_ref_async<'a, Map, U, E>(self: Pin<&'a Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, Pin<&'a T>) -> Result<U, E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_pin_ref(|i, x| mapper(i, x)).try_join_runs().await
    }
    async fn try_enumerate_map_pin_mut_async<'a, Map, U, E>(self: Pin<&'a mut Self>, mapper: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(usize, Pin<&'a mut T>) -> Result<U, E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        self.enumerate_map_pin_mut(|i, x| mapper(i, x)).try_join_runs().await
    }
}

mod r#impl
{
    use core::mem::MaybeUninit;

    use crate::{form::ArrayForm, private::guard::{Dir, PartialMapGuard}};

    pub(super) fn enumerate_map<const N: usize, A, F>(src: A, mapper: F) -> [F::Output; N]
    where
        A: ArrayForm<N>,
        F: FnMut<(usize, A::Elem)>
    {
        enumerate_dmap::<{Dir::Left}, N, A, F>(src, mapper)
    }

    pub(super) fn enumerate_rmap<const N: usize, A, F>(src: A, mapper: F) -> [F::Output; N]
    where
        A: ArrayForm<N>,
        F: FnMut<(usize, A::Elem)>
    {
        enumerate_dmap::<{Dir::Right}, N, A, F>(src, mapper)
    }

    fn enumerate_dmap<const D: Dir, const N: usize, A, F>(src: A, mut mapper: F) -> [F::Output; N]
    where
        A: ArrayForm<N>,
        F: FnMut<(usize, A::Elem)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialMapGuard::<A, _, D, N>::new(
            src,
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

    pub(super) fn try_enumerate_map<const N: usize, A, F, U, E>(src: A, mapper: F) -> Result<[U; N], E>
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem) -> Result<U, E>
    {
        try_enumerate_dmap::<{Dir::Left}, N, A, F, U, E>(src, mapper)
    }

    pub(super) fn try_enumerate_rmap<const N: usize, A, F, U, E>(src: A, mapper: F) -> Result<[U; N], E>
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem) -> Result<U, E>
    {
        try_enumerate_dmap::<{Dir::Right}, N, A, F, U, E>(src, mapper)
    }

    fn try_enumerate_dmap<const D: Dir, const N: usize, A, F, U, E>(src: A, mut mapper: F) -> Result<[U; N], E>
    where
        A: ArrayForm<N>,
        F: FnMut(usize, A::Elem) -> Result<U, E>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialMapGuard::<A, U, D, N>::new(
            src,
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
}