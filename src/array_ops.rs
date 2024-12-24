use core::{alloc::Allocator, borrow::{Borrow, BorrowMut}, cmp::Ordering, marker::Destruct, mem::{ManuallyDrop, MaybeUninit}, ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign, AsyncFn}, pin::Pin, simd::{LaneCount, Simd, SimdElement, SupportedLaneCount}};

use array_trait::Array;
use private::guard::{PartialDivideAndConquerGuard, PartialEmptyGuard, PartialInitGuard, PartialMapGuard, PartialZipEmptyGuard, PartialZipGuard};
use slice_ops::{is_power_of, Padded};

#[cfg(feature = "alloc")]
use alloc::{alloc::Global, boxed::Box};

use super::*;

#[const_trait]
pub trait ArrayOps<T, const N: usize>: Array + IntoIterator<Item = T>
    + Borrow<[T; N]>
    + BorrowMut<[T; N]>
{
    fn split_len(n: usize) -> (usize, usize);
    fn rsplit_len(n: usize) -> (usize, usize);
        
    fn split_ptr(&self, n: usize) -> (*const T, *const T);
    fn split_mut_ptr(&mut self, n: usize) -> (*mut T, *mut T);

    fn rsplit_ptr(&self, n: usize) -> (*const T, *const T);
    fn rsplit_mut_ptr(&mut self, n: usize) -> (*mut T, *mut T);

    fn from_fn<F>(fill: F) -> Self
    where
        F: FnMut(usize) -> T + ~const Destruct;
    fn rfrom_fn<F>(fill: F) -> Self
    where
        F: FnMut(usize) -> T + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn from_fn_boxed<F>(fill: F) -> Box<Self>
    where
        F: FnMut(usize) -> T + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn rfrom_fn_boxed<F>(fill: F) -> Box<Self>
    where
        F: FnMut(usize) -> T + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn from_fn_boxed_in<F, A>(fill: F, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T + ~const Destruct,
        A: Allocator;
    #[cfg(feature = "alloc")]
    fn rfrom_fn_boxed_in<F, A>(fill: F, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T + ~const Destruct,
        A: Allocator;
        
    fn try_from_fn<F, E>(fill: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct;
    fn try_rfrom_fn<F, E>(fill: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn try_from_fn_boxed<F, E>(fill: F) -> Result<Box<Self>, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn try_rfrom_fn_boxed<F, E>(fill: F) -> Result<Box<Self>, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct;
    #[cfg(feature = "alloc")]
    fn try_from_fn_boxed_in<F, E, A>(fill: F, alloc: A) -> Result<Box<Self, A>, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct,
        A: Allocator;
    #[cfg(feature = "alloc")]
    fn try_rfrom_fn_boxed_in<F, E, A>(fill: F, alloc: A) -> Result<Box<Self, A>, E>
    where
        F: FnMut(usize) -> Result<T, E> + ~const Destruct,
        A: Allocator;

    async fn from_fn_async<F>(fill: F) -> Self
    where
        F: AsyncFn(usize) -> T + ~const Destruct;
    async fn try_from_fn_async<F, E>(fill: F) -> Result<Self, E>
    where
        F: AsyncFn(usize) -> Result<T, E> + ~const Destruct;

    fn truncate<const M: usize>(self) -> [T; M]
    where
        T: ~const Destruct,
        [(); N - M]:;
    fn rtruncate<const M: usize>(self) -> [T; M]
    where
        T: ~const Destruct,
        [(); N - M]:;
        
    fn truncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:;
    fn rtruncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:;
        
    fn truncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:;
    fn rtruncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:;

    fn resize<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct;
    fn rresize<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        T: ~const Destruct;

    fn extend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;
    fn rextend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + ~const Destruct,
        [(); M - N]:;

    fn reformulate_length<const M: usize>(self) -> [T; M]
    where
        [(); M - N]:,
        [(); N - M]:;
    
    fn reformulate_length_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); M - N]:,
        [(); N - M]:;
        
    fn reformulate_length_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); M - N]:,
        [(); N - M]:;
        
    fn try_reformulate_length<const M: usize>(self) -> Result<[T; M], [T; N]>;
    
    fn try_reformulate_length_ref<const M: usize>(&self) -> Option<&[T; M]>;
        
    fn try_reformulate_length_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>;

    fn into_collumn(self) -> [[T; 1]; N];
    fn as_collumn(&self) -> &[[T; 1]; N];
    fn as_collumn_mut(&mut self) -> &mut [[T; 1]; N];

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
    fn map<Map>(self, map: Map) -> [Map::Output; N]
    where
        Map: FnMut<(T,)> + ~const Destruct;
    fn map_ref<'a, Map>(&'a self, map: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a T,)> + ~const Destruct;
    fn map_mut<'a, Map>(&'a mut self, map: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a mut T,)> + ~const Destruct;
    fn map_outer<Map>(&self, map: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(T, T)> + ~const Destruct,
        T: Copy;
    fn map_outer_ref<'a, Map>(&'a self, map: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(&'a T, &'a T)> + ~const Destruct;
    fn zip_with<Map, Rhs>(self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: FnMut<(T, Rhs::Elem)> + ~const Destruct;
    fn zip_ref_with<'a, Map, Rhs>(&'a self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: FnMut<(&'a T, Rhs::Elem)> + ~const Destruct;
    fn zip_mut_with<'a, Map, Rhs>(&'a mut self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: FnMut<(&'a mut T, Rhs::Elem)> + ~const Destruct;
    fn zip_outer_with<Map, Rhs, const M: usize>(&self, rhs: &Rhs, map: Map) -> [[Map::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: FnMut<(T, Rhs::Elem)> + ~const Destruct,
        T: Copy;
    fn zip_outer_ref_with<'a, Map, Rhs, const M: usize>(&'a self, rhs: &Rhs, map: Map) -> [[Map::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: FnMut<(&'a T, Rhs::Elem)> + ~const Destruct;
    fn flatmap<Map, O, const M: usize>(self, map: Map) -> [O; N*M]
    where
        Map: FnMut<(T,), Output = [O; M]> + ~const Destruct,
        [(); N*M]:;
    fn flatmap_ref<'a, Map, O, const M: usize>(&'a self, map: Map) -> [O; N*M]
    where
        Map: FnMut<(&'a T,), Output = [O; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn flatmap_mut<'a, Map, O, const M: usize>(&'a mut self, map: Map) -> [O; N*M]
    where
        Map: FnMut<(&'a mut T,), Output = [O; M]> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    fn map_assign<Map>(&mut self, map: Map)
    where
        Map: FnMut(T) -> T + ~const Destruct;
    fn zip_assign_with<Rhs, Zip>(&mut self, rhs: Rhs, map: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> T + ~const Destruct;
        
    async fn map_async<Map>(self, map: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(T,)> + ~const Destruct;
    async fn map_ref_async<'a, Map>(&'a self, map: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(&'a T,)> + ~const Destruct,
        T: 'a;
    async fn map_mut_async<'a, Map>(&'a mut self, map: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(&'a mut T,)> + ~const Destruct,
        T: 'a;
    async fn map_outer_async<Map>(&self, map: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(T, T)> + ~const Destruct,
        T: Copy;
    async fn map_outer_ref_async<'a, Map>(&'a self, map: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(&'a T, &'a T)> + ~const Destruct,
        T: 'a;
    async fn zip_async_with<Map, Rhs>(self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn<(T, Rhs::Elem)> + ~const Destruct;
    async fn zip_ref_async_with<'a, Map, Rhs>(&'a self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn<(&'a T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn zip_mut_async_with<'a, Map, Rhs>(&'a mut self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn<(&'a mut T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn zip_outer_async_with<Map, Rhs, const M: usize>(&self, rhs: &Rhs, map: Map) -> [[Map::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: AsyncFn<(T, Rhs::Elem)> + ~const Destruct,
        T: Copy;
    async fn zip_outer_ref_async_with<'a, Map, Rhs, const M: usize>(&'a self, rhs: &Rhs, map: Map) -> [[Map::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: AsyncFn<(&'a T, Rhs::Elem)> + ~const Destruct,
        T: 'a;
    async fn flatmap_async<Map, O, const M: usize>(self, map: Map) -> [O; N*M]
    where
        Map: AsyncFn(T) -> [O; M] + ~const Destruct,
        [(); N*M]:;
    async fn flatmap_ref_async<'a, Map, O, const M: usize>(&'a self, map: Map) -> [O; N*M]
    where
        Map: AsyncFn(&'a T) -> [O; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn flatmap_mut_async<'a, Map, O, const M: usize>(&'a mut self, map: Map) -> [O; N*M]
    where
        Map: AsyncFn(&'a mut T) -> [O; M] + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn map_assign_async<Map>(&mut self, map: Map)
    where
        Map: AsyncFn(T) -> T + ~const Destruct;
    async fn zip_assign_async_with<Rhs, Zip>(&mut self, rhs: Rhs, map: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> T + ~const Destruct;
        
    fn try_map<Map, U, E>(self, map: Map) -> Result<[U; N], E>
    where
        Map: FnMut(T) -> Result<U, E> + ~const Destruct;
    fn try_map_ref<'a, Map, U, E>(&'a self, map: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_map_mut<'a, Map, U, E>(&'a mut self, map: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a mut T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_map_outer<Map, U, E>(&self, map: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(T, T) -> Result<U, E> + ~const Destruct,
        T: Copy;
    fn try_map_outer_ref<'a, Map, U, E>(&'a self, map: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(&'a T, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_zip_with<Map, Rhs, U, E>(self, rhs: Rhs, map: Map) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Map: FnMut(T, Rhs::Elem) -> Result<U, E> + ~const Destruct;
    fn try_zip_ref_with<'a, Map, Rhs, U, E>(&'a self, rhs: Rhs, map: Map) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Map: FnMut(&'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_zip_mut_with<'a, Map, Rhs, U, E>(&'a mut self, rhs: Rhs, map: Map) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Map: FnMut(&'a mut T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_zip_outer_with<Map, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, map: Map) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: FnMut(T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: Copy;
    fn try_zip_outer_ref_with<'a, Map, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, map: Map) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: FnMut(&'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    fn try_flatmap<Map, U, E, const M: usize>(self, map: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(T) -> Result<[U; M], E> + ~const Destruct;
    fn try_flatmap_ref<'a, Map, U, E, const M: usize>(&'a self, map: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_flatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, map: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a mut T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a;
    fn try_map_assign<Map, E>(&mut self, map: Map) -> Result<(), E>
    where
        Map: FnMut(T) -> Result<T, E> + ~const Destruct;
    fn try_zip_assign_with<Rhs, Zip, E>(&mut self, rhs: Rhs, map: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> Result<T, E> + ~const Destruct;
        
    async fn try_map_async<Map, U, E>(self, map: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(T) -> Result<U, E> + ~const Destruct;
    async fn try_map_ref_async<'a, Map, U, E>(&'a self, map: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(&'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_map_mut_async<'a, Map, U, E>(&'a mut self, map: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(&'a mut T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_map_outer_async<Map, U, E>(&self, map: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(T, T) -> Result<U, E> + ~const Destruct,
        T: Copy;
    async fn try_map_outer_ref_async<'a, Map, U, E>(&'a self, map: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(&'a T, &'a T) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_zip_async_with<Map, Rhs, U, E>(self, rhs: Rhs, map: Map) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn(T, Rhs::Elem) -> Result<U, E> + ~const Destruct;
    async fn try_zip_ref_async_with<'a, Map, Rhs, U, E>(&'a self, rhs: Rhs, map: Map) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn(&'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_zip_mut_async_with<'a, Map, Rhs, U, E>(&'a mut self, rhs: Rhs, map: Map) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn(&'a mut T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_zip_outer_async_with<Map, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, map: Map) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: AsyncFn(T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: Copy;
    async fn try_zip_outer_ref_async_with<'a, Map, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, map: Map) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: AsyncFn(&'a T, Rhs::Elem) -> Result<U, E> + ~const Destruct,
        T: 'a;
    async fn try_flatmap_async<Map, U, E, const M: usize>(self, map: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(T) -> Result<[U; M], E> + ~const Destruct,
        [(); N*M]:;
    async fn try_flatmap_ref_async<'a, Map, U, E, const M: usize>(&'a self, map: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(&'a T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn try_flatmap_mut_async<'a, Map, U, E, const M: usize>(&'a mut self, map: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(&'a mut T) -> Result<[U; M], E> + ~const Destruct,
        T: 'a,
        [(); N*M]:;
    async fn try_map_assign_async<Map, E>(&mut self, map: Map) -> Result<(), E>
    where
        Map: AsyncFn(T) -> Result<T, E> + ~const Destruct;
    async fn try_zip_assign_async_with<Rhs, Zip, E>(&mut self, rhs: Rhs, map: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<T, E> + ~const Destruct;

    /// Combines two arrays with possibly different items into parallel, where each element lines up in the same position.
    /// 
    /// This method can be executed at compile-time, as opposed to the standard-library method.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// const A: [u8; 4] = [4, 3, 2, 1];
    /// const B: [&str; 4] = ["four", "three", "two", "one"];
    /// let c = A.zip(B);
    /// 
    /// assert_eq!(c, [(4, "four"), (3, "three"), (2, "two"), (1, "one")]);
    /// ```
    fn zip<Z>(self, other: Z) -> [(T, Z::Elem); N]
    where
        Z: ArrayForm<N>;
    fn zip_ref<Z>(&self, other: Z) -> [(&T, Z::Elem); N]
    where
        Z: ArrayForm<N>;
    fn zip_mut<Z>(&mut self, other: Z) -> [(&mut T, Z::Elem); N]
    where
        Z: ArrayForm<N>;
    fn zip_outer<Z, const M: usize>(&self, other: &Z) -> [[(T, Z::Elem); M]; N]
    where
        T: Copy,
        Z: ArrayForm<M, Elem: Copy>;
    fn zip_outer_ref<Z, const M: usize>(&self, other: &Z) -> [[(&T, Z::Elem); M]; N]
    where
        Z: ArrayForm<M, Elem: Copy>;

    fn enumerate(self) -> [(usize, T); N];

    fn diagonal<const H: usize, const W: usize>(self) -> [[T; W]; H]
    where
        T: Default + Copy,
        [(); H - N]:,
        [(); W - N]:;

    fn toeplitz_matrix(&self) -> [[T; N]; N]
    where
        T: Copy;
    fn hankel_matrix<const M: usize>(&self, r: &[T; M]) -> [[T; M]; N]
    where
        T: Copy;
    
    /// Differentiates array (discrete calculus)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut a = [1, 2, 3];
    /// 
    /// a.differentiate();
    /// 
    /// assert_eq!(a, [1, 2 - 1, 3 - 2]);
    /// ```
    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy;
    
    /// Integrates array (discrete calculus)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut a = [1, 2, 3];
    /// 
    /// a.integrate();
    /// 
    /// assert_eq!(a, [1, 1 + 2, 1 + 2 + 3])
    /// ```
    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy;
        
    /// Performs a linear search for the first value that equals `x`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                   v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let i = x.find(&5).unwrap();
    /// 
    /// assert_eq!(i, 4);
    /// assert_eq!(x[i], 5);
    /// ```
    fn find(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    /// Performs a linear search for the first value that satisfies the given predicate.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                      v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn > 5; 
    /// 
    /// let i = x.find_by(f).unwrap();
    /// 
    /// assert_eq!(i, 5);
    /// ```
    fn find_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
    /// Performs a linear search for the first value that matches the given key given a hashing function.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //             v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn % 2;
    /// 
    /// let i = x.find_by_key(&0, f).unwrap();
    /// 
    /// assert_eq!(i, 2);
    /// ```
    fn find_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a;
        
    /// Performs a linear search from the right for the first value that equals `x`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                               v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let i = x.rfind(&5).unwrap();
    /// 
    /// assert_eq!(i, 8);
    /// assert_eq!(x[i], 5);
    /// ```
    fn rfind(&self, x: &T) -> Option<usize>
    where
        T: PartialEq;
    /// Performs a linear search from the right for the first value that satisfies the given predicate.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                            v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn > 5;
    /// 
    /// let i = x.rfind_by(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn rfind_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
    /// Performs a linear search from the right for the first value that matches the given key given a hashing function.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                            v
    /// let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    /// 
    /// let f = |&xn| xn % 2;
    /// 
    /// let i = x.rfind_by_key(&0, f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn rfind_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a;

    /// Reduces elements in array into one element, using a given operand
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::ArrayOps;
    /// 
    /// const A: [u8; 3] = [1, 2, 3];
    /// 
    /// let r: u8 = A.reduce(|a, b| a + b).unwrap();
    /// 
    /// assert_eq!(r, 6);
    /// ```
    fn reduce<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T + ~const Destruct;
    fn reduce_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T + ~const Destruct;
    fn reduce_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T + ~const Destruct;
        
    fn fold<F, O>(self, default: O, fold: F) -> O
    where
        F: FnMut(O, T) -> O + ~const Destruct;
    fn fold_ref<'a, F, O>(&'a self, default: O, fold: F) -> O
    where
        F: FnMut(O, &'a T) -> O + ~const Destruct,
        T: 'a;
    fn fold_mut<'a, F, O>(&'a mut self, default: O, fold: F) -> O
    where
        F: FnMut(O, &'a mut T) -> O + ~const Destruct,
        T: 'a;
        
    fn divide_and_conquer<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T + ~const Destruct;
    fn divide_and_conquer_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T + ~const Destruct;
    fn divide_and_conquer_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T + ~const Destruct;
        
    async fn divide_and_conquer_async<F>(self, reduce: F) -> Option<T>
    where
        F: AsyncFn(T, T) -> T + ~const Destruct;
    async fn divide_and_conquer_ref_async<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: AsyncFn(&'a T, &'a T) -> &'a T + ~const Destruct,
        T: 'a;
    async fn divide_and_conquer_mut_async<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: AsyncFn(&'a mut T, &'a mut T) -> &'a mut T + ~const Destruct,
        T: 'a;

    fn try_sum(self) -> Option<T>
    where
        T: AddAssign;
    fn sum_from<S>(self, from: S) -> S
    where
        S: AddAssign<T>;
    async fn try_sum_async(self) -> Option<T>
    where
        T: AddAssign;
        
    fn try_product(self) -> Option<T>
    where
        T: MulAssign;
    fn product_from<P>(self, from: P) -> P
    where
        P: MulAssign<T>;
    async fn try_product_async(self) -> Option<T>
    where
        T: MulAssign;

    fn max(self) -> Option<T>
    where
        T: Ord;
    fn min(self) -> Option<T>
    where
        T: Ord;
    async fn max_async(self) -> Option<T>
    where
        T: Ord;
    async fn min_async(self) -> Option<T>
    where
        T: Ord;
        
    fn first_max(self) -> Option<T>
    where
        T: PartialOrd<T>;
        
    fn first_min(self) -> Option<T>
    where
        T: PartialOrd<T>;
        
    /// Performs an argument reduction, finding the final righthand operand for which the comparison yields true.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// fn my_argmax<T>(slice: &[T]) -> Option<usize>
    /// where
    ///     T: PartialOrd
    /// {
    ///     slice.argreduce(PartialOrd::gt)
    /// }
    /// 
    /// fn my_argmin<T>(slice: &[T]) -> Option<usize>
    /// where
    ///     T: PartialOrd
    /// {
    ///     slice.argreduce(PartialOrd::lt)
    /// }
    /// 
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// assert_eq!(my_argmax(&x), x.argmax());
    /// assert_eq!(my_argmin(&x), x.argmin());
    /// ```
    fn argreduce<'a, F>(&'a self, reduction: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> bool /*+ ~const Destruct*/,
        T: 'a;
        
    /// Performs an argument reduction on the hashed values, finding the final righthand operand for which the comparison yields true.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// fn hasher(str: &&str) -> i32
    /// {
    ///     i32::from_str_radix(str, 10).unwrap()
    /// }
    /// 
    /// fn my_argmax(slice: &[&str]) -> Option<usize>
    /// {
    ///     slice.argreduce_key(PartialOrd::gt, hasher)
    /// }
    /// 
    /// fn my_argmin(slice: &[&str]) -> Option<usize>
    /// {
    ///     slice.argreduce_key(PartialOrd::lt, hasher)
    /// }
    /// 
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// assert_eq!(my_argmax(&x), x.argmax_by_key(hasher));
    /// assert_eq!(my_argmin(&x), x.argmin_by_key(hasher));
    /// ```
    fn argreduce_key<'a, B, FR, FB>(&'a self, reduction: FR, hasher: FB) -> Option<usize>
    where
        FR: FnMut(&B, &B) -> bool /*+ ~const Destruct*/,
        FB: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        T: 'a;
        
    /// Finds the index of the maximum value in the slice.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let i = x.argmax().unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    /// Finds the index of the minimum value in the slice.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                              v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let i = x.argmin().unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd<T>;
    /// Finds the index of the maximum value in the slice, given a comparison predicate.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let f = Ord::cmp;
    /// 
    /// let i = x.argmax_by(f).unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering /*+ ~const Destruct*/,
        T: 'a;
    /// Finds the index of the minimum value in the slice, given a comparison predicate.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                              v
    /// let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];
    /// 
    /// let f = Ord::cmp;
    /// 
    /// let i = x.argmin_by(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering /*+ ~const Destruct*/,
        T: 'a;
    /// Finds the index of the maximum key in the slice, given a hashing function.
    /// 
    /// If there are multiple maxima, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                       v
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// let f = |&e| i32::from_str_radix(e, 10).unwrap();
    /// 
    /// let i = x.argmax_by_key(f).unwrap();
    /// 
    /// assert_eq!(i, 3);
    /// ```
    fn argmax_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialOrd,
        T: 'a;
    /// Finds the index of the minimum key in the slice, given a hashing function.
    /// 
    /// If there are multiple minimums, only the first will have its index returned.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// //                                  v
    /// let x = ["1", "5", "5", "6", "2", "-1", "0", "-4", "-1", "6"];
    /// 
    /// let f = |&e| i32::from_str_radix(e, 10).unwrap();
    /// 
    /// let i = x.argmin_by_key(f).unwrap();
    /// 
    /// assert_eq!(i, 7);
    /// ```
    fn argmin_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialOrd,
        T: 'a;

    /// Visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// let mut i = 0;
    /// 
    /// x.visit(|&e| {
    ///     i += 1;
    ///     assert_eq!(i, e)
    /// });
    /// ```
    fn visit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(&'a T) /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// x.visit_mut(|e| {
    ///     i += 1;
    ///     *e = i;
    /// });
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    fn visit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(&'a mut T) /*+ ~const Destruct*/,
        T: 'a;
    /// Visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_visit(|&e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     assert_eq!(i, e);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// ```
    fn try_visit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_visit_mut(|e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     *e = i;
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// assert_eq!(x, [1, 2, 3, 4, 0, 0, 0, 0])
    /// ```
    fn try_visit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
        
    /// Visits each element once, from right to left.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// let mut i = 0;
    /// 
    /// x.rvisit(|&e| {
    ///     i += 1;
    ///     assert_eq!(i, e)
    /// });
    /// ```
    fn rvisit<'a, F>(&'a self, visitor: F)
    where
        F: FnMut(&'a T) /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from right to left.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// x.rvisit_mut(|e| {
    ///     i += 1;
    ///     *e = i;
    /// });
    /// 
    /// assert_eq!(x, [8, 7, 6, 5, 4, 3, 2, 1]);
    /// ```
    fn rvisit_mut<'a, F>(&'a mut self, visitor: F)
    where
        F: FnMut(&'a mut T) /*+ ~const Destruct*/,
        T: 'a;
    /// Visits each element once, from right to left, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_rvisit(|&e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     assert_eq!(i, e);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// ```
    fn try_rvisit<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from right to left, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [0; 8];
    /// 
    /// let mut i = 0;
    /// 
    /// let result = x.try_rvisit_mut(|e| {
    ///     i += 1;
    ///     if i > 4
    ///     {
    ///         return Err(i)
    ///     }
    ///     *e = i;
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(5));
    /// assert_eq!(x, [0, 0, 0, 0, 4, 3, 2, 1])
    /// ```
    fn try_rvisit_mut<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
        
    /// Visits each element once, asyncronously.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// # tokio_test::block_on(async {
    /// x.visit_async(async |&e| {
    ///     assert_eq!(x[e - 1], e)
    /// }).await;
    /// # })
    /// ```
    async fn visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(&'a T) /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, asyncronously.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// # tokio_test::block_on(async {
    /// x.visit_mut_async(async |e| {
    ///     *e = 9 - *e
    /// }).await;
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// # })
    /// ```
    async fn visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(&'a mut T) /*+ ~const Destruct*/,
        T: 'a;
    /// Visits each element once, asyncronously, or short-circuits if visitor returns error.
    /// 
    /// # Warning
    /// 
    /// When any of the tasks return an error, all other tasks will be ignored. The tasks are not nessecarily stopped, and may still be running in the background.
    /// 
    /// If you want to wait for all tasks to complete, keep polling the future until it returns an [Ok](core::result::Result).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// # tokio_test::block_on(async {
    /// let result = x.try_visit_async(async |&e| {
    ///     if e > 4
    ///     {
    ///         return Err(e)
    ///     }
    ///     assert_eq!(x[e - 1], e);
    ///     Ok(())
    /// }).await;
    /// 
    /// assert!(result == Err(5) || result == Err(6) || result == Err(7) || result == Err(8));
    /// # })
    /// ```
    async fn try_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, asyncronously, or short-circuits if visitor returns error.
    /// 
    /// # Warning
    /// 
    /// When any of the tasks return an error, all other tasks will be ignored. The tasks are not nessecarily stopped, and may still be running in the background.
    /// 
    /// If you want to wait for all tasks to complete, keep polling the future until it returns an [Ok](core::result::Result).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// # tokio_test::block_on(async {
    /// let result = x.try_visit_mut_async(async |e| {
    ///     if *e <= 4
    ///     {
    ///         return Err(*e)
    ///     }
    ///     *e = 9 - *e;
    ///     Ok(())
    /// }).await;
    /// 
    /// assert_eq!(x[..4], [1, 2, 3, 4]);
    /// assert!(x[4] == 5 || x[4] == 4);
    /// assert!(x[5] == 6 || x[5] == 3);
    /// assert!(x[6] == 7 || x[6] == 2);
    /// assert!(x[7] == 8 || x[7] == 1);
    /// # })
    /// ```
    async fn try_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a mut T) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    
    /// Visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let y = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.visit_with(|&a, b| {
    ///     assert_eq!(a, b)
    /// }, y);
    /// ```
    fn visit_with<'a, F, Rhs>(&'a self, with: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from left to right.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [0; 8];
    /// let y = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// x.visit_mut_with(|a, b| {
    ///     *a += b
    /// }, y);
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    fn visit_mut_with<'a, F, Rhs>(&'a mut self, with: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a;
    /// Visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let y = [1, 2, 3, 4, -1, -2, -3, -4];
    /// 
    /// let result = x.try_visit_with(|&a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     assert_eq!(a, b);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(-1));
    /// ```
    fn try_visit_with<'a, E, F, Rhs>(&'a self, with: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from left to right, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [0; 8];
    /// let y = [1, 2, 3, 4, -1, -2, -3, -4];
    /// 
    /// let result = x.try_visit_mut_with(|a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     *a = b;
    ///     Ok(())
    /// }, y);
    /// 
    /// assert_eq!(result, Err(-1));
    /// assert_eq!(x, [1, 2, 3, 4, 0, 0, 0, 0])
    /// ```
    fn try_visit_mut_with<'a, E, F, Rhs>(&'a mut self, with: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
        
    /// Visits each element once, from right to left.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// let y = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// x.rvisit_with(|&a, b| {
    ///     assert_eq!(a, b)
    /// }, y);
    /// ```
    fn rvisit_with<'a, F, Rhs>(&'a self, with: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from right to left.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [0; 8];
    /// let y = [8, 7, 6, 5, 4, 3, 2, 1];
    /// 
    /// x.rvisit_mut_with(|a, b| {
    ///     *a = b;
    /// }, y);
    /// 
    /// assert_eq!(x, [8, 7, 6, 5, 4, 3, 2, 1]);
    /// ```
    fn rvisit_mut_with<'a, F, Rhs>(&'a mut self, with: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a;
    /// Visits each element once, from right to left, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// let y = [-4, -3, -2, -1, 4, 3, 2, 1];
    /// 
    /// let result = x.try_rvisit_with(|&a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     assert_eq!(a, b);
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(-1));
    /// ```
    fn try_rvisit_with<'a, E, F, Rhs>(&'a self, with: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, from right to left, or short-circuits if visitor returns error.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [0; 8];
    /// let y = [-4, -3, -2, -1, 4, 3, 2, 1];
    /// 
    /// let result = x.try_rvisit_mut_with(|a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     *a = b;
    ///     Ok(())
    /// });
    /// 
    /// assert_eq!(result, Err(-1));
    /// assert_eq!(x, [0, 0, 0, 0, 4, 3, 2, 1])
    /// ```
    fn try_rvisit_mut_with<'a, E, F, Rhs>(&'a mut self, with: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
        
    /// Visits each element once, asyncronously.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let y = [1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    /// # tokio_test::block_on(async {
    /// x.visit_async_with(async |&a, b| {
    ///     assert_eq!(x[a - 1], a)
    ///     assert_eq!(y[b - 1], b)
    ///     assert_eq!(a, b);
    /// }, y).await;
    /// # })
    /// ```
    async fn visit_async_with<'a, F, Rhs>(&'a self, with: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, asyncronously.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// let y = [-7, -5, -3, -1, 1, 3, 5, 7];
    /// 
    /// # tokio_test::block_on(async {
    /// x.visit_mut_async_with(async |a, b| {
    ///     *a += b
    /// }, y).await;
    /// 
    /// assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
    /// # })
    /// ```
    async fn visit_mut_async_with<'a, F, Rhs>(&'a mut self, with: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a mut T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a;
    /// Visits each element once, asyncronously, or short-circuits if visitor returns error.
    /// 
    /// # Warning
    /// 
    /// When any of the tasks return an error, all other tasks will be ignored. The tasks are not nessecarily stopped, and may still be running in the background.
    /// 
    /// If you want to wait for all tasks to complete, keep polling the future until it returns an [Ok](core::result::Result).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let y = [1, 2, 3, 4, -1, -2, -3, -4];
    /// 
    /// # tokio_test::block_on(async {
    /// let result = x.try_visit_async_with(async |&a, b| {
    ///     assert_eq!(x[a - 1], a);
    ///     assert_eq!(y[b - 1], b);
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     assert_eq!(a, b);
    ///     Ok(())
    /// }, y).await;
    /// 
    /// assert!(result == Err(-1) || result == Err(-2) || result == Err(-3) || result == Err(-4));
    /// # })
    /// ```
    async fn try_visit_async_with<'a, E, F, Rhs>(&'a self, with: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
    /// Mutably visits each element once, asyncronously, or short-circuits if visitor returns error.
    /// 
    /// # Warning
    /// 
    /// When any of the tasks return an error, all other tasks will be ignored. The tasks are not nessecarily stopped, and may still be running in the background.
    /// 
    /// If you want to wait for all tasks to complete, keep polling the future until it returns an [Ok](core::result::Result).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut x = [8, 7, 6, 5, 4, 3, 2, 1];
    /// let y = [-7, -5, -3, -1, -1, -2, -3, -4];
    /// 
    /// # tokio_test::block_on(async {
    /// let result = x.try_visit_mut_async_with(async |a, b| {
    ///     if b < 0
    ///     {
    ///         return Err(b)
    ///     }
    ///     *a += b;
    ///     Ok(())
    /// }, y).await;
    /// 
    /// assert_eq!(x[..4], [1, 2, 3, 4]);
    /// assert!(x[4] == 5 || x[4] == 4);
    /// assert!(x[5] == 6 || x[5] == 3);
    /// assert!(x[6] == 7 || x[6] == 2);
    /// assert!(x[7] == 8 || x[7] == 1);
    /// assert!(result == Err(-1) || result == Err(-2) || result == Err(-3) || result == Err(-4));
    /// # })
    /// ```
    async fn try_visit_mut_async_with<'a, E, F, Rhs>(&'a mut self, with: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a mut T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a;
        
    fn add_all<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>,
        Rhs: Copy;
    fn sub_all<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>,
        Rhs: Copy;
    fn mul_all<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy;
    fn div_all<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>,
        Rhs: Copy;
    fn rem_all<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy;
    fn shl_all<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>,
        Rhs: Copy;
    fn shr_all<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy;
    fn bitor_all<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>,
        Rhs: Copy;
    fn bitand_all<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>,
        Rhs: Copy;
    fn bitxor_all<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>,
        Rhs: Copy;
    fn rsub_all<Lhs>(self, lhs: Lhs) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Copy + Sub<T>;
    fn rdiv_all<Lhs>(self, lhs: Lhs) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Copy + Div<T>;
    fn neg_all(self) -> [<T as Neg>::Output; N]
    where
        T: Neg;
    fn not_all(self) -> [<T as Not>::Output; N]
    where
        T: Not;
        
    async fn add_all_async<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>,
        Rhs: Copy;
    async fn sub_all_async<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>,
        Rhs: Copy;
    async fn mul_all_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy;
    async fn div_all_async<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>,
        Rhs: Copy;
    async fn rem_all_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy;
    async fn shl_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>,
        Rhs: Copy;
    async fn shr_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy;
    async fn bitor_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>,
        Rhs: Copy;
    async fn bitand_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>,
        Rhs: Copy;
    async fn bitxor_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>,
        Rhs: Copy;
    async fn rsub_all_async<Lhs>(self, lhs: Lhs) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Copy + Sub<T>;
    async fn rdiv_all_async<Lhs>(self, lhs: Lhs) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Copy + Div<T>;
    async fn neg_all_async(self) -> [<T as Neg>::Output; N]
    where
        T: Neg;
    async fn not_all_async(self) -> [<T as Not>::Output; N]
    where
        T: Not;

    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy;
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy;
    fn rsub_assign_all<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Sub<T, Output = T>;
    fn rdiv_assign_all<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Div<T, Output = T>;
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>;
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>;
        
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy;
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy;
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy;
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy;
    async fn rem_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy;
    async fn shl_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy;
    async fn shr_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy;
    async fn bitor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy;
    async fn bitand_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy;
    async fn bitxor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy;
    async fn rsub_assign_all_async<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Sub<T, Output = T>;
    async fn rdiv_assign_all_async<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Div<T, Output = T>;
    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>;
    async fn not_assign_all_async(&mut self)
    where
        T: Not<Output = T>;
    
    fn add_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>;
    fn sub_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>;
    fn mul_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>;
    fn div_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>;
    fn rem_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>;
    fn shl_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>;
    fn shr_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>;
    fn bitor_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>;
    fn bitand_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>;
    fn bitxor_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>;
    fn rsub_each<Lhs>(self, lhs: [Lhs; N]) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Sub<T>;
    fn rdiv_each<Lhs>(self, lhs: [Lhs; N]) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Div<T>;
        
    async fn add_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>;
    async fn sub_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>;
    async fn mul_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>;
    async fn div_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>;
    async fn rem_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>;
    async fn shl_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>;
    async fn shr_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>;
    async fn bitor_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>;
    async fn bitand_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>;
    async fn bitxor_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>;
    async fn rsub_each_async<Lhs>(self, lhs: [Lhs; N]) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Sub<T>;
    async fn rdiv_each_async<Lhs>(self, lhs: [Lhs; N]) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Div<T>;

    fn add_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: AddAssign<Rhs>;
    fn sub_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: SubAssign<Rhs>;
    fn mul_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: MulAssign<Rhs>;
    fn div_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: DivAssign<Rhs>;
    fn rem_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: RemAssign<Rhs>;
    fn shl_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: ShlAssign<Rhs>;
    fn shr_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: ShrAssign<Rhs>;
    fn bitor_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitOrAssign<Rhs>;
    fn bitand_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitAndAssign<Rhs>;
    fn bitxor_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitXorAssign<Rhs>;
    fn rsub_assign_each<Lhs>(&mut self, lhs: [Lhs; N])
    where
        Lhs: Sub<T, Output = T>;
    fn rdiv_assign_each<Lhs>(&mut self, lhs: [Lhs; N])
    where
        Lhs: Div<T, Output = T>;
        
    async fn add_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: AddAssign<Rhs>;
    async fn sub_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: SubAssign<Rhs>;
    async fn mul_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: MulAssign<Rhs>;
    async fn div_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: DivAssign<Rhs>;
    async fn rem_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: RemAssign<Rhs>;
    async fn shl_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: ShlAssign<Rhs>;
    async fn shr_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: ShrAssign<Rhs>;
    async fn bitor_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitOrAssign<Rhs>;
    async fn bitand_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitAndAssign<Rhs>;
    async fn bitxor_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitXorAssign<Rhs>;
    async fn rsub_assign_each_async<Lhs>(&mut self, lhs: [Lhs; N])
    where
        Lhs: Sub<T, Output = T>;
    async fn rdiv_assign_each_async<Lhs>(&mut self, lhs: [Lhs; N])
    where
        Lhs: Div<T, Output = T>;

    fn try_mul_dot<Rhs>(self, rhs: [Rhs; N]) -> Option<<T as Mul<Rhs>>::Output>
    where
        T: Mul<Rhs, Output: AddAssign>;
    async fn try_mul_dot_async<Rhs>(self, rhs: [Rhs; N]) -> Option<<T as Mul<Rhs>>::Output>
    where
        T: Mul<Rhs, Output: AddAssign>;
    fn proj<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output>>::Output; N]
    where
        T: Mul<Rhs, Output: AddAssign + Div<<T as Mul>::Output, Output: Copy>> + Mul<T, Output: AddAssign> + Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output> + Copy;
    async fn proj_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output>>::Output; N]
    where
        T: Mul<Rhs, Output: AddAssign + Div<<T as Mul>::Output, Output: Copy>> + Mul<T, Output: AddAssign> + Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output> + Copy;

    fn mul_dot_bias<Rhs>(self, rhs: [Rhs; N], bias: <T as Mul<Rhs>>::Output) -> <T as Mul<Rhs>>::Output
    where
        T: Mul<Rhs, Output: AddAssign>;
    async fn mul_dot_bias_async<Rhs>(self, rhs: [Rhs; N], bias: <T as Mul<Rhs>>::Output) -> <T as Mul<Rhs>>::Output
    where
        T: Mul<Rhs, Output: AddAssign>;

    fn mul_outer<Rhs, const M: usize>(&self, rhs: &[Rhs; M]) -> [[<T as Mul<Rhs>>::Output; M]; N]
    where
        T: Mul<Rhs> + Copy,
        Rhs: Copy;
    async fn mul_outer_async<Rhs, const M: usize>(&self, rhs: &[Rhs; M]) -> [[<T as Mul<Rhs>>::Output; M]; N]
    where
        T: Mul<Rhs> + Copy,
        Rhs: Copy;
        
    /// Computes the general cross-product of the two arrays (as if vectors, in the mathematical sense).
    /// 
    /// # Example
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use array__ops::*;
    /// 
    /// const U: [f64; 3] = [1.0, 0.0, 0.0];
    /// const V: [f64; 3] = [0.0, 1.0, 0.0];
    /// 
    /// let w = U.mul_cross([&V]);
    /// 
    /// assert_eq!(w, [0.0, 0.0, 1.0]);
    /// ```
    fn mul_cross<Rhs>(&self, rhs: [&[Rhs; N]; N - 2]) -> [<T as Sub>::Output; N]
    where
        T: MulAssign<Rhs> + Sub + Copy,
        Rhs: Copy;
        
    async fn mul_cross_async<Rhs>(&self, rhs: [&[Rhs; N]; N - 2]) -> [<T as Sub>::Output; N]
    where
        T: MulAssign<Rhs> + Sub + Copy,
        Rhs: Copy;

    fn try_magnitude_squared(self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy;
    async fn try_magnitude_squared_async(self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy;

    /// Chains two arrays with the same item together.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let a = ["one", "two"];
    /// let b = ["three"];
    /// 
    /// assert_eq!(a.chain(b), ["one", "two", "three"]);
    /// ```
    fn chain<const M: usize>(self, rhs: [T; M]) -> [T; N + M];

    /// Chains two arrays with the same item together in reverse.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let a = ["two", "three"];
    /// let b = ["one"];
    /// 
    /// assert_eq!(a.rchain(b), ["one", "two", "three"]);
    /// ```
    fn rchain<const M: usize>(self, rhs: [T; M]) -> [T; N + M];
    
    fn into_rotate_left(self, n: usize) -> [T; N];

    fn into_rotate_right(self, n: usize) -> [T; N];

    fn into_shift_many_left<const M: usize>(self, items: [T; M]) -> ([T; M], [T; N]);
        
    fn into_shift_many_right<const M: usize>(self, items: [T; M]) -> ([T; N], [T; M]);

    fn into_shift_left(self, item: T) -> (T, [T; N]);
        
    fn into_shift_right(self, item: T) -> ([T; N], T);

    fn rotate_left2(&mut self, n: usize);

    fn rotate_right2(&mut self, n: usize);

    fn shift_many_left<const M: usize>(&mut self, items: [T; M]) -> [T; M];
    
    fn shift_many_right<const M: usize>(&mut self, items: [T; M]) -> [T; M];
    
    fn shift_left(&mut self, item: T) -> T;

    fn shift_right(&mut self, item: T) -> T;

    /// Distributes items of an array equally across a given width, then provides the rest as a separate array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = ["ping 1", "pong 1", "ping 2", "pong 2", "ping 3", "pong 3", "uhh..."];
    /// 
    /// let ([ping, pong], rest) = array.spread_chunks::<2>();
    /// 
    /// assert_eq!(ping, ["ping 1", "ping 2", "ping 3"]);
    /// assert_eq!(pong, ["pong 1", "pong 2", "pong 3"]);
    /// assert_eq!(rest, ["uhh..."]);
    /// ```
    fn spread_chunks<const M: usize>(self) -> ([[T; N / M]; M], [T; N % M])
    where
        [(); M - 1]:,
        [(); N / M]:,
        [(); N % M]:;

    /// Distributes items of an array-slice equally across a given width, then provides the rest as a separate array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    fn spread_chunks_ref<const M: usize>(&self) -> ([&[Padded<T, M>; N / M]; M], &[T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:;
    
    /// Distributes items of a mutable array-slice equally across a given width, then provides the rest as a separate mutable array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let mut array = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20"];
    /// 
    /// let (threes, _) = array.spread_chunks_mut::<3>();
    /// 
    /// for fizz in threes.into_iter().last().unwrap()
    /// {
    ///     **fizz = "fizz";
    /// }
    /// 
    /// let (fives, _) = array.spread_chunks_mut::<5>();
    /// 
    /// for buzz in fives.into_iter().last().unwrap()
    /// {
    ///     **buzz = "buzz";
    /// }
    /// 
    /// let (fifteens, _) = array.spread_chunks_mut::<15>();
    /// 
    /// for fizzbuzz in fifteens.into_iter().last().unwrap()
    /// {
    ///     **fizzbuzz = "fizzbuzz";
    /// }
    /// 
    /// assert_eq!(array, ["1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14", "fizzbuzz", "16", "17", "fizz", "19", "buzz"]);
    /// 
    /// ```
    fn spread_chunks_mut<const M: usize>(&mut self) -> ([&mut [Padded<T, M>; N / M]; M], &mut [T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:;
    
    /// Distributes items of an array equally across a given width, then provides the leftmost rest as a separate array.
    fn rspread_chunks<const M: usize>(self) -> ([T; N % M], [[T; N / M]; M])
    where
        [(); M - 1]:,
        [(); N / M]:,
        [(); N % M]:,
        T: Copy;

    /// Distributes items of an array-slice equally across a given width, then provides the leftmost rest as a separate array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    /// 
    /// let (zero, [odd, even]) = array.rspread_chunks_ref::<2>();
    /// 
    /// assert_eq!(*zero, [0]);
    /// assert_eq!(odd.each_ref().map(|padding| **padding), [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    /// assert_eq!(even.each_ref().map(|padding| **padding), [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    /// ```
    fn rspread_chunks_ref<const M: usize>(&self) -> (&[T; N % M], [&[Padded<T, M>; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:;
    /// Distributes items of a mutable array-slice equally across a given width, then provides the leftmost rest as a separate mutable array-slice.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array__ops::*;
    /// 
    /// let mut array = ["the", "beat", "goes", "1", "2", "3", "4", "5", "6", "7", "8"];
    /// 
    /// let (start, [boots, n, cats, and]) = array.rspread_chunks_mut::<4>();
    /// 
    /// for boots in boots
    /// {
    ///     **boots = "boots";
    /// }
    /// for n in n
    /// {
    ///     **n = "n";
    /// }
    /// for cats in cats
    /// {
    ///     **cats = "cats";
    /// }
    /// for and in and
    /// {
    ///     **and = "and";
    /// }
    /// 
    /// assert_eq!(array, ["the", "beat", "goes", "boots", "n", "cats", "and", "boots", "n", "cats", "and"]);
    /// ```
    fn rspread_chunks_mut<const M: usize>(&mut self) -> (&mut [T; N % M], [&mut [Padded<T, M>; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:;
    
    /// Distributes items of an array equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = *b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
    /// 
    /// let [lower_case, upper_case] = array.spread_chunks_exact::<2>();
    /// 
    /// assert_eq!(lower_case, *b"abcdefghijklmnopqrstuvwxyz");
    /// assert_eq!(upper_case, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// ```
    fn spread_chunks_exact<const M: usize>(self) -> [[T; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:,
        [(); N / M]:;
    
    /// Distributes items of an array-slice equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array__ops::*;
    /// 
    /// let statement = ["s", "he", "be", "lie", "ve", "d"];
    /// 
    /// let [interpretation2, interpretation1] = statement.spread_chunks_exact_ref::<2>();
    /// 
    /// assert_eq!(interpretation1, &["he", "lie", "d"]);
    /// assert_eq!(interpretation2, &["s", "be", "ve"]);
    /// ```
    fn spread_chunks_exact_ref<const M: usize>(&self) -> [&[Padded<T, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:;

    /// Distributes items of a mutable array-slice equally across a given width, with no rest.
    /// 
    /// The width must be a factor of the array length, otherwise it will not compile.
    /// 
    /// The spread-out slices are given in padded arrays. Each padded item can be borrowed into a reference to the array's item.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// #![feature(array_methods)]
    /// 
    /// use array__ops::*;
    /// 
    /// let mut array = *b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
    /// 
    /// let [lower_case, upper_case] = array.spread_chunks_exact_mut::<2>();
    /// 
    /// assert_eq!(lower_case.each_ref().map(|padding| padding.borrow()), b"abcdefghijklmnopqrstuvwxyz".each_ref());
    /// assert_eq!(upper_case.each_ref().map(|padding| padding.borrow()), b"ABCDEFGHIJKLMNOPQRSTUVWXYZ".each_ref());
    /// 
    /// for c in upper_case
    /// {
    ///     **c = b'_';
    /// }
    /// 
    /// assert_eq!(array, *b"a_b_c_d_e_f_g_h_i_j_k_l_m_n_o_p_q_r_s_t_u_v_w_x_y_z_")
    /// ```
    fn spread_chunks_exact_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:;

    /// Divides an array into chunks, then yielding the rest in a separate array.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = ["carrot", "potato", "beet", "tomato", "kiwi", "banana", "cherry", "peach", "strawberry", "nine volt batteries"];
    /// 
    /// let ([root_vegetables, technically_berries, stone_fruits], not_for_human_consumption) = array.chunks::<3>();
    /// 
    /// assert_eq!(root_vegetables, ["carrot", "potato", "beet"]);
    /// assert_eq!(technically_berries, ["tomato", "kiwi", "banana"]);
    /// assert_eq!(stone_fruits, ["cherry", "peach", "strawberry"]);
    /// assert_eq!(not_for_human_consumption, ["nine volt batteries"]);
    /// ```
    fn chunks<const M: usize>(self) -> ([[T; M]; N / M], [T; N % M])
    where
        [(); N % M]:,
        [(); N / M]:;
    /// Divides an array-slice into chunks, then yielding the rest in a separate array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let transistors = ["2N3904", "2N2222A", "BC107", "AC127", "OC7", "NKT275", "2SK30A", "2N5458", "J108", "2N7000", "BS170"];
    /// 
    /// let ([silicon_bjts, germanium_bjts, jfets], mosfets) = transistors.chunks_ref::<3>();
    /// 
    /// assert_eq!(silicon_bjts, &["2N3904", "2N2222A", "BC107"]);
    /// assert_eq!(germanium_bjts, &["AC127", "OC7", "NKT275"]);
    /// assert_eq!(jfets, &["2SK30A", "2N5458", "J108"]);
    /// assert_eq!(mosfets, &["2N7000", "BS170"]);
    /// ```
    fn chunks_ref<const M: usize>(&self) -> (&[[T; M]; N / M], &[T; N % M])
    where
        [(); N % M]:,
        [(); N / M]:;
    /// Divides a mutable array-slice into chunks, then yielding the rest in a separate mutable array-slice.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let mut array = [0, 1, 0, 1, 0, 1, 6];
    /// 
    /// let (pairs, last) = array.chunks_mut::<2>();
    /// 
    /// for (i, pair) in pairs.into_iter().enumerate()
    /// {
    ///     for number in pair
    ///     {
    ///         *number += i*2;
    ///     }
    /// }
    /// 
    /// assert_eq!(array, [0, 1, 2, 3, 4, 5, 6]);
    /// ```
    fn chunks_mut<const M: usize>(&mut self) -> (&mut [[T; M]; N / M], &mut [T; N % M])
    where
        [(); N % M]:,
        [(); N / M]:;
    
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate mutable array-slice.
    fn rchunks<const M: usize>(self) -> ([T; N % M], [[T; M]; N / M])
    where
        [(); N % M]:,
        [(); N / M]:;
    /// Divides an array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn rchunks_ref<const M: usize>(&self) -> (&[T; N % M], &[[T; M]; N / M])
    where
        [(); N % M]:,
        [(); N / M]:;
    /// Divides a mutable array-slice into chunks, then yielding the leftmost rest in a separate array-slice.
    fn rchunks_mut<const M: usize>(&mut self) -> (&mut [T; N % M], &mut [[T; M]; N / M])
    where
        [(); N % M]:,
        [(); N / M]:;
    
    /// Divides an array into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    /// 
    /// let [lower_half, upper_half] = array.chunks_exact::<5>();
    /// 
    /// assert_eq!(lower_half, [0.0, 0.1, 0.2, 0.3, 0.4]);
    /// assert_eq!(upper_half, [0.5, 0.6, 0.7, 0.8, 0.9]);
    /// ```
    fn chunks_exact<const M: usize>(self) -> [[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    /// Divides an array-slice into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// #![feature(generic_arg_infer)]
    /// 
    /// use array__ops::*;
    /// 
    /// let array = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    /// 
    /// let [lower_half, upper_half] = array.chunks_exact_ref::<5>();
    /// 
    /// assert_eq!(lower_half, &[0.0, 0.1, 0.2, 0.3, 0.4]);
    /// assert_eq!(upper_half, &[0.5, 0.6, 0.7, 0.8, 0.9]);
    /// ```
    fn chunks_exact_ref<const M: usize>(&self) -> &[[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;
    /// Divides a mutable array-slice into chunks, with no rest.
    /// 
    /// The chunk length must be a factor of the array length, otherwise it will not compile.
    fn chunks_exact_mut<const M: usize>(&mut self) -> &mut [[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:;

    fn array_simd<const M: usize>(self) -> ([Simd<T, M>; N / M], [T; N % M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    
    fn array_rsimd<const M: usize>(self) -> ([T; N % M], [Simd<T, M>; N / M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:;
    
    fn array_simd_exact<const M: usize>(self) -> [Simd<T, M>; N / M]
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:;

    /// Splits an array at a chosen index.
    fn split_array<const M: usize>(self) -> ([T; M], [T; N - M])
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as array-slices.
    fn split_array_ref2<const M: usize>(&self) -> (&[T; M], &[T; N - M])
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as mutable array-slices.
    fn split_array_mut2<const M: usize>(&mut self) -> (&mut [T; M], &mut [T; N - M])
    where
        [(); N - M]:;
    
    /// Splits an array at a chosen index, where the index goes from right to left.
    fn rsplit_array<const M: usize>(self) -> ([T; N - M], [T; M])
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as array-slices, where the index goes from right to left.
    fn rsplit_array_ref2<const M: usize>(&self) -> (&[T; N - M], &[T; M])
    where
        [(); N - M]:;
    /// Splits an array at a chosen index as mutable array-slices, where the index goes from right to left.
    fn rsplit_array_mut2<const M: usize>(&mut self) -> (&mut [T; N - M], &mut [T; M])
    where
        [(); N - M]:;

    fn each_ref(&self) -> [&T; N];
    fn each_mut(&mut self) -> [&mut T; N];
    fn each_pin_ref(self: Pin<&Self>) -> [Pin<&T>; N];
    fn each_pin_mut(self: Pin<&mut Self>) -> [Pin<&mut T>; N];
    
    /// Performs the bit-reverse permutation. Length must be a power of 2.
    /// 
    /// # Example
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.bit_rev_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
    /// ```
    fn bit_rev_permutation(&mut self)
    where
        [(); is_power_of(N, 2) as usize - 1]:;
    fn digit_rev_permutation<const R: usize>(&mut self)
    where
        [(); is_power_of(N, R) as usize - 1]:;

    /// Performs the grey code permutation. Length must be a power of 2.
    /// 
    /// # Example
    /// ```rust
    /// use array__ops::*;
    /// 
    /// let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
    /// 
    /// arr.grey_code_permutation();
    /// 
    /// assert_eq!(arr, [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100])
    /// ```
    fn grey_code_permutation(&mut self)
    where
        [(); N.is_power_of_two() as usize - 1]:;
}

pub const fn split_ptr<T, const N: usize>(array: &[T; N], mid: usize) -> (*const T, *const T)
{
    let ptr = array.as_ptr();
    (ptr, unsafe {ptr.add(slice_ops::split_len(N, mid).0)})
}

pub const fn split_mut_ptr<T, const N: usize>(array: &mut [T; N], mid: usize) -> (*mut T, *mut T)
{
    let ptr = array.as_mut_ptr();
    (ptr, unsafe {ptr.add(slice_ops::split_len(N, mid).0)})
}

pub const fn rsplit_ptr<T, const N: usize>(array: &[T; N], mid: usize) -> (*const T, *const T)
{
    let ptr = array.as_ptr();
    (ptr, unsafe {ptr.add(slice_ops::rsplit_len(N, mid).0)})
}

pub const fn rsplit_mut_ptr<T, const N: usize>(array: &mut [T; N], mid: usize) -> (*mut T, *mut T)
{
    let ptr = array.as_mut_ptr();
    (ptr, unsafe {ptr.add(slice_ops::rsplit_len(N, mid).0)})
}

pub const fn truncate_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> &[T; M]
where
    [(); N - M]:
{
    crate::split_array_ref(array).0
}
pub const fn rtruncate_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> &[T; M]
where
    [(); N - M]:
{
    crate::rsplit_array_ref(array).1
}

pub const fn truncate_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> &mut [T; M]
where
    [(); N - M]:
{
    crate::split_array_mut(array).0
}
pub const fn rtruncate_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> &mut [T; M]
where
    [(); N - M]:
{
    crate::rsplit_array_mut(array).1
}

pub const fn into_rotate_left<T, const N: usize>(array: [T; N], n: usize) -> [T; N]
{
    let n = n % N;
    let mut rotated = MaybeUninit::<[T; N]>::uninit();

    let (left, right) = slice_ops::split_len(N, n);
    let (src_left, src_right) = crate::split_ptr(&array, n);

    unsafe {
        let (dst_left, dst_right) = crate::rsplit_mut_ptr(rotated.assume_init_mut(), n);

        core::ptr::copy_nonoverlapping(src_right, dst_left, right);
        core::ptr::copy_nonoverlapping(src_left, dst_right, left);
    }

    core::mem::forget(array);

    unsafe {
        MaybeUninit::assume_init(rotated)
    }
}

pub const fn into_rotate_right<T, const N: usize>(array: [T; N], n: usize) -> [T; N]
{
    let n = n % N;
    let mut rotated = MaybeUninit::<[T; N]>::uninit();

    let (left, right) = slice_ops::rsplit_len(N, n);
    let (src_left, src_right) = crate::rsplit_ptr(&array, n);

    unsafe {
        let (dst_left, dst_right) = crate::split_mut_ptr(rotated.assume_init_mut(), n);

        core::ptr::copy_nonoverlapping(src_right, dst_left, right);
        core::ptr::copy_nonoverlapping(src_left, dst_right, left);
    }

    core::mem::forget(array);

    unsafe {
        MaybeUninit::assume_init(rotated)
    }
}

pub const fn into_shift_many_left<T, const N: usize, const M: usize>(array: [T; N], items: [T; M]) -> ([T; M], [T; N])
{
    unsafe {
        private::overlap_swap_transmute(array, items)
    }
}

pub const fn into_shift_many_right<T, const N: usize, const M: usize>(array: [T; N], items: [T; M]) -> ([T; N], [T; M])
{
    unsafe {
        private::overlap_swap_transmute(items, array)
    }
}

pub const fn into_shift_left<T, const N: usize>(array: [T; N], item: T) -> (T, [T; N])
{
    unsafe {
        private::overlap_swap_transmute(array, item)
    }
}
pub const fn into_shift_right<T, const N: usize>(array: [T; N], item: T) -> ([T; N], T)
{
    unsafe {
        private::overlap_swap_transmute(item, array)
    }
}

pub const fn rotate_left<T, const N: usize>(array: &mut [T; N], n: usize)
{
    let n = n % N;
    unsafe {
        let mut buffer: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();

        let (left, right) = slice_ops::split_len(N, n);
        let (src_left, src_right) = crate::split_mut_ptr(&mut buffer, n);
        let (dst_left, dst_right) = crate::rsplit_mut_ptr(array, n);

        core::ptr::copy_nonoverlapping(
            dst_left,
            src_left.cast(),
            N
        );
        core::ptr::copy_nonoverlapping(
            src_right,
            dst_left.cast(),
            right
        );
        core::ptr::copy_nonoverlapping(
            src_left,
            dst_right.cast(),
            left
        );
        core::mem::forget(buffer);
    }
}

pub const fn rotate_right<T, const N: usize>(array: &mut [T; N], n: usize)
{
    let n = n % N;
    unsafe {
        let mut buffer: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();

        let (left, right) = slice_ops::rsplit_len(N, n);
        let (src_left, src_right) = crate::rsplit_mut_ptr(&mut buffer, n);
        let (dst_left, dst_right) = crate::split_mut_ptr(array, n);

        core::ptr::copy_nonoverlapping(
            dst_left,
            src_left.cast(),
            N
        );
        core::ptr::copy_nonoverlapping(
            src_right,
            dst_left.cast(),
            right
        );
        core::ptr::copy_nonoverlapping(
            src_left,
            dst_right.cast(),
            left
        );
        core::mem::forget(buffer);
    }
}

pub const fn shift_many_left<T, const N: usize, const M: usize>(array: &mut [T; N], items: [T; M]) -> [T; M]
{
    unsafe {
        let mut buffer: private::Pair<[T; M], [MaybeUninit<T>; N]> = private::Pair::new(items, MaybeUninit::uninit_array());
        let buf_left = buffer.left.as_mut_ptr();
        let buf_right = buf_left.add(N);

        core::ptr::copy_nonoverlapping(buffer.left.as_ptr(), buf_right, M);
        core::ptr::copy_nonoverlapping(array.as_ptr(), buf_left, N);

        let (overflow, shifted) = buffer.unpack_mandrop();

        core::ptr::copy_nonoverlapping((&shifted as *const ManuallyDrop<[MaybeUninit<T>; N]>).cast::<T>(), array.as_mut_ptr(), N);
        core::mem::forget(shifted);

        ManuallyDrop::into_inner(overflow)
    }
}

pub const fn shift_many_right<T, const N: usize, const M: usize>(array: &mut [T; N], items: [T; M]) -> [T; M]
{
    unsafe {
        let mut buffer: private::Pair<[MaybeUninit<T>; N], [T; M]> = private::Pair::new(MaybeUninit::uninit_array(), items);
        let buf_left = buffer.left.as_mut_ptr().cast::<T>();
        let buf_right = buf_left.add(M);

        core::ptr::copy_nonoverlapping(buffer.right.as_ptr(), buf_left, M);
        core::ptr::copy_nonoverlapping(array.as_ptr(), buf_right, N);

        let (shifted, overflow) = buffer.unpack_mandrop();

        core::ptr::copy_nonoverlapping((&shifted as *const ManuallyDrop<[MaybeUninit<T>; N]>).cast::<T>(), array.as_mut_ptr(), N);
        core::mem::forget(shifted);

        ManuallyDrop::into_inner(overflow)
    }
}

pub const fn shift_left<T, const N: usize>(array: &mut [T; N], item: T) -> T
{
    unsafe {
        let mut buffer: private::Pair<T, [MaybeUninit<T>; N]> = private::Pair::new(item, MaybeUninit::uninit_array());
        let buf_left = &mut buffer.left as *mut T;
        let buf_right = buf_left.add(N);

        core::ptr::copy_nonoverlapping(&buffer.left as *const T, buf_right, 1);
        core::ptr::copy_nonoverlapping(array.as_ptr(), buf_left, N);

        let (overflow, shifted) = buffer.unpack_mandrop();

        core::ptr::copy_nonoverlapping((&shifted as *const ManuallyDrop<[MaybeUninit<T>; N]>).cast::<T>(), array.as_mut_ptr(), N);
        core::mem::forget(shifted);

        ManuallyDrop::into_inner(overflow)
    }
}
pub const fn shift_right<T, const N: usize>(array: &mut [T; N], item: T) -> T
{
    unsafe {
        let mut buffer: private::Pair<[MaybeUninit<T>; N], T> = private::Pair::new(MaybeUninit::uninit_array(), item);
        let buf_left = buffer.left.as_mut_ptr().cast::<T>();
        let buf_right = buf_left.add(1);

        core::ptr::copy_nonoverlapping(&buffer.right as *const T, buf_left, 1);
        core::ptr::copy_nonoverlapping(array.as_ptr(), buf_right, N);

        let (shifted, overflow) = buffer.unpack_mandrop();

        core::ptr::copy_nonoverlapping((&shifted as *const ManuallyDrop<[MaybeUninit<T>; N]>).cast::<T>(), array.as_mut_ptr(), N);
        core::mem::forget(shifted);

        ManuallyDrop::into_inner(overflow)
    }
}

pub const fn from_item<T>(item: T) -> [T; 1]
{
    [item]
}

pub const fn reformulate_length<T, const N: usize, const M: usize>(array: [T; N]) -> [T; M]
where
    [(); M - N]:,
    [(); N - M]:
{
    unsafe {private::transmute_unchecked_size(array)}
}
pub const fn reformulate_length_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> &[T; M]
where
    [(); M - N]:,
    [(); N - M]:
{
    unsafe {&*array.as_ptr().cast()}
}
pub const fn reformulate_length_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> &mut [T; M]
where
    [(); M - N]:,
    [(); N - M]:
{
    unsafe {&mut *array.as_mut_ptr().cast()}
}
pub const fn try_reformulate_length<T, const N: usize, const M: usize>(array: [T; N]) -> Result<[T; M], [T; N]>
{
    if N == M
    {
        Ok(unsafe {private::transmute_unchecked_size(array)})
    }
    else
    {
        Err(array)
    }
}
pub const fn try_reformulate_length_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> Option<&[T; M]>
{
    if N == M
    {
        Some(unsafe {&*array.as_ptr().cast()})
    }
    else
    {
        None
    }
}
pub const fn try_reformulate_length_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> Option<&mut [T; M]>
{
    if N == M
    {
        Some(unsafe {&mut *array.as_mut_ptr().cast()})
    }
    else
    {
        None
    }
}

pub const fn into_collumn<T, const N: usize>(array: [T; N]) -> [[T; 1]; N]
{
    unsafe {
        private::transmute_unchecked_size(array)
    }
}
pub const fn as_collumn<T, const N: usize>(array: &[T; N]) -> &[[T; 1]; N]
{
    unsafe {
        &*array.as_ptr().cast()
    }
}
pub const fn as_collumn_mut<T, const N: usize>(array: &mut [T; N]) -> &mut [[T; 1]; N]
{
    unsafe {
        &mut *array.as_mut_ptr().cast()
    }
}

pub const fn chain<T, const N: usize, const M: usize>(array: [T; N], rhs: [T; M]) -> [T; N + M]
{
    unsafe {private::merge_transmute(array, rhs)}
}
pub const fn rchain<T, const N: usize, const M: usize>(array: [T; N], rhs: [T; M]) -> [T; N + M]
{
    unsafe {private::merge_transmute(rhs, array)}
}

pub const fn spread_chunks<T, const N: usize, const M: usize>(array: [T; N]) -> ([[T; N / M]; M], [T; N % M])
where
    [(); M - 1]:,
    [(); N % M]:,
    [(); N / M]:
{
    let split = crate::chunks(array);

    let spread_t = unsafe {core::ptr::read(&split.0 as *const [[T; _]; _])};
    let rest = unsafe {core::ptr::read(&split.1 as *const [T; _])};
    core::mem::forget(split);

    (crate::transpose(spread_t), rest)
}

pub const fn rspread_chunks<T, const N: usize, const M: usize>(array: [T; N]) -> ([T; N % M], [[T; N / M]; M])
where
    [(); M - 1]:,
    [(); N % M]:,
    [(); N / M]:
{
    let split = crate::rchunks(array);
    
    let start = unsafe {core::ptr::read(&split.0 as *const [T; _])};
    let spread_t = unsafe {core::ptr::read(&split.1 as *const [[T; _]; _])};
    core::mem::forget(split);

    (start, crate::transpose(spread_t))
}

pub const fn spread_chunks_exact<T, const N: usize, const M: usize>(array: [T; N]) -> [[T; N / M]; M]
where
    [(); M - 1]:,
    [(); 0 - N % M]:,
    [(); N / M]:
{
    let spread_t: [[T; M]; N / M] = unsafe {
        private::transmute_unchecked_size(array)
    };
    crate::transpose(spread_t)
}

pub const fn chunks<T, const N: usize, const M: usize>(array: [T; N]) -> ([[T; M]; N / M], [T; N % M])
{
    unsafe {private::split_transmute(array)}
}
pub const fn chunks_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> (&[[T; M]; N / M], &[T; N % M])
{
    let (ptr_left, ptr_right) = crate::rsplit_ptr(array, N % M);
    unsafe {(&*ptr_left.cast(), &*ptr_right.cast())}
}
pub const fn chunks_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> (&mut [[T; M]; N / M], &mut [T; N % M])
{
    let (ptr_left, ptr_right) = crate::rsplit_mut_ptr(array, N % M);
    unsafe {(&mut *ptr_left.cast(), &mut *ptr_right.cast())}
}

pub const fn rchunks<T, const N: usize, const M: usize>(array: [T; N]) -> ([T; N % M], [[T; M]; N / M])
{
    unsafe {private::split_transmute(array)}
}
pub const fn rchunks_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> (&[T; N % M], &[[T; M]; N / M])
{
    let (ptr_left, ptr_right) = crate::split_ptr(array, N % M);
    unsafe {(&*ptr_left.cast(), &*ptr_right.cast())}
}
pub const fn rchunks_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> (&mut [T; N % M], &mut [[T; M]; N / M])
{
    let (ptr_left, ptr_right) = crate::split_mut_ptr(array, N % M);
    unsafe {(&mut *ptr_left.cast(), &mut *ptr_right.cast())}
}

pub const fn chunks_exact<T, const N: usize, const M: usize>(array: [T; N]) -> [[T; M]; N / M]
where
    [(); 0 - N % M]:,
    [(); N / M]:
{
    unsafe {private::transmute_unchecked_size(array)}
}
pub const fn chunks_exact_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> &[[T; M]; N / M]
where
    [(); 0 - N % M]:,
    [(); N / M]:
{
    unsafe {&*array.as_ptr().cast()}
}
pub const fn chunks_exact_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> &mut [[T; M]; N / M]
where
    [(); 0 - N % M]:,
    [(); N / M]:
{
    unsafe {&mut *array.as_mut_ptr().cast()}
}

pub const fn array_simd<T, const N: usize, const M: usize>(array: [T; N]) -> ([Simd<T, M>; N / M], [T; N % M])
where
    T: SimdElement,
    LaneCount<M>: SupportedLaneCount
{
    unsafe {private::split_transmute(array)}
}

pub const fn array_rsimd<T, const N: usize, const M: usize>(array: [T; N]) -> ([T; N % M], [Simd<T, M>; N / M])
where
    T: SimdElement,
    LaneCount<M>: SupportedLaneCount
{
    unsafe {private::split_transmute(array)}
}

pub const fn array_simd_exact<T, const N: usize, const M: usize>(array: [T; N]) -> [Simd<T, M>; N / M]
where
    T: SimdElement,
    LaneCount<M>: SupportedLaneCount,
    [(); 0 - N % M]:,
    [(); N / M]:
{
    unsafe {private::transmute_unchecked_size(array)}
}

pub const fn split_array<T, const N: usize, const M: usize>(array: [T; N]) -> ([T; M], [T; N - M])
where
    [(); N - M]:
{
    unsafe {private::split_transmute(array)}
}
pub const fn split_array_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> (&[T; M], &[T; N - M])
where
    [(); N - M]:
{
    let (ptr_left, ptr_right) = crate::split_ptr(array, M);
    unsafe {(&*ptr_left.cast(), &*ptr_right.cast())}
}
pub const fn split_array_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> (&mut [T; M], &mut [T; N - M])
where
    [(); N - M]:
{
    let (ptr_left, ptr_right) = crate::split_mut_ptr(array, M);
    unsafe {(&mut *ptr_left.cast(), &mut *ptr_right.cast())}
}

pub const fn rsplit_array<T, const N: usize, const M: usize>(array: [T; N]) -> ([T; N - M], [T; M])
where
    [(); N - M]:
{
    unsafe {private::split_transmute(array)}
}
pub const fn rsplit_array_ref<T, const N: usize, const M: usize>(array: &[T; N]) -> (&[T; N - M], &[T; M])
where
    [(); N - M]:
{
    let (ptr_left, ptr_right) = crate::rsplit_ptr(array, M);
    unsafe {(&*ptr_left.cast(), &*ptr_right.cast())}
}
pub const fn rsplit_array_mut<T, const N: usize, const M: usize>(array: &mut [T; N]) -> (&mut [T; N - M], &mut [T; M])
where
    [(); N - M]:
{
    let (ptr_left, ptr_right) = crate::rsplit_mut_ptr(array, M);
    unsafe {(&mut *ptr_left.cast(), &mut *ptr_right.cast())}
}

#[test]
fn bench()
{
    use std::time::SystemTime;

    const N: usize = 1 << 10;
    let mut a: [usize; N] = ArrayOps::from_fn(|i| i);
    let t0 = SystemTime::now();
    for _ in 0..1000
    {
        a.bit_rev_permutation();
    }
    let dt = SystemTime::now().duration_since(t0);

    // 8.8810513s
    println!("{:?}", dt);
}

pub const fn digit_rev_permutation<T, const N: usize, const R: usize>(array: &mut [T; N])
where
    [(); is_power_of(N, R) as usize - 1]:
{
    if N <= R
    {
        return;
    }

    let mut i = 1;
    let mut j = N/R + 1;
    while i < N - 1
    {
        if i < j - 1
        {
            unsafe {
                core::ptr::swap_nonoverlapping(array.as_mut_ptr().add(i), array.as_mut_ptr().add(j - 1), 1);
            }
        }
        let mut k = N/R;
        while k*(R - 1) < j
        {
            j -= k*(R - 1);
            k /= R;
        }
        j += k;
        i += 1;
    }
}

pub const fn grey_code_permutation<T, const N: usize>(array: &mut [T; N])
where
    [(); N.is_power_of_two() as usize - 1]:
{
    let mut i = 0;
    while i < N
    {
        let mut j = i ^ (i >> 1);
        while j < i
        {
            j = j ^ (j >> 1);
        }
        if j != i
        {
            unsafe {
                core::ptr::swap_nonoverlapping(array.as_mut_ptr().add(i), array.as_mut_ptr().add(j), 1);
            }
        }
        i += 1;
    }
}

impl<T, const N: usize> /*const*/ ArrayOps<T, N> for [T; N]
{
    //type Array<I, const M: usize> = [I; M];
    
    fn split_len(mid: usize) -> (usize, usize)
    {
        slice_ops::split_len(N, mid)
    }
    fn rsplit_len(mid: usize) -> (usize, usize)
    {
        slice_ops::rsplit_len(N, mid)
    }
    
    fn split_ptr(&self, mid: usize) -> (*const T, *const T)
    {
        crate::split_ptr(self, mid)
    }
    fn split_mut_ptr(&mut self, mid: usize) -> (*mut T, *mut T)
    {
        crate::split_mut_ptr(self, mid)
    }

    fn rsplit_ptr(&self, mid: usize) -> (*const T, *const T)
    {
        crate::rsplit_ptr(self, mid)
    }
    fn rsplit_mut_ptr(&mut self, mid: usize) -> (*mut T, *mut T)
    {
        crate::rsplit_mut_ptr(self, mid)
    }

    fn from_fn<F>(mut fill: F) -> Self
    where
        F: FnMut(usize) -> T + Destruct
    {
        let mut array = MaybeUninit::uninit_array();
        let mut guard = PartialInitGuard::new_left(&mut array);

        while guard.more()
        {
            guard.push_by_fn(&mut fill)
        }

        guard.done();

        unsafe {
            MaybeUninit::array_assume_init(array)
        }
    }
    fn rfrom_fn<F>(mut fill: F) -> Self
    where
        F: FnMut(usize) -> T + Destruct
    {
        let mut array = MaybeUninit::uninit_array();
        let mut guard = PartialInitGuard::new_right(&mut array);

        while guard.more()
        {
            guard.push_by_fn(&mut fill)
        }

        guard.done();

        unsafe {
            MaybeUninit::array_assume_init(array)
        }
    }
    #[cfg(feature = "alloc")]
    fn from_fn_boxed<F>(fill: F) -> Box<Self>
    where
        F: FnMut(usize) -> T + Destruct
    {
        Self::from_fn_boxed_in(fill, Global)
    }
    #[cfg(feature = "alloc")]
    fn rfrom_fn_boxed<F>(fill: F) -> Box<Self>
    where
        F: FnMut(usize) -> T + Destruct
    {
        Self::rfrom_fn_boxed_in(fill, Global)
    }
    #[cfg(feature = "alloc")]
    fn from_fn_boxed_in<F, A>(mut fill: F, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T + Destruct,
        A: Allocator
    {
        let mut array = private::boxed_array::new_uninit_in(alloc);
        let mut guard = PartialInitGuard::new_left(&mut *array);

        while guard.more()
        {
            guard.push_by_fn(&mut fill)
        }

        guard.done();

        unsafe {
            private::boxed_array::assume_init(array)
        }
    }
    #[cfg(feature = "alloc")]
    fn rfrom_fn_boxed_in<F, A>(mut fill: F, alloc: A) -> Box<Self, A>
    where
        F: FnMut(usize) -> T + Destruct,
        A: Allocator
    {
        let mut array = private::boxed_array::new_uninit_in(alloc);
        let mut guard = PartialInitGuard::new_right(&mut *array);

        while guard.more()
        {
            guard.push_by_fn(&mut fill)
        }

        guard.done();

        unsafe {
            private::boxed_array::assume_init(array)
        }
    }
    
    fn try_from_fn<F, E>(mut fill: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        let mut array = MaybeUninit::uninit_array();
        let mut guard = PartialInitGuard::new_left(&mut array);

        while guard.more()
        {
            guard.try_push_by_fn(&mut fill)?
        }

        guard.done();

        unsafe {
            Ok(MaybeUninit::array_assume_init(array))
        }
    }
    fn try_rfrom_fn<F, E>(mut fill: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        let mut array = MaybeUninit::uninit_array();
        let mut guard = PartialInitGuard::new_right(&mut array);

        while guard.more()
        {
            guard.try_push_by_fn(&mut fill)?
        }

        guard.done();

        unsafe {
            Ok(MaybeUninit::array_assume_init(array))
        }
    }
    #[cfg(feature = "alloc")]
    fn try_from_fn_boxed<F, E>(fill: F) -> Result<Box<Self>, E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        Self::try_from_fn_boxed_in(fill, Global)
    }
    #[cfg(feature = "alloc")]
    fn try_rfrom_fn_boxed<F, E>(fill: F) -> Result<Box<Self>, E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        Self::try_rfrom_fn_boxed_in(fill, Global)
    }
    #[cfg(feature = "alloc")]
    fn try_from_fn_boxed_in<F, E, A>(mut fill: F, alloc: A) -> Result<Box<Self, A>, E>
    where
        F: FnMut(usize) -> Result<T, E>,
        A: Allocator
    {
        let mut array = private::boxed_array::new_uninit_in(alloc);
        let mut guard = PartialInitGuard::new_left(&mut *array);

        while guard.more()
        {
            guard.try_push_by_fn(&mut fill)?
        }

        guard.done();

        unsafe {
            Ok(private::boxed_array::assume_init(array))
        }
    }
    #[cfg(feature = "alloc")]
    fn try_rfrom_fn_boxed_in<F, E, A>(mut fill: F, alloc: A) -> Result<Box<Self, A>, E>
    where
        F: FnMut(usize) -> Result<T, E>,
        A: Allocator
    {
        let mut array = private::boxed_array::new_uninit_in(alloc);
        let mut guard = PartialInitGuard::new_right(&mut *array);

        while guard.more()
        {
            guard.try_push_by_fn(&mut fill)?
        }

        guard.done();

        unsafe {
            Ok(private::boxed_array::assume_init(array))
        }
    }

    async fn from_fn_async<F>(fill: F) -> Self
    where
        F: AsyncFn(usize) -> T
    {
        Runs::new(ArrayOps::from_fn(|i| fill(i))).await
    }
    async fn try_from_fn_async<F, E>(fill: F) -> Result<Self, E>
    where
        F: AsyncFn(usize) -> Result<T, E>
    {
        TryRuns::new(ArrayOps::from_fn(|i| fill(i))).await
    }
    
    fn truncate<const M: usize>(self) -> [T; M]
    where
        T: Destruct,
        [(); N - M]:
    {
        crate::split_array(self).0
    }
    fn rtruncate<const M: usize>(self) -> [T; M]
    where
        T: Destruct,
        [(); N - M]:
    {
        crate::rsplit_array(self).1
    }
    
    fn truncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:
    {
        crate::truncate_ref(self)
    }
    fn rtruncate_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); N - M]:
    {
        crate::rtruncate_ref(self)
    }
        
    fn truncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:
    {
        crate::truncate_mut(self)
    }
    fn rtruncate_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); N - M]:
    {
        crate::rtruncate_mut(self)
    }

    fn resize<const M: usize, F>(mut self, mut fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + Destruct,
        T: Destruct
    {
        let overlap = N.min(M);

        if M < N
        {
            // Drop truncated elements
            unsafe {
                core::ptr::drop_in_place(&mut self[M..N]);
            }
        }

        let src = self.as_ptr();

        if M <= N
        {
            // If not larger than original, dont make a new uninit, instead read directly from original
            let array = unsafe {
                core::ptr::read(src.cast())
            };
            core::mem::forget(self);
            return array;
        }
    
        // Make new uninit array
        let mut array = MaybeUninit::uninit_array();
        let mut dst = (&mut array as *mut MaybeUninit<T>).cast::<T>();
    
        // Copy over
        unsafe {core::ptr::copy_nonoverlapping(src, dst, overlap)};
        core::mem::forget(self);
    
        // Extend with fill
        let mut i = N;
        dst = unsafe {dst.add(N)};
        while i < M
        {
            unsafe {core::ptr::write(dst, fill(i))};
            i += 1;
            dst = unsafe {dst.add(1)};
        }
        unsafe {
            MaybeUninit::array_assume_init(array)
        }
    }
    fn rresize<const M: usize, F>(mut self, mut fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + Destruct,
        T: Destruct
    {
        let trunc = N.saturating_sub(M);
        let offset = M.saturating_sub(N);
        let overlap = N.min(M);

        if M < N
        {
            // Drop truncated elements
            unsafe {
                core::ptr::drop_in_place(&mut self[0..trunc]);
            }
        }

        let src = unsafe {
            self.as_ptr().add(trunc)
        };

        if M <= N
        {
            // If not larger than original, dont make a new uninit, instead read directly from original
            let array = unsafe {
                core::ptr::read(src.cast())
            };
            core::mem::forget(self);
            return array;
        }
        
        // Make new uninit array
        let mut array = MaybeUninit::uninit_array();
        let mut dst = unsafe {
            (&mut array as *mut MaybeUninit<T>).cast::<T>().add(offset)
        };
    
        // Copy over
        unsafe {core::ptr::copy_nonoverlapping(src, dst, overlap)};
        core::mem::forget(self);
    
        // Extend with fill
        let mut i = offset;
        while i > 0
        {
            i -= 1;
            dst = unsafe {dst.sub(1)};
            unsafe {core::ptr::write(dst, fill(i))};
        }
    
        unsafe {
            MaybeUninit::array_assume_init(array)
        }
    }

    fn into_rotate_left(self, n: usize) -> Self
    {
        crate::into_rotate_left(self, n)
    }
    
    fn into_rotate_right(self, n: usize) -> Self
    {
        crate::into_rotate_right(self, n)
    }

    fn into_shift_many_left<const M: usize>(self, items: [T; M]) -> ([T; M], Self)
    {
        crate::into_shift_many_left(self, items)
    }

    fn into_shift_many_right<const M: usize>(self, items: [T; M]) -> (Self, [T; M])
    {
        crate::into_shift_many_right(self, items)
    }

    fn into_shift_left(self, item: T) -> (T, Self)
    {
        crate::into_shift_left(self, item)
    }

    fn into_shift_right(self, item: T) -> (Self, T)
    {
        crate::into_shift_right(self, item)
    }

    fn rotate_left2(&mut self, n: usize)
    {
        crate::rotate_left(self, n)
    }

    fn rotate_right2(&mut self, n: usize)
    {
        crate::rotate_right(self, n)
    }

    fn shift_many_left<const M: usize>(&mut self, items: [T; M]) -> [T; M]
    {
        crate::shift_many_left(self, items)
    }

    fn shift_many_right<const M: usize>(&mut self, items: [T; M]) -> [T; M]
    {
        crate::shift_many_right(self, items)
    }
    
    fn shift_left(&mut self, item: T) -> T
    {
        crate::shift_left(self, item)
    }

    fn shift_right(&mut self, item: T) -> T
    {
        crate::shift_right(self, item)
    }
    
    fn extend<const M: usize, F>(self, mut fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + Destruct,
        [(); M - N]:
    {
        let filled: [T; M - N] = ArrayOps::from_fn(|i| fill(i + N));
        unsafe {private::merge_transmute(self, filled)}
    }
    fn rextend<const M: usize, F>(self, fill: F) -> [T; M]
    where
        F: FnMut(usize) -> T + Destruct,
        [(); M - N]:
    {
        let filled: [T; M - N] = ArrayOps::rfrom_fn(fill);
        unsafe {private::merge_transmute(filled, self)}
    }
    
    fn reformulate_length<const M: usize>(self) -> [T; M]
    where
        [(); M - N]:,
        [(); N - M]:
    {
        crate::reformulate_length(self)
    }
    
    fn reformulate_length_ref<const M: usize>(&self) -> &[T; M]
    where
        [(); M - N]:,
        [(); N - M]:
    {
        crate::reformulate_length_ref(self)
    }
        
    fn reformulate_length_mut<const M: usize>(&mut self) -> &mut [T; M]
    where
        [(); M - N]:,
        [(); N - M]:
    {
        crate::reformulate_length_mut(self)
    }
    
    fn try_reformulate_length<const M: usize>(self) -> Result<[T; M], Self>
    {
        crate::try_reformulate_length(self)
    }
    
    fn try_reformulate_length_ref<const M: usize>(&self) -> Option<&[T; M]>
    {
        crate::try_reformulate_length_ref(self)
    }
        
    fn try_reformulate_length_mut<const M: usize>(&mut self) -> Option<&mut [T; M]>
    {
        crate::try_reformulate_length_mut(self)
    }
    
    fn into_collumn(self) -> [[T; 1]; N]
    {
        crate::into_collumn(self)
    }
    fn as_collumn(&self) -> &[[T; 1]; N]
    {
        crate::as_collumn(self)
    }
    fn as_collumn_mut(&mut self) -> &mut [[T; 1]; N]
    {
        crate::as_collumn_mut(self)
    }
    
    fn map<Map>(self, mut mapper: Map) -> [Map::Output; N]
    where
        Map: FnMut<(T,)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialMapGuard::new_left(
            self,
            &mut dst
        );

        while guard.more()
        {
            guard.map(&mut mapper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    fn map_ref<'a, Map>(&'a self, mut map: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a T,)>
    {
        ArrayOps::from_fn(|i| map(&self[i]))
    }
    fn map_mut<'a, Map>(&'a mut self, mut map: Map) -> [Map::Output; N]
    where
        Map: FnMut<(&'a mut T,)>
    {
        ArrayOps::from_fn(|i| unsafe {
            map((&mut self[i] as *mut T).as_mut_unchecked())
        })
    }
    fn map_outer<Map>(&self, map: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(T, T)> + Destruct,
        T: Copy
    {
        self.zip_outer_with(self, map)
    }
    fn map_outer_ref<'a, Map>(&'a self, map: Map) -> [[Map::Output; N]; N]
    where
        Map: FnMut<(&'a T, &'a T)>
    {
        self.zip_outer_ref_with(&self, map)
    }
    fn zip_with<Zip, Rhs>(self, rhs: Rhs, mut zipper: Zip) -> [Zip::Output; N]
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut<(T, Rhs::Elem)> + Destruct
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        while guard.more()
        {
            guard.zip(&mut zipper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    fn zip_ref_with<'a, Map, Rhs>(&'a self, rhs: Rhs, mut zipper: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: FnMut<(&'a T, Rhs::Elem)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        while guard.more()
        {
            guard.zip(&mut zipper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    fn zip_mut_with<'a, Map, Rhs>(&'a mut self, rhs: Rhs, mut zipper: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: FnMut<(&'a mut T, Rhs::Elem)>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        while guard.more()
        {
            guard.zip(&mut zipper)
        }
        guard.done();
    
        unsafe {
            MaybeUninit::array_assume_init(dst)
        }
    }
    fn zip_outer_with<Map, Rhs, const M: usize>(&self, rhs: &Rhs, mut map: Map) -> [[Map::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: FnMut<(T, Rhs::Elem)>,
        T: Copy
    {
        ArrayOps::from_fn(|x| ArrayOps::from_fn(|y| map(self[x], rhs.copy_elem(y))))
    }
    fn zip_outer_ref_with<'a, Map, Rhs, const M: usize>(&'a self, rhs: &Rhs, mut map: Map) -> [[Map::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: FnMut<(&'a T, Rhs::Elem)>
    {
        ArrayOps::from_fn(|x| ArrayOps::from_fn(|y| map(&self[x], rhs.copy_elem(y))))
    }
    fn flatmap<Map, O, const M: usize>(self, map: Map) -> [O; N*M]
    where
        Map: FnMut<(T,), Output = [O; M]> + Destruct,
        [(); N*M]:
    {
        self.map(map).flatten()
    }
    fn flatmap_ref<'a, Map, O, const M: usize>(&'a self, map: Map) -> [O; N*M]
    where
        Map: FnMut<(&'a T,), Output = [O; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.map_ref(map).flatten()
    }
    fn flatmap_mut<'a, Map, O, const M: usize>(&'a mut self, map: Map) -> [O; N*M]
    where
        Map: FnMut<(&'a mut T,), Output = [O; M]>,
        T: 'a,
        [(); N*M]:
    {
        self.map_mut(map).flatten()
    }
    fn map_assign<Map>(&mut self, mut map: Map)
    where
        Map: FnMut(T) -> T
    {
        self.visit_mut(|x| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, map(value))
        })
    }
    fn zip_assign_with<Rhs, Zip>(&mut self, rhs: Rhs, mut zip: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> T
    {
        self.visit_mut_with(rhs, |x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(value, y))
        });
    }
    
    async fn map_async<Map>(self, map: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(T,)>
    {
        Runs::new(self.map(|x| map(x))).await
    }
    async fn map_ref_async<'a, Map>(&'a self, map: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(&'a T,)>,
        T: 'a
    {
        Runs::new(self.map_ref(|x| map(x))).await
    }
    async fn map_mut_async<'a, Map>(&'a mut self, map: Map) -> [Map::Output; N]
    where
        Map: AsyncFn<(&'a mut T,)>,
        T: 'a
    {
        Runs::new(self.map_mut(|x| map(x))).await
    }
    async fn map_outer_async<Map>(&self, map: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(T, T)>,
        T: Copy
    {
        Runs2D::new(self.map_outer(|x, y| map(x, y))).await
    }
    async fn map_outer_ref_async<'a, Map>(&'a self, map: Map) -> [[Map::Output; N]; N]
    where
        Map: AsyncFn<(&'a T, &'a T)>,
        T: 'a
    {
        Runs2D::new(self.map_outer_ref(|x, y| map(x, y))).await
    }
    async fn zip_async_with<Map, Rhs>(self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn<(T, Rhs::Elem)>
    {
        Runs::new(self.zip_with(rhs, |x, y| map(x, y))).await
    }
    async fn zip_ref_async_with<'a, Map, Rhs>(&'a self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn<(&'a T, Rhs::Elem)>,
        T: 'a
    {
        Runs::new(self.zip_ref_with(rhs, |x, y| map(x, y))).await
    }
    async fn zip_mut_async_with<'a, Map, Rhs>(&'a mut self, rhs: Rhs, map: Map) -> [Map::Output; N]
    where
        Rhs: ArrayForm<N>,
        Map: AsyncFn<(&'a mut T, Rhs::Elem)>,
        T: 'a
    {
        Runs::new(self.zip_mut_with(rhs, |x, y| map(x, y))).await
    }
    async fn zip_outer_async_with<Map, Rhs, const M: usize>(&self, rhs: &Rhs, map: Map) -> [[Map::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: AsyncFn<(T, Rhs::Elem)>,
        T: Copy
    {
        Runs2D::new(self.zip_outer_with(rhs, |x, y| map(x, y))).await
    }
    async fn zip_outer_ref_async_with<'a, Map, Rhs, const M: usize>(&'a self, rhs: &Rhs, map: Map) -> [[Map::Output; M]; N]
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: AsyncFn<(&'a T, Rhs::Elem)>,
        T: 'a
    {
        Runs2D::new(self.zip_outer_ref_with(rhs, |x, y| map(x, y))).await
    }
    async fn flatmap_async<Map, O, const M: usize>(self, map: Map) -> [O; N*M]
    where
        Map: AsyncFn(T) -> [O; M],
        [(); N*M]:
    {
        self.map_async(map)
            .await
            .flatten()
    }
    async fn flatmap_ref_async<'a, Map, O, const M: usize>(&'a self, map: Map) -> [O; N*M]
    where
        Map: AsyncFn(&'a T) -> [O; M],
        T: 'a,
        [(); N*M]:
    {
        self.map_ref_async(map)
            .await
            .flatten()
    }
    async fn flatmap_mut_async<'a, Map, O, const M: usize>(&'a mut self, map: Map) -> [O; N*M]
    where
        Map: AsyncFn(&'a mut T) -> [O; M],
        T: 'a,
        [(); N*M]:
    {
        self.map_mut_async(map)
            .await
            .flatten()
    }
    async fn map_assign_async<Map>(&mut self, map: Map)
    where
        Map: AsyncFn(T) -> T + Destruct
    {
        self.visit_mut_async(async |x| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, map(value).await)
        }).await
    }
    async fn zip_assign_async_with<Rhs, Zip>(&mut self, rhs: Rhs, zip: Zip)
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> T
    {
        self.visit_mut_async_with(rhs, async |x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(value, y).await)
        }).await
    }
    
    fn try_map<Map, U, E>(self, mut mapper: Map) -> Result<[U; N], E>
    where
        Map: FnMut(T) -> Result<U, E>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialMapGuard::new_left(
            self,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_map(&mut mapper)
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
    fn try_map_ref<'a, Map, U, E>(&'a self, mut map: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a T) -> Result<U, E>,
        T: 'a
    {
        ArrayOps::try_from_fn(|i| map(&self[i]))
    }
    fn try_map_mut<'a, Map, U, E>(&'a mut self, mut map: Map) -> Result<[U; N], E>
    where
        Map: FnMut(&'a mut T) -> Result<U, E>,
        T: 'a
    {
        ArrayOps::try_from_fn(|i| unsafe {
            map((&mut self[i] as *mut T).as_mut_unchecked())
        })
    }
    fn try_map_outer<Map, U, E>(&self, map: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(T, T) -> Result<U, E>,
        T: Copy
    {
        self.try_zip_outer_with(self, map)
    }
    fn try_map_outer_ref<'a, Map, U, E>(&'a self, map: Map) -> Result<[[U; N]; N], E>
    where
        Map: FnMut(&'a T, &'a T) -> Result<U, E>,
        T: 'a
    {
        self.try_zip_outer_ref_with(&self, map)
    }
    fn try_zip_with<Zip, Rhs, U, E>(self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> Result<U, E>
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_zip(&mut zipper)
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
    fn try_zip_ref_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(&'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_zip(&mut zipper)
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
    fn try_zip_mut_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, mut zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(&'a mut T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        let mut dst = MaybeUninit::uninit_array();
        let mut guard = PartialZipGuard::new_left(
            self,
            rhs,
            &mut dst
        );

        let mut result = Ok(());

        while guard.more()
        {
            if let Err(error) = guard.try_zip(&mut zipper)
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
    fn try_zip_outer_with<Map, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, mut map: Map) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: FnMut(T, Rhs::Elem) -> Result<U, E>,
        T: Copy
    {
        ArrayOps::try_from_fn(|x| ArrayOps::try_from_fn(|y| map(self[x], rhs.copy_elem(y))))
    }
    fn try_zip_outer_ref_with<'a, Map, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, mut map: Map) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Map: FnMut(&'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        ArrayOps::try_from_fn(|x| ArrayOps::try_from_fn(|y| map(&self[x], rhs.copy_elem(y))))
    }
    fn try_flatmap<Map, U, E, const M: usize>(self, map: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(T) -> Result<[U; M], E>
    {
        Ok(self.try_map(map)?.flatten())
    }
    fn try_flatmap_ref<'a, Map, U, E, const M: usize>(&'a self, map: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a T) -> Result<[U; M], E>,
        T: 'a
    {
        Ok(self.try_map_ref(map)?.flatten())
    }
    fn try_flatmap_mut<'a, Map, U, E, const M: usize>(&'a mut self, map: Map) -> Result<[U; N*M], E>
    where
        Map: FnMut(&'a mut T) -> Result<[U; M], E>,
        T: 'a
    {
        Ok(self.try_map_mut(map)?.flatten())
    }
    fn try_map_assign<Map, E>(&mut self, mut map: Map) -> Result<(), E>
    where
        Map: FnMut(T) -> Result<T, E>
    {
        self.try_visit_mut(|x| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, map(value)?);
            Ok(())
        })
    }
    fn try_zip_assign_with<Rhs, Zip, E>(&mut self, rhs: Rhs, mut zip: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: FnMut(T, Rhs::Elem) -> Result<T, E>
    {
        self.try_visit_mut_with(rhs, |x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(value, y)?);
            Ok(())
        })
    }
    
    async fn try_map_async<Map, U, E>(self, map: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(T) -> Result<U, E>
    {
        TryRuns::new(self.map(|x| map(x))).await
    }
    async fn try_map_ref_async<'a, Map, U, E>(&'a self, map: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(&'a T) -> Result<U, E>,
        T: 'a
    {
        TryRuns::new(self.map_ref(|x| map(x))).await
    }
    async fn try_map_mut_async<'a, Map, U, E>(&'a mut self, map: Map) -> Result<[U; N], E>
    where
        Map: AsyncFn(&'a mut T) -> Result<U, E>,
        T: 'a
    {
        TryRuns::new(self.map_mut(|x| map(x))).await
    }
    async fn try_map_outer_async<Map, U, E>(&self, mut map: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(T, T) -> Result<U, E>,
        T: Copy
    {
        TryRuns2D::new(self.map_outer(|x, y| map(x, y))).await
    }
    async fn try_map_outer_ref_async<'a, Map, U, E>(&'a self, map: Map) -> Result<[[U; N]; N], E>
    where
        Map: AsyncFn(&'a T, &'a T) -> Result<U, E>,
        T: 'a
    {
        TryRuns2D::new(self.map_outer_ref(|x, y| map(x, y))).await
    }
    async fn try_zip_async_with<Zip, Rhs, U, E>(self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<U, E>
    {
        TryRuns::new(self.zip_with(rhs, |x, y| zipper(x, y))).await
    }
    async fn try_zip_ref_async_with<'a, Zip, Rhs, U, E>(&'a self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(&'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        TryRuns::new(self.zip_ref_with(rhs, |x, y| zipper(x, y))).await
    }
    async fn try_zip_mut_async_with<'a, Zip, Rhs, U, E>(&'a mut self, rhs: Rhs, zipper: Zip) -> Result<[U; N], E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(&'a mut T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        TryRuns::new(self.zip_mut_with(rhs, |x, y| zipper(x, y))).await
    }
    async fn try_zip_outer_async_with<Zip, Rhs, U, E, const M: usize>(&self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<U, E>,
        T: Copy
    {
        TryRuns2D::new(self.zip_outer_with(rhs, |x, y| zipper(x, y))).await
    }
    async fn try_zip_outer_ref_async_with<'a, Zip, Rhs, U, E, const M: usize>(&'a self, rhs: &Rhs, zipper: Zip) -> Result<[[U; M]; N], E>
    where
        Rhs: ArrayForm<M, Elem: Copy>,
        Zip: AsyncFn(&'a T, Rhs::Elem) -> Result<U, E>,
        T: 'a
    {
        TryRuns2D::new(self.zip_outer_ref_with(rhs, |x, y| zipper(x, y))).await
    }
    async fn try_flatmap_async<Map, U, E, const M: usize>(self, map: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(T) -> Result<[U; M], E>,
        [(); N*M]:
    {
        Ok(self.try_map_async(map).await?.flatten())
    }
    async fn try_flatmap_ref_async<'a, Map, U, E, const M: usize>(&'a self, map: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(&'a T) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        Ok(self.try_map_ref_async(map).await?.flatten())
    }
    async fn try_flatmap_mut_async<'a, Map, U, E, const M: usize>(&'a mut self, map: Map) -> Result<[U; N*M], E>
    where
        Map: AsyncFn(&'a mut T) -> Result<[U; M], E>,
        T: 'a,
        [(); N*M]:
    {
        Ok(self.try_map_mut_async(map).await?.flatten())
    }
    async fn try_map_assign_async<Map, E>(&mut self, map: Map) -> Result<(), E>
    where
        Map: AsyncFn(T) -> Result<T, E>
    {
        self.try_visit_mut_async(async |x| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, map(value).await?);
            Ok(())
        }).await
    }
    async fn try_zip_assign_async_with<Rhs, Zip, E>(&mut self, rhs: Rhs, zip: Zip) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        Zip: AsyncFn(T, Rhs::Elem) -> Result<T, E>
    {
        self.try_visit_mut_async_with(rhs, async |x, y| unsafe {
            let value = core::ptr::read(x);
            core::ptr::write(x, zip(value, y).await?);
            Ok(())
        }).await
    }
    
    fn zip<Z>(self, other: Z) -> [(T, Z::Elem); N]
    where
        Z: ArrayForm<N>
    {
        self.zip_with(other, const |x, y| (x, y))
    }
    fn zip_ref<Z>(&self, other: Z) -> [(&T, Z::Elem); N]
    where
        Z: ArrayForm<N>
    {
        self.zip_ref_with(other, const |x, y| (x, y))
    }
    fn zip_mut<Z>(&mut self, other: Z) -> [(&mut T, Z::Elem); N]
    where
        Z: ArrayForm<N>
    {
        self.zip_mut_with(other, const |x, y| (x, y))
    }
    fn zip_outer<Z, const M: usize>(&self, other: &Z) -> [[(T, Z::Elem); M]; N]
    where
        T: Copy,
        Z: ArrayForm<M, Elem: Copy>
    {
        self.zip_outer_with(other, const |x, y| (x, y))
    }
    fn zip_outer_ref<Z, const M: usize>(&self, other: &Z) -> [[(&T, Z::Elem); M]; N]
    where
        Z: ArrayForm<M, Elem: Copy>
    {
        self.zip_outer_ref_with(other, const |x, y| (x, y))
    }
    
    fn enumerate(self) -> [(usize, T); N]
    {
        let ptr = &self as *const T;
    
        let dst = ArrayOps::from_fn(|i| unsafe {
            (i, ptr.add(i).read())
        });
    
        core::mem::forget(self);
    
        dst
    }
    
    fn diagonal<const H: usize, const W: usize>(self) -> [[T; W]; H]
    where
        T: Default,
        [(); H - N]:,
        [(); W - N]:
    {
        let ptr = self.as_ptr();
        
        let dst = ArrayOps::from_fn(|i| ArrayOps::from_fn(|j| if i == j && i < N
            {
                unsafe {
                    ptr.add(i).read()
                }
            }
            else
            {
                T::default()
            }
        ));
    
        core::mem::forget(self);
    
        dst
    }
    
    fn toeplitz_matrix(&self) -> [[T; N]; N]
    where
        T: Copy
    {
        ArrayOps::from_fn(|i| ArrayOps::from_fn(|j| self[if i >= j {i - j} else {j - i}]))
    }
    fn hankel_matrix<const M: usize>(&self, r: &[T; M]) -> [[T; M]; N]
    where
        T: Copy
    {
        ArrayOps::from_fn(|i| ArrayOps::from_fn(|j| if i + j < N
        {
            self[i + j]
        }
        else
        {
            r[i + j + 1 - N]
        }))
    }

    fn differentiate(&mut self)
    where
        T: SubAssign<T> + Copy + Destruct
    {
        if N > 0
        {
            let mut i = N - 1;
            while i > 0
            {
                self[i] -= self[i - 1];
                i -= 1;
            }
        }
    }

    fn integrate(&mut self)
    where
        T: AddAssign<T> + Copy + Destruct
    {
        let mut i = 1;
        while i < N
        {
            self[i] += self[i - 1];
            i += 1;
        }
    }
    
    fn find(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        self.as_slice().find(x)
    }
    fn find_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a
    {
        self.as_slice().find_by(f)
    }
    fn find_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a
    {
        self.as_slice().find_by_key(b, f)
    }
        
    fn rfind(&self, x: &T) -> Option<usize>
    where
        T: PartialEq
    {
        self.as_slice().rfind(x)
    }
    fn rfind_by<'a, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool /*+ ~const Destruct*/,
        T: 'a
    {
        self.as_slice().rfind_by(f)
    }
    fn rfind_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        B: PartialEq,
        T: 'a
    {
        self.as_slice().rfind_by_key(b, f)
    }
    
    fn reduce<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
    fn reduce_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T
    {
        PartialEmptyGuard::new_left(self).reduce(reduce)
    }
        
    fn fold<F, O>(self, default: O, fold: F) -> O
    where
        F: FnMut(O, T) -> O
    {
        PartialEmptyGuard::new_left(self).fold(default, fold)
    }
    fn fold_ref<'a, F, O>(&'a self, default: O, fold: F) -> O
    where
        F: FnMut(O, &'a T) -> O,
        T: 'a
    {
        PartialEmptyGuard::new_left(self).fold(default, fold)
    }
    fn fold_mut<'a, F, O>(&'a mut self, default: O, fold: F) -> O
    where
        F: FnMut(O, &'a mut T) -> O,
        T: 'a
    {
        PartialEmptyGuard::new_left(self).fold(default, fold)
    }
        
    fn divide_and_conquer<F>(self, reduce: F) -> Option<T>
    where
        F: FnMut(T, T) -> T
    {
        PartialDivideAndConquerGuard::new_left(self).reduce(reduce)
    }
    fn divide_and_conquer_ref<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: FnMut(&'a T, &'a T) -> &'a T
    {
        PartialDivideAndConquerGuard::new_left(self).reduce(reduce)
    }
    fn divide_and_conquer_mut<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: FnMut(&'a mut T, &'a mut T) -> &'a mut T
    {
        PartialDivideAndConquerGuard::new_left(self).reduce(reduce)
    }
        
    async fn divide_and_conquer_async<F>(self, reduce: F) -> Option<T>
    where
        F: AsyncFn(T, T) -> T
    {
        DivideAndConquer::new(self, |x, y| reduce(x, y)).await
    }
    async fn divide_and_conquer_ref_async<'a, F>(&'a self, reduce: F) -> Option<&'a T>
    where
        F: AsyncFn(&'a T, &'a T) -> &'a T,
        T: 'a
    {
        DivideAndConquer::new(self.each_ref(), |x, y| reduce(x, y)).await
    }
    async fn divide_and_conquer_mut_async<'a, F>(&'a mut self, reduce: F) -> Option<&'a mut T>
    where
        F: AsyncFn(&'a mut T, &'a mut T) -> &'a mut T,
        T: 'a
    {
        DivideAndConquer::new(self.each_mut(), |x, y| reduce(x, y)).await
    }
    
    fn try_sum(self) -> Option<T>
    where
        T: AddAssign
    {
        self.reduce(|mut x, y| {
            x += y;
            x
        })
    }
    fn sum_from<S>(self, from: S) -> S
    where
        S: AddAssign<T>
    {
        self.fold(from, |mut x, y| {
            x += y;
            x
        })
    }
    async fn try_sum_async(self) -> Option<T>
    where
        T: AddAssign
    {
        self.divide_and_conquer_async(async |mut x, y| {
            x += y;
            x
        }).await
    }
        
    fn try_product(self) -> Option<T>
    where
        T: MulAssign
    {
        self.reduce(|mut x, y| {
            x *= y;
            x
        })
    }
    fn product_from<P>(self, from: P) -> P
    where
        P: MulAssign<T>
    {
        self.fold(from, |mut x, y| {
            x *= y;
            x
        })
    }
    async fn try_product_async(self) -> Option<T>
    where
        T: MulAssign
    {
        self.divide_and_conquer_async(async |mut x, y| {
            x *= y;
            x
        }).await
    }
    
    fn max(self) -> Option<T>
    where
        T: Ord
    {
        self.reduce(T::max)
    }
    fn min(self) -> Option<T>
    where
        T: Ord
    {
        self.reduce(T::min)
    }
    async fn max_async(self) -> Option<T>
    where
        T: Ord
    {
        self.divide_and_conquer_async(async |x, y| x.max(y)).await
    }
    async fn min_async(self) -> Option<T>
    where
        T: Ord
    {
        self.divide_and_conquer_async(async |x, y| x.min(y)).await
    }
    
    fn first_max(self) -> Option<T>
    where
        T: PartialOrd<T>
    {
        self.reduce(|a, b| if a >= b {a} else {b})
    }
    fn first_min(self) -> Option<T>
    where
        T: PartialOrd<T>
    {
        self.reduce(|a, b| if a <= b {a} else {b})
    }
    
    fn argreduce<'a, F>(&'a self, mut reduction: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> bool /*+ ~const Destruct*/,
        T: 'a
    {
        if N == 0
        {
            return None;
        }
        let mut i = 1;
        let mut j = 0;
        while i < N
        {
            if reduction(&self[i], &self[j])
            {
                j = i;
            }
            i += 1;
        }
        Some(j)
    }
    fn argreduce_key<'a, B, FR, FB>(&'a self, mut reduction: FR, mut hasher: FB) -> Option<usize>
    where
        FR: FnMut(&B, &B) -> bool /*+ ~const Destruct*/,
        FB: FnMut(&'a T) -> B /*+ ~const Destruct*/,
        T: 'a
    {
        if N == 0
        {
            return None;
        }
        let mut j = 0;
        let mut i = 1;
        let mut key = hasher(&self[j]);
        while i < N
        {
            let next_key = hasher(&self[i]);
            if reduction(&next_key, &key)
            {
                j = i;
                key = next_key;
            }
            i += 1;
        }
        Some(j)
    }
    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd<T>
    {
        self.argreduce(PartialOrd::gt)
    }
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd<T>
    {
        self.argreduce(PartialOrd::lt)
    }
    fn argmax_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering,
        T: 'a
    {
        self.argreduce(|a, b| matches!(f(a, b), Ordering::Greater))
    }
    fn argmin_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&'a T, &'a T) -> Ordering,
        T: 'a
    {
        self.argreduce(|a, b| matches!(f(a, b), Ordering::Less))
    }
    fn argmax_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B,
        B: PartialOrd,
        T: 'a
    {
        self.argreduce_key(PartialOrd::gt, f)
    }
    fn argmin_by_key<'a, B, F>(&'a self, f: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> B,
        B: PartialOrd,
        T: 'a
    {
        self.argreduce_key(PartialOrd::lt, f)
    }
    
    fn visit<'a, F>(&'a self, mut visitor: F)
    where
        F: FnMut(&'a T),
        T: 'a
    {
        let mut i = 0;
        while i < N
        {
            visitor(&self[i]);
            i += 1;
        }
    }
    fn visit_mut<'a, F>(&'a mut self, mut visitor: F)
    where
        F: FnMut(&'a mut T),
        T: 'a
    {
        let mut i = 0;
        while i < N
        {
            visitor(unsafe {
                core::mem::transmute::<&mut T, &mut T>(&mut self[i])
            });
            i += 1;
        }
    }
    fn try_visit<'a, E, F>(&'a self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E>,
        T: 'a
    {
        let mut i = 0;
        while i < N
        {
            visitor(&self[i])?;
            i += 1;
        }
        Ok(())
    }
    fn try_visit_mut<'a, E, F>(&'a mut self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        let mut i = 0;
        while i < N
        {
            visitor(unsafe {
                core::mem::transmute::<&mut T, &mut T>(&mut self[i])
            })?;
            i += 1;
        }
        Ok(())
    }
        
    fn rvisit<'a, F>(&'a self, mut visitor: F)
    where
        F: FnMut(&'a T),
        T: 'a
    {
        let mut i = N;
        while i > 0
        {
            i -= 1;
            visitor(&self[i]);
        }
    }
    fn rvisit_mut<'a, F>(&'a mut self, mut visitor: F)
    where
        F: FnMut(&'a mut T),
        T: 'a
    {
        let mut i = N;
        while i > 0
        {
            i -= 1;
            visitor(unsafe {
                core::mem::transmute::<&mut T, &mut T>(&mut self[i])
            });
        }
    }
    fn try_rvisit<'a, E, F>(&'a self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a T) -> Result<(), E>,
        T: 'a
    {
        let mut i = N;
        while i > 0
        {
            i -= 1;
            visitor(&self[i])?;
        }
        Ok(())
    }
    fn try_rvisit_mut<'a, E, F>(&'a mut self, mut visitor: F) -> Result<(), E>
    where
        F: FnMut(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        let mut i = N;
        while i > 0
        {
            i -= 1;
            visitor(unsafe {
                core::mem::transmute::<&mut T, &mut T>(&mut self[i])
            })?;
        }
        Ok(())
    }
    
    async fn visit_async<'a, F>(&'a self, visitor: F)
    where
        F: AsyncFn(&'a T),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        Actions::new(self.map_ref(|x| visitor(x))).await
    }
    async fn visit_mut_async<'a, F>(&'a mut self, visitor: F)
    where
        F: AsyncFn(&'a mut T),
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        Actions::new(self.map_mut(|x| visitor(x))).await
    }
    async fn try_visit_async<'a, E, F>(&'a self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a T) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        TryActions::new(self.map_ref(|x| visitor(x))).await
    }
    async fn try_visit_mut_async<'a, E, F>(&'a mut self, visitor: F) -> Result<(), E>
    where
        F: AsyncFn(&'a mut T) -> Result<(), E>,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        TryActions::new(self.map_mut(|x| visitor(x))).await
    }

    fn visit_with<'a, F, Rhs>(&'a self, with: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a
    {
        let mut i = 0;
        while i < N
        {
            visitor(
                &self[i],
                unsafe {
                    with.read_elem(i)
                }
            );
            i += 1;
        }
        core::mem::forget(with)
    }
    fn visit_mut_with<'a, F, Rhs>(&'a mut self, with: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a
    {
        let mut i = 0;
        while i < N
        {
            unsafe {
                visitor(
                    (&mut self[i] as *mut T).as_mut_unchecked(),
                    with.read_elem(i)
                );
            }
            i += 1;
        }
        core::mem::forget(with)
    }
    fn try_visit_with<'a, E, F, Rhs>(&'a self, mut with: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a
    {
        let mut result = Ok(());
        let mut i = 0;
        while i < N
        {
            result = visitor(
                &self[i],
                unsafe {
                    with.read_elem(i)
                }
            );
            i += 1;
            if result.is_err()
            {
                unsafe {
                    with.drop_elems(i..)
                };
                break;
            }
        }
        core::mem::forget(with);
        result
    }
    fn try_visit_mut_with<'a, E, F, Rhs>(&'a mut self, mut with: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a
    {
        let mut result = Ok(());
        let mut i = 0;
        while i < N
        {
            unsafe {
                result = visitor(
                    (&mut self[i] as *mut T).as_mut_unchecked(),
                    with.read_elem(i)
                );
            }
            i += 1;
            if result.is_err()
            {
                unsafe {
                    with.drop_elems(i..)
                };
                break;
            }
        }
        core::mem::forget(with);
        result
    }
        
    fn rvisit_with<'a, F, Rhs>(&'a self, with: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a
    {
        let mut i = N;
        while i > 0
        {
            i -= 1;
            visitor(
                &self[i],
                unsafe {
                    with.read_elem(i)
                }
            );
        }
        core::mem::forget(with)
    }
    fn rvisit_mut_with<'a, F, Rhs>(&'a mut self, with: Rhs, mut visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a
    {
        let mut i = N;
        while i > 0
        {
            i -= 1;
            unsafe {
                visitor(
                    (&mut self[i] as *mut T).as_mut_unchecked(),
                    with.read_elem(i)
                );
            }
        }
        core::mem::forget(with)
    }
    fn try_rvisit_with<'a, E, F, Rhs>(&'a self, mut with: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a
    {
        let mut result = Ok(());
        let mut i = N;
        while i > 0
        {
            i -= 1;
            result = visitor(
                &self[i],
                unsafe {
                    with.read_elem(i)
                }
            );
            if result.is_err()
            {
                unsafe {
                    with.drop_elems(..i)
                };
                break;
            }
        }
        core::mem::forget(with);
        result
    }
    fn try_rvisit_mut_with<'a, E, F, Rhs>(&'a mut self, mut with: Rhs, mut visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: FnMut(&'a mut T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a
    {
        let mut result = Ok(());
        let mut i = N;
        while i > 0
        {
            i -= 1;
            unsafe {
                result = visitor(
                    (&mut self[i] as *mut T).as_mut_unchecked(),
                    with.read_elem(i)
                );
            }
            if result.is_err()
            {
                unsafe {
                    with.drop_elems(..i)
                };
                break;
            }
        }
        core::mem::forget(with);
        result
    }
        
    async fn visit_async_with<'a, F, Rhs>(&'a self, with: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        Actions::new(self.zip_ref_with(with, |x, y| visitor(x, y))).await
    }
    async fn visit_mut_async_with<'a, F, Rhs>(&'a mut self, with: Rhs, visitor: F)
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a mut T, Rhs::Elem) /*+ ~const Destruct*/,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        Actions::new(self.zip_mut_with(with, |x, y| visitor(x, y))).await
    }
    async fn try_visit_async_with<'a, E, F, Rhs>(&'a self, with: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        TryActions::new(self.zip_ref_with(with, |x, y| visitor(x, y))).await
    }
    async fn try_visit_mut_async_with<'a, E, F, Rhs>(&'a mut self, with: Rhs, visitor: F) -> Result<(), E>
    where
        Rhs: ArrayForm<N>,
        F: AsyncFn(&'a mut T, Rhs::Elem) -> Result<(), E> /*+ ~const Destruct*/,
        T: 'a
    {
        #[allow(clippy::redundant_closure)]
        TryActions::new(self.zip_mut_with(with, |x, y| visitor(x, y))).await
    }
    
    fn add_all<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x + rhs)
    }
    fn sub_all<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x - rhs)
    }
    fn mul_all<Rhs>(self, rhs: Rhs) ->  [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x * rhs)
    }
    fn div_all<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x / rhs)
    }
    fn rem_all<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x % rhs)
    }
    fn shl_all<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x << rhs)
    }
    fn shr_all<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x >> rhs)
    }
    fn bitor_all<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x | rhs)
    }
    fn bitand_all<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x & rhs)
    }
    fn bitxor_all<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>,
        Rhs: Copy
    {
        self.map(|x| x ^ rhs)
    }
    fn rsub_all<Rhs>(self, rhs: Rhs) -> [<Rhs as Sub<T>>::Output; N]
    where
        Rhs: Copy + Sub<T>
    {
        self.map(|x| rhs - x)
    }
    fn rdiv_all<Rhs>(self, rhs: Rhs) -> [<Rhs as Div<T>>::Output; N]
    where
        Rhs: Copy + Div<T>
    {
        self.map(|x| rhs / x)
    }
    fn neg_all(self) -> [<T as Neg>::Output; N]
    where
        T: Neg
    {
        self.map(|x| -x)
    }
    fn not_all(self) -> [<T as Not>::Output; N]
    where
        T: Not
    {
        self.map(|x| !x)
    }
        
    async fn add_all_async<Rhs>(self, rhs: Rhs) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x + rhs).await
    }
    async fn sub_all_async<Rhs>(self, rhs: Rhs) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x - rhs).await
    }
    async fn mul_all_async<Rhs>(self, rhs: Rhs) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x * rhs).await
    }
    async fn div_all_async<Rhs>(self, rhs: Rhs) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x / rhs).await
    }
    async fn rem_all_async<Rhs>(self, rhs: Rhs) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x % rhs).await
    }
    async fn shl_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x << rhs).await
    }
    async fn shr_all_async<Rhs>(self, rhs: Rhs) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x >> rhs).await
    }
    async fn bitor_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x | rhs).await
    }
    async fn bitand_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x & rhs).await
    }
    async fn bitxor_all_async<Rhs>(self, rhs: Rhs) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>,
        Rhs: Copy
    {
        self.map_async(async |x| x ^ rhs).await
    }
    async fn rsub_all_async<Lhs>(self, lhs: Lhs) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Copy + Sub<T>
    {
        self.map_async(async |x| lhs - x).await
    }
    async fn rdiv_all_async<Lhs>(self, lhs: Lhs) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Copy + Div<T>
    {
        self.map_async(async |x| lhs / x).await
    }
    async fn neg_all_async(self) -> [<T as Neg>::Output; N]
    where
        T: Neg
    {
        self.map_async(async |x| -(x as T)).await
    }
    async fn not_all_async(self) -> [<T as Not>::Output; N]
    where
        T: Not
    {
        self.map_async(async |x| !(x as T)).await
    }

    fn add_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] += rhs;
            i += 1;
        }
    }
    fn sub_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] -= rhs;
            i += 1;
        }
    }
    fn mul_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] *= rhs;
            i += 1;
        }
    }
    fn div_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] /= rhs;
            i += 1;
        }
    }
    fn rem_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] %= rhs;
            i += 1;
        }
    }
    fn shl_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] <<= rhs;
            i += 1;
        }
    }
    fn shr_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] >>= rhs;
            i += 1;
        }
    }
    fn bitor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] |= rhs;
            i += 1;
        }
    }
    fn bitand_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] &= rhs;
            i += 1;
        }
    }
    fn bitxor_assign_all<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy
    {
        let mut i = 0;
        while i < N
        {
            self[i] ^= rhs;
            i += 1;
        }
    }
    fn rsub_assign_all<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Sub<T, Output = T>
    {
        self.map_assign(|x| lhs - x);
    }
    fn rdiv_assign_all<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Div<T, Output = T>
    {
        self.map_assign(|x| lhs / x);
    }
    fn neg_assign_all(&mut self)
    where
        T: Neg<Output = T>
    {
        self.map_assign(|x| -x)
    }
    fn not_assign_all(&mut self)
    where
        T: Not<Output = T>
    {
        self.map_assign(|x| !x)
    }
    
    async fn add_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x += rhs).await
    }
    async fn sub_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x -= rhs).await
    }
    async fn mul_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x *= rhs).await
    }
    async fn div_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x /= rhs).await
    }
    async fn rem_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x %= rhs).await
    }
    async fn shl_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShlAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x <<= rhs).await
    }
    async fn shr_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: ShrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x >>= rhs).await
    }
    async fn bitor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitOrAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x |= rhs).await
    }
    async fn bitand_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitAndAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x &= rhs).await
    }
    async fn bitxor_assign_all_async<Rhs>(&mut self, rhs: Rhs)
    where
        T: BitXorAssign<Rhs>,
        Rhs: Copy
    {
        self.visit_mut_async(async |x| *x ^= rhs).await
    }
    async fn rsub_assign_all_async<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Sub<T, Output = T>
    {
        self.map_assign_async(async |x| lhs - x).await
    }
    async fn rdiv_assign_all_async<Lhs>(&mut self, lhs: Lhs)
    where
        Lhs: Copy + Div<T, Output = T>
    {
        self.map_assign_async(async |x| lhs / x).await
    }
    async fn neg_assign_all_async(&mut self)
    where
        T: Neg<Output = T>
    {
        self.map_assign_async(async |x| -x).await
    }
    async fn not_assign_all_async(&mut self)
    where
        T: Not<Output = T>
    {
        self.map_assign_async(async |x| !x).await
    }
    
    fn add_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>
    {
        self.zip_with(rhs, Add::add)
    }
    fn sub_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>
    {
        self.zip_with(rhs, Sub::sub)
    }
    fn mul_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>
    {
        self.zip_with(rhs, Mul::mul)
    }
    fn div_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>
    {
        self.zip_with(rhs, Div::div)
    }
    fn rem_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>
    {
        self.zip_with(rhs, Rem::rem)
    }
    fn shl_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>
    {
        self.zip_with(rhs, Shl::shl)
    }
    fn shr_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>
    {
        self.zip_with(rhs, Shr::shr)
    }
    fn bitor_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>
    {
        self.zip_with(rhs, BitOr::bitor)
    }
    fn bitand_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>
    {
        self.zip_with(rhs, BitAnd::bitand)
    }
    fn bitxor_each<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>
    {
        self.zip_with(rhs, BitXor::bitxor)
    }
    fn rsub_each<Lhs>(self, lhs: [Lhs; N]) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Sub<T>
    {
        self.zip_with(lhs, |x, y| y - x)
    }
    fn rdiv_each<Lhs>(self, lhs: [Lhs; N]) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Div<T>
    {
        self.zip_with(lhs, |x, y| y / x)
    }
    
    async fn add_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Add<Rhs>>::Output; N]
    where
        T: Add<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x + y).await
    }
    async fn sub_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Sub<Rhs>>::Output; N]
    where
        T: Sub<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x - y).await
    }
    async fn mul_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<Rhs>>::Output; N]
    where
        T: Mul<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x * y).await
    }
    async fn div_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Div<Rhs>>::Output; N]
    where
        T: Div<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x / y).await
    }
    async fn rem_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Rem<Rhs>>::Output; N]
    where
        T: Rem<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x % y).await
    }
    async fn shl_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Shl<Rhs>>::Output; N]
    where
        T: Shl<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x << y).await
    }
    async fn shr_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Shr<Rhs>>::Output; N]
    where
        T: Shr<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x >> y).await
    }
    async fn bitor_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitOr<Rhs>>::Output; N]
    where
        T: BitOr<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x | y).await
    }
    async fn bitand_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitAnd<Rhs>>::Output; N]
    where
        T: BitAnd<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x & y).await
    }
    async fn bitxor_each_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as BitXor<Rhs>>::Output; N]
    where
        T: BitXor<Rhs>
    {
        self.zip_async_with(rhs, async |x, y| x ^ y).await
    }
    async fn rsub_each_async<Lhs>(self, lhs: [Lhs; N]) -> [<Lhs as Sub<T>>::Output; N]
    where
        Lhs: Sub<T>
    {
        self.zip_async_with(lhs, async |x, y| y - x).await
    }
    async fn rdiv_each_async<Lhs>(self, lhs: [Lhs; N]) -> [<Lhs as Div<T>>::Output; N]
    where
        Lhs: Div<T>
    {
        self.zip_async_with(lhs, async |x, y| y / x).await
    }

    fn add_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: AddAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x += y)
    }
    fn sub_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: SubAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x -= y)
    }
    fn mul_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: MulAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x *= y)
    }
    fn div_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: DivAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x /= y)
    }
    fn rem_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: RemAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x %= y)
    }
    fn shl_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: ShlAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x <<= y)
    }
    fn shr_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: ShrAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x >>= y)
    }
    fn bitor_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitOrAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x |= y)
    }
    fn bitand_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitAndAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x &= y)
    }
    fn bitxor_assign_each<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitXorAssign<Rhs>
    {
        self.visit_mut_with(rhs, |x, y| *x ^= y)
    }
    fn rsub_assign_each<Lhs>(&mut self, lhs: [Lhs; N])
    where
        Lhs: Sub<T, Output = T>
    {
        self.zip_assign_with(lhs, |x, y| y - x);
    }
    fn rdiv_assign_each<Lhs>(&mut self, lhs: [Lhs; N])
    where
        Lhs: Div<T, Output = T>
    {
        self.zip_assign_with(lhs, |x, y| y / x);
    }
    
    async fn add_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: AddAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x += y).await
    }
    async fn sub_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: SubAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x -= y).await
    }
    async fn mul_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: MulAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x *= y).await
    }
    async fn div_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: DivAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x /= y).await
    }
    async fn rem_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: RemAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x %= y).await
    }
    async fn shl_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: ShlAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x <<= y).await
    }
    async fn shr_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: ShrAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x >>= y).await
    }
    async fn bitor_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitOrAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x |= y).await
    }
    async fn bitand_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitAndAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x &= y).await
    }
    async fn bitxor_assign_each_async<Rhs>(&mut self, rhs: [Rhs; N])
    where
        T: BitXorAssign<Rhs>
    {
        self.visit_mut_async_with(rhs, async |x, y| *x ^= y).await
    }
    async fn rsub_assign_each_async<Lhs>(&mut self, lhs: [Lhs; N])
    where
        Lhs: Sub<T, Output = T>
    {
        self.zip_assign_async_with(lhs, async |x, y| y - x).await
    }
    async fn rdiv_assign_each_async<Lhs>(&mut self, lhs: [Lhs; N])
    where
        Lhs: Div<T, Output = T>
    {
        self.zip_assign_async_with(lhs, async |x, y| y / x).await
    }

    fn try_mul_dot<Rhs>(self, rhs: [Rhs; N]) -> Option<<T as Mul<Rhs>>::Output>
    where
        T: Mul<Rhs, Output: AddAssign>
    {
        if N == 0
        {
            return None
        }

        let mut guard = PartialZipEmptyGuard::new_left(
            self,
            rhs
        );
        let mut value = None;

        if guard.more()
        {
            let pop = |guard: &mut PartialZipEmptyGuard<_, _, _, _>| {
                let (x, y) = guard.pop();
                x*y
            };
            let value = value.insert(pop(&mut guard));
            while guard.more()
            {
                *value += pop(&mut guard)
            }
        }

        guard.done();

        value
    }
    async fn try_mul_dot_async<Rhs>(self, rhs: [Rhs; N]) -> Option<<T as Mul<Rhs>>::Output>
    where
        T: Mul<Rhs, Output: AddAssign>
    {
        self.zip_async_with(rhs, async |x, y| x*y).await
            .try_sum_async().await
    }
    fn proj<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output>>::Output; N]
    where
        T: Mul<Rhs, Output: AddAssign + Div<<T as Mul>::Output, Output: Copy>> + Mul<T, Output: AddAssign> + Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output> + Copy
    {
        if N == 0
        {
            return private::empty()
        }
        let uv = self.try_mul_dot(rhs);
        let uu = self.try_magnitude_squared();
        self.mul_all(uv.unwrap()/uu.unwrap())
    }
    async fn proj_async<Rhs>(self, rhs: [Rhs; N]) -> [<T as Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output>>::Output; N]
    where
        T: Mul<Rhs, Output: AddAssign + Div<<T as Mul>::Output, Output: Copy>> + Mul<T, Output: AddAssign> + Mul<<<T as Mul<Rhs>>::Output as Div<<T as Mul<T>>::Output>>::Output> + Copy
    {
        if N == 0
        {
            return private::empty()
        }
        let (uv, uu) = core::future::join!(
            self.try_mul_dot_async(rhs),
            self.try_magnitude_squared_async()
        ).await;
        self.mul_all_async(uv.unwrap()/uu.unwrap()).await
    }
    
    fn mul_dot_bias<Rhs>(self, rhs: [Rhs; N], bias: <T as Mul<Rhs>>::Output) -> <T as Mul<Rhs>>::Output
    where
        T: Mul<Rhs, Output: AddAssign>
    {
        if N == 0
        {
            return bias
        }

        let mut guard = PartialZipEmptyGuard::new_left(
            self,
            rhs
        );
        let mut value = bias;

        let pop = |guard: &mut PartialZipEmptyGuard<_, _, _, _>| {
            let (x, y) = guard.pop();
            x*y
        };
        while guard.more()
        {
            value += pop(&mut guard)
        }

        guard.done();

        value
    }
    async fn mul_dot_bias_async<Rhs>(self, rhs: [Rhs; N], mut bias: <T as Mul<Rhs>>::Output) -> <T as Mul<Rhs>>::Output
    where
        T: Mul<Rhs, Output: AddAssign>
    {
        if let Some(x) = self.try_mul_dot_async(rhs).await
        {
            bias += x
        }
        bias
    }

    fn mul_outer<Rhs, const M: usize>(&self, rhs: &[Rhs; M]) -> [[<T as Mul<Rhs>>::Output; M]; N]
    where
        T: Mul<Rhs> + Copy,
        Rhs: Copy
    {
        self.zip_outer_with(rhs, Mul::mul)
    }
    async fn mul_outer_async<Rhs, const M: usize>(&self, rhs: &[Rhs; M]) -> [[<T as Mul<Rhs>>::Output; M]; N]
    where
        T: Mul<Rhs> + Copy,
        Rhs: Copy
    {
        self.zip_outer_async_with(rhs, async |x, y| x*y).await
    }
    
    fn mul_cross<Rhs>(&self, rhs: [&[Rhs; N]; N - 2]) -> [<T as Sub>::Output; N]
    where
        T: MulAssign<Rhs> + Sub + Copy,
        Rhs: Copy
    {
        ArrayOps::from_fn(|i| {
            let mut m_p = self[(i + 1) % N];
            let mut m_m = self[(i + (N - 1)) % N];
    
            let mut n = 2;
            while n < N
            {
                m_p *= rhs[n - 2][(i + n) % N];
                m_m *= rhs[n - 2][(i + (N - n)) % N];
                
                n += 1;
            }
    
            m_p - m_m
        })
    }
    async fn mul_cross_async<Rhs>(&self, rhs: [&[Rhs; N]; N - 2]) -> [<T as Sub>::Output; N]
    where
        T: MulAssign<Rhs> + Sub + Copy,
        Rhs: Copy
    {
        <[<T as Sub>::Output; N] as ArrayOps<_, _>>::from_fn_async(async |i| {
            let mut m_p = self[(i + 1) % N];
            let mut m_m = self[(i + (N - 1)) % N];
    
            let mut n = 2;
            while n < N
            {
                m_p *= rhs[n - 2][(i + n) % N];
                m_m *= rhs[n - 2][(i + (N - n)) % N];
                
                n += 1;
            }
    
            m_p - m_m
        }).await
    }
    
    fn try_magnitude_squared(self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy
    {
        self.try_mul_dot(self)
    }
    async fn try_magnitude_squared_async(self) -> Option<<T as Mul<T>>::Output>
    where
        T: Mul<T, Output: AddAssign> + Copy
    {
        self.try_mul_dot_async(self).await
    }
    
    fn chain<const M: usize>(self, rhs: [T; M]) -> [T; N + M]
    {
        crate::chain(self, rhs)
    }
    
    fn rchain<const M: usize>(self, rhs: [T; M]) -> [T; N + M]
    {
        crate::rchain(self, rhs)
    }
    
    fn spread_chunks<const M: usize>(self) -> ([[T; N / M]; M], [T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:,
        [(); N / M]:
    {
        crate::spread_chunks(self)
    }
    fn spread_chunks_ref<const M: usize>(&self) -> ([&[Padded<T, M>; N / M]; M], &[T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:
    {
        let (left, right) = crate::rsplit_ptr(self, N % M);
    
        unsafe {(
            ArrayOps::from_fn(|i| &*left.add(i).cast()),
            &*right.cast()
        )}
    }
    fn spread_chunks_mut<const M: usize>(&mut self) -> ([&mut [Padded<T, M>; N / M]; M], &mut [T; N % M])
    where
        [(); M - 1]:,
        [(); N % M]:
    {
        let (left, right) = crate::rsplit_mut_ptr(self, N % M);
    
        unsafe {(
            ArrayOps::from_fn(|i| &mut *left.add(i).cast()),
            &mut *right.cast()
        )}
    }
    
    fn rspread_chunks<const M: usize>(self) -> ([T; N % M], [[T; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:,
        [(); N / M]:
    {
        crate::rspread_chunks(self)
    }
    fn rspread_chunks_ref<const M: usize>(&self) -> (&[T; N % M], [&[Padded<T, M>; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:
    {
        let (left, right) = crate::split_ptr(self, N % M);
    
        unsafe {(
            &*left.cast(),
            ArrayOps::from_fn(|i| &*right.add(i).cast())
        )}
    }
    fn rspread_chunks_mut<const M: usize>(&mut self) -> (&mut [T; N % M], [&mut [Padded<T, M>; N / M]; M])
    where
        [(); M - 1]:,
        [(); N % M]:
    {
        let (left, right) = crate::split_mut_ptr(self, N % M);
    
        unsafe {(
            &mut *left.cast(),
            ArrayOps::from_fn(|i| &mut *right.add(i).cast())
        )}
    }
    fn spread_chunks_exact<const M: usize>(self) -> [[T; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        crate::spread_chunks_exact(self)
    }
    fn spread_chunks_exact_ref<const M: usize>(&self) -> [&[Padded<T, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:
    {
        let ptr = self as *const T;
        
        ArrayOps::from_fn(|i| unsafe {&*ptr.add(i).cast()})
    }
    fn spread_chunks_exact_mut<const M: usize>(&mut self) -> [&mut [Padded<T, M>; N / M]; M]
    where
        [(); M - 1]:,
        [(); 0 - N % M]:
    {
        let ptr = self as *mut T;
        
        ArrayOps::from_fn(|i| unsafe {&mut *ptr.add(i).cast()})
    }
    
    fn chunks<const M: usize>(self) -> ([[T; M]; N / M], [T; N % M])
    {
        crate::chunks(self)
    }
    fn chunks_ref<const M: usize>(&self) -> (&[[T; M]; N / M], &[T; N % M])
    {
        crate::chunks_ref(self)
    }
    fn chunks_mut<const M: usize>(&mut self) -> (&mut [[T; M]; N / M], &mut [T; N % M])
    {
        crate::chunks_mut(self)
    }

    fn rchunks<const M: usize>(self) -> ([T; N % M], [[T; M]; N / M])
    {
        crate::rchunks(self)
    }
    fn rchunks_ref<const M: usize>(&self) -> (&[T; N % M], &[[T; M]; N / M])
    {
        crate::rchunks_ref(self)
    }
    fn rchunks_mut<const M: usize>(&mut self) -> (&mut [T; N % M], &mut [[T; M]; N / M])
    {
        crate::rchunks_mut(self)
    }
    
    fn chunks_exact<const M: usize>(self) -> [[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        crate::chunks_exact(self)
    }
    fn chunks_exact_ref<const M: usize>(&self) -> &[[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        crate::chunks_exact_ref(self)
    }
    fn chunks_exact_mut<const M: usize>(&mut self) -> &mut [[T; M]; N / M]
    where
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        crate::chunks_exact_mut(self)
    }
    
    fn array_simd<const M: usize>(self) -> ([Simd<T, M>; N / M], [T; N % M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        crate::array_simd(self)
    }
    
    fn array_rsimd<const M: usize>(self) -> ([T; N % M], [Simd<T, M>; N / M])
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); N % M]:,
        [(); N / M]:
    {
        crate::array_rsimd(self)
    }
    
    fn array_simd_exact<const M: usize>(self) -> [Simd<T, M>; N / M]
    where
        T: SimdElement,
        LaneCount<M>: SupportedLaneCount,
        [(); 0 - N % M]:,
        [(); N / M]:
    {
        crate::array_simd_exact(self)
    }
    
    fn split_array<const M: usize>(self) -> ([T; M], [T; N - M])
    where
        [(); N - M]:
    {
        crate::split_array(self)
    }
    fn split_array_ref2<const M: usize>(&self) -> (&[T; M], &[T; N - M])
    where
        [(); N - M]:
    {
        crate::split_array_ref(self)
    }
    fn split_array_mut2<const M: usize>(&mut self) -> (&mut [T; M], &mut [T; N - M])
    where
        [(); N - M]:
    {
        crate::split_array_mut(self)
    }
    
    fn rsplit_array<const M: usize>(self) -> ([T; N - M], [T; M])
    where
        [(); N - M]:
    {
        crate::rsplit_array(self)
    }
    fn rsplit_array_mut2<const M: usize>(&mut self) -> (&mut [T; N - M], &mut [T; M])
    where
        [(); N - M]:
    {
        crate::rsplit_array_mut(self)
    }
    fn rsplit_array_ref2<const M: usize>(&self) -> (&[T; N - M], &[T; M])
    where
        [(); N - M]:
    {
        crate::rsplit_array_ref(self)
    }

    fn each_ref(&self) -> [&T; N]
    {
        ArrayOps::from_fn(|i| {
            &self[i]
        })
    }
    fn each_mut(&mut self) -> [&mut T; N]
    {
        ArrayOps::from_fn(|i| unsafe {
            (&mut self[i] as *mut T).as_mut_unchecked()
        })
    }
    fn each_pin_ref(self: Pin<&Self>) -> [Pin<&T>; N]
    {
        ArrayOps::from_fn(|i| unsafe {
            self.map_unchecked(|this| {
                &this[i]
            })
        })
    }
    fn each_pin_mut(mut self: Pin<&mut Self>) -> [Pin<&mut T>; N]
    {
        ArrayOps::from_fn(|i| unsafe {
            Pin::new_unchecked(&mut (self.as_mut().get_unchecked_mut() as *mut Self).as_mut_unchecked()[i])
        })
    }
    
    fn bit_rev_permutation(&mut self)
    where
        [(); is_power_of(N, 2) as usize - 1]:
    {
        self.digit_rev_permutation::<2>()
    }
    fn digit_rev_permutation<const R: usize>(&mut self)
    where
        [(); is_power_of(N, R) as usize - 1]:
    {
        crate::digit_rev_permutation::<_, _, R>(self)
    }

    fn grey_code_permutation(&mut self)
    where
        [(); N.is_power_of_two() as usize - 1]:
    {
        crate::grey_code_permutation(self)
    }
}