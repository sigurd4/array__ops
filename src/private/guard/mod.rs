use core::marker::ConstParamTy;

moddef::moddef!(
    flat(pub(crate)) mod {
        partial_divide_and_conquer_guard,
        partial_empty_guard,
        partial_zip_empty_guard,
        partial_map_guard,
        partial_zip_guard,
        partial_init_guard
    }
);

#[derive(ConstParamTy, PartialEq, Eq)]
pub(crate) enum Dir
{
    Left,
    Right
}