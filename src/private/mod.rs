moddef::moddef!(
    pub(crate) mod {
        guard,
        boxed_array for cfg(feature = "alloc")
    }
);

#[repr(C)]
pub(crate) struct Pair<L, R>
{
    pub left: L,
    pub right: R
}

impl<L, R> Pair<L, R>
{
    pub(crate) const fn new(left: L, right: R) -> Self
    {
        Self {left, right}
    }

    pub(crate) const fn unpack(self) -> (L, R)
    {
        if core::mem::size_of::<(L, R)>() == core::mem::size_of::<Pair<L, R>>() && core::mem::align_of::<(L, R)>() == core::mem::align_of::<Pair<L, R>>()
        {
            unsafe {
                return transmute(self)
            }
        }

        let left_right = unsafe {(
            core::ptr::read(&self.left),
            core::ptr::read(&self.right)
        )};

        core::mem::forget(self);

        left_right
    }

    pub(crate) const fn pack(left_right: (L, R)) -> Self
    {
        if core::mem::size_of::<(L, R)>() == core::mem::size_of::<Pair<L, R>>() && core::mem::align_of::<(L, R)>() == core::mem::align_of::<Pair<L, R>>()
        {
            unsafe {
                return transmute(left_right)
            }
        }

        let pair =  unsafe {
            Self {
                left: core::ptr::read(&left_right.0),
                right: core::ptr::read(&left_right.1)
            }
        };

        core::mem::forget(left_right);

        pair
    }
    
    pub(crate) const fn unpack_mandrop(self) -> (ManuallyDrop<L>, ManuallyDrop<R>)
    {
        if const {core::mem::size_of::<(L, R)>() == core::mem::size_of::<Pair<L, R>>()}
            && const {core::mem::align_of::<(L, R)>() == core::mem::align_of::<Pair<L, R>>()}
        {
            unsafe {
                return transmute(self)
            }
        }

        let left_right = unsafe {(
            ManuallyDrop::new(core::ptr::read(&self.left)),
            ManuallyDrop::new(core::ptr::read(&self.right))
        )};

        core::mem::forget(self);

        left_right
    }
}

impl<L, R> From<(L, R)> for Pair<L, R>
{
    fn from(left_right: (L, R)) -> Self
    {
        Self::pack(left_right)
    }
}

impl<L, R> Into<(L, R)> for Pair<L, R>
{
    fn into(self) -> (L, R)
    {
        self.unpack()
    }
}

use core::mem::{ManuallyDrop, MaybeUninit};

/*impl<T, const P: &'static [usize]> NotTuple for PartitionedArray<T, P>
where
[(); crate::sum_len::<{P}>()]: {}*/

pub(crate) const fn empty<T, const N: usize>() -> [T; N]
{
    assert!(N == 0);
    unsafe {
        MaybeUninit::assume_init(MaybeUninit::uninit())
    }
}

pub(crate) const unsafe fn split_transmute<A, B, C>(a: A) -> (B, C)
{
    // Doesn't help
    /*if const {core::mem::size_of::<A>() == core::mem::size_of::<(B, C)>()}
        && const {core::mem::align_of::<A>() == core::mem::align_of::<(B, C)>()}
    {
        unsafe {
            return transmute(a)
        }
    }*/
    transmute::<_, Pair<_, _>>(a).unpack()
}

pub(crate) const unsafe fn merge_transmute<A, B, C>(a: A, b: B) -> C
{
    // Doesn't help
    /*if const {core::mem::size_of::<(A, B)>() == core::mem::size_of::<C>()}
        && const {core::mem::align_of::<(A, B)>() == core::mem::align_of::<C>()}
    {
        unsafe {
            return transmute((a, b))
        }
    }*/
    transmute(Pair::new(a, b))
}

pub(crate) const unsafe fn overlap_swap_transmute<A, B>(a: A, b: B) -> (B, A)
{
    // Doesn't help
    /*if const {core::mem::size_of::<(A, B)>() == core::mem::size_of::<(B, A)>()}
        && const {core::mem::align_of::<(A, B)>() == core::mem::align_of::<(B, A)>()}
    {
        unsafe {
            return transmute((a, b))
        }
    }*/
    merge_transmute::<_, _, Pair<_, _>>(a, b).unpack()
}

pub(crate) const unsafe fn transmute<A, B>(from: A) -> B
{
    #[cfg(test)]
    assert!(
        const {core::mem::size_of::<A>() == core::mem::size_of::<B>()} && const {core::mem::align_of::<A>() == core::mem::align_of::<B>()},
        "Cannot transmute due to unequal size or alignment"
    );
    core::intrinsics::transmute_unchecked(from)
}

pub(crate) const unsafe fn uninit_extend_transmute<A, B>(from: A) -> MaybeUninit<B>
{
    union AB<A, B>
    {
        from: ManuallyDrop<A>,
        to: ManuallyDrop<MaybeUninit<B>>
    }

    unsafe {
        ManuallyDrop::into_inner(AB {from: ManuallyDrop::new(from)}.to)
    }
}