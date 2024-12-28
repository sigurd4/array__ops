#![allow(non_snake_case)]
#![cfg_attr(not(test), no_std)]
#![allow(incomplete_features)]
#![allow(internal_features)]
#![allow(async_fn_in_trait)]
#![allow(refining_impl_trait)]
#![allow(clippy::type_complexity)]
#![feature(associated_type_defaults)]
#![feature(const_trait_impl)]
#![feature(unboxed_closures)]
#![feature(const_for)]
#![feature(generic_arg_infer)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(const_destruct)]
#![feature(associated_const_equality)]
#![feature(const_swap_nonoverlapping)]
#![feature(portable_simd)]
#![feature(const_swap)]
#![feature(allocator_api)]
#![feature(let_chains)]
#![feature(const_array_each_ref)]
#![feature(ptr_as_ref_unchecked)]
#![feature(async_fn_traits)]
#![feature(maybe_uninit_slice)]
#![feature(future_join)]
#![feature(slice_swap_unchecked)]
#![feature(adt_const_params)]
#![feature(arbitrary_self_types)]
#![feature(stmt_expr_attributes)]
#![feature(specialization)]
#![feature(core_intrinsics)]
#![feature(unsized_const_params)]
#![feature(const_closures)]
#![feature(generic_const_exprs)]

//! Provides many useful utility methods for arrays.
//! 
//! I had to name it `array__ops` because `array_ops` was taken.
//! 
//! # Todo
//! 
//! - for_each_zip
//! - nd-ops(?)
//! - Document more

#[cfg(feature = "alloc")]
extern crate alloc;

moddef::moddef!(
    flat(pub) mod {
        from_fn for warn(non_snake_case)
    },
    pub mod {
        join for warn(non_snake_case),
        ops for warn(non_snake_case),
        form for warn(non_snake_case)
    },
    mod private for warn(non_snake_case)
);

pub use array_trait::*;
pub use slice_ops;
pub use slice_ops::Padded;
pub use slice_ops::Slice;
pub use slice_ops::SliceOps;
pub use slice_ops::SlicePrereq;

pub const fn min_len(a: usize, b: usize) -> usize
{
    if a < b
    {
        a
    }
    else
    {
        b
    }
}
pub const fn max_len(a: usize, b: usize) -> usize
{
    if a > b
    {
        a
    }
    else
    {
        b
    }
}

#[warn(non_snake_case)]
pub mod asm
{
    use slice_ops::Padded;

    use crate::ops::*;

    const I: usize = 2;
    const N: usize = 4;
    const M: usize = 3;
    const S: usize = 2;
    const MN: (usize, usize) = (4, 3);
    const HW: (usize, usize) = (2, 2);

    #[inline(never)]
    pub fn visit(a: &mut [i32; N])
    {
        a.visit_mut(|x| *x = 0)
    }

    #[inline(never)]
    pub fn truncate(a: [i32; N]) -> [i32; N - S]
    {
        a.truncate()
    }

    #[inline(never)]
    pub fn spread_into(a: [i32; N]) -> ([[i32; N / M]; M], [i32; N % M])
    {
        a.spread()
    }

    #[inline(never)]
    pub fn spread_ref(a: &[i32; N]) -> ([&[Padded<i32, M>; N / M]; M], &[i32; N % M])
    {
        a.spread_ref()
    }

    #[inline(never)]
    pub fn split_into(a: [i32; N]) -> ([i32; S], [i32; N - S])
    {
        a.split_array2()
    }

    #[inline(never)]
    pub fn split_ref(a: &[i32; N]) -> (&[i32; S], &[i32; N - S])
    {
        a.split_array_ref2()
    }

    #[inline(never)]
    pub fn into_shift(a: [i32; N], b: [i32; M]) -> ([i32; M], [i32; N])
    {
        a.into_shift_many_left(b)
    }
    #[inline(never)]
    pub fn shift_mut(a: &mut [i32; N], b: &mut [i32; M])
    {
        a.shift_many_left(b);
    }
    #[inline(never)]
    pub fn shift_drop(a: &mut [i32; N], b: [i32; M])
    {
        a.shift_many_left(b);
    }

    #[inline(never)]
    pub fn into_rotate(a: [i32; N]) -> [i32; N]
    {
        a.into_rotate_left(1)
    }
    #[inline(never)]
    pub fn rotate_mut(a: &mut [i32; N])
    {
        a.rotate_left(1)
    }

    #[inline(never)]
    pub fn grey_code_permutation(a: &mut [i32; 1 << I])
    {
        a.grey_code_permutation()
    }
    #[inline(never)]
    pub fn bit_rev_permutation(a: &mut [i32; 1 << I])
    {
        a.bit_rev_permutation()
    }

    #[inline(never)]
    pub fn mul_cross(a: &[i32; 3], b: &[i32; 3]) -> [i32; 3]
    {
        a.mul_cross([b])
    }
    #[inline(never)]
    pub fn mul_dot(a: [i32; N], b: [i32; N]) -> i32
    {
        a.mul_dot_bias(b, 0)
    }
    #[inline(never)]
    pub fn mul_outer(a: &[i32; N], b: &[i32; N]) -> [[i32; N]; N]
    {
        a.mul_outer(b)
    }
    #[inline(never)]
    pub fn mul_kronecker(a: &[[i32; MN.1]; MN.0], b: &[[i32; HW.1]; HW.0]) -> [[i32; MN.1 * HW.1]; MN.0 * HW.0]
    {
        a.mul_kronecker(b)
    }
    #[inline(never)]
    pub fn proj(a: [f32; N], b: [f32; N]) -> [f32; N]
    {
        a.proj(b)
    }

    #[inline(never)]
    pub fn magnitude(a: [i32; N]) -> Option<i32>
    {
        a.try_magnitude_squared()
    }

    #[inline(never)]
    pub fn isolate(a: [i32; N]) -> Option<i32>
    {
        a.isolate(0)
    }

    #[inline(never)]
    pub fn into_diagonal(a: [[i32; MN.1]; MN.0]) -> [i32; crate::min_len(MN.0, MN.1)]
    {
        a.diagonal()
    }

    #[inline(never)]
    pub fn diagonal_matrix(a: [i32; N]) -> [[i32; N]; N]
    {
        a.diagonal_matrix_exact()
    }

    #[inline(never)]
    pub fn from_fn() -> [i32; N]
    {
        crate::from_fn(|i| i as i32)
    }

    #[inline(never)]
    pub fn fold(o: i32, a: [i32; N]) -> i32
    {
        a.fold(o, |o, x| o + x)
    }

    #[inline(never)]
    pub fn reduce(a: [i32; N]) -> Option<i32>
    {
        a.reduce(|x, y| x + y)
    }

    #[inline(never)]
    pub fn divide_and_conquer(a: [i32; N]) -> Option<i32>
    {
        a.divide_and_conquer(|x, y| x + y)
    }

    #[inline(never)]
    pub fn flatmap(a: [[i32; M]; N]) -> [i32; N * M]
    {
        a.flatmap(|x| x)
    }

    #[inline(never)]
    pub fn map_(a: [i32; N]) -> [i32; N]
    {
        a.map(|x| x)
    }

    #[inline(never)]
    pub fn chunks(a: [i32; N]) -> ([[i32; M]; N / M], [i32; N % M])
    {
        a.chunks()
    }

    #[inline(never)]
    pub fn differentiate(a: &mut [i32; N])
    {
        a.differentiate();
    }

    #[inline(never)]
    pub fn integrate(a: &mut [i32; N])
    {
        a.integrate();
    }

    #[inline(never)]
    pub fn find(a: &[i32; N]) -> Option<usize>
    {
        a.find(&1)
    }

    #[inline(never)]
    pub fn argmin(a: &[i32; N]) -> Option<usize>
    {
        a.argmin()
    }

    #[inline(never)]
    pub fn add_simd(mut a: [i32; N], b: [i32; N]) -> [i32; N]
    {
        const SIMD: usize = 4;

        let (a1, a2) = a.array_simd_mut::<SIMD>();
        let (b1, b2) = b.array_simd::<SIMD>();

        a1.add_assign_each(b1);
        a2.add_assign_each(b2);

        a
    }

    #[inline(never)]
    pub fn add_each(a: [i32; N], b: [i32; N]) -> [i32; N]
    {
        a.add_each(b)
    }

    #[inline(never)]
    pub fn add_assign(a: &mut [i32; N], b: [i32; N])
    {
        a.add_assign_each(b)
    }

    #[inline(never)]
    pub fn transpose(a: [[i32; MN.1]; MN.0]) -> [[i32; MN.0]; MN.1]
    {
        a.transpose()
    }
}

#[cfg(test)]
#[warn(non_snake_case)]
mod tests
{
    use slice_ops::Padded;

    use crate::ops::*;

    #[test]
    fn kronecker()
    {
        let a = [1, 2, 3];
        let b = [1, 2];
        let [c] = [a].mul_kronecker(&[b]);

        println!("{:?}", c)
    }

    #[test]
    fn toeplitz()
    {
        let a = [1, 2, 3];
        let t = a.toeplitz_matrix();
        let b = [4, 5, 6];
        let h = a.hankel_matrix(&b);

        println!("{:?}", t);
        println!("{:?}", h)
    }

    #[test]
    fn mod0()
    {
        let a = [1, 2, 3];
        let c = a.chunks_exact::<1>();
        println!("{:?}", c);
    }

    #[test]
    fn gpa()
    {
        #[repr(u8)]
        enum Grade
        {
            A = 5,
            B = 4,
            C = 3,
            D = 2,
            E = 1
        }

        const GRADES_UNI: [(u8, Grade); 21] = [
            (5, Grade::C),  // Ingeniørrollen
            (5, Grade::A),  // Programmering for beregning
            (5, Grade::B),  // Elektrisitetslære
            (5, Grade::D),  // Digitalteknikk
            (10, Grade::A), // Programmering og mikrokontrollere
            (10, Grade::A), // Matematikk 1
            (5, Grade::C),  // Fysikk 1 - Mekanikk
            (5, Grade::A),  // Elektrisitetslære 2
            (5, Grade::A),  // Programmerbare logiske kretser
            (10, Grade::A), // Matematikk 2
            (5, Grade::C),  // Kommunikasjon
            (10, Grade::B), // Analog elektronikk
            (10, Grade::B), // Systems design and engineering
            (5, Grade::C),  // Statistikk
            (10, Grade::E), // Signalbehandling
            (10, Grade::C), // Reguleringsteknikk 1
            (5, Grade::B),  // Fysikk 2 - Elektromagnetisme
            (10, Grade::C), // Reguleringsteknikk 2
            (10, Grade::C), // Matematikk 3
            (10, Grade::C), // Instrumentering og styring
            (20, Grade::B)  // Bacheloroppgave - Automatisk gir-system for Lone Wolf ATV
        ];
        const GRADES_VGS: [u8; 23] = [
            5, // Engelsk
            2, // Spansk II
            4, // Geografi
            4, // Historie
            4, // Kroppsøving
            4, // Matematikk 1T
            5, // Naturfag
            4, // Norsk hovedmål
            4, // Norsk hovedmål, eksamen
            3, // Norsk sidemål
            2, // Norsk sidemål, eksamen
            3, // Norsk
            3, // Religion og etikk
            4, // Samfunnsfag
            4, // Fysikk 1
            4, // Fysikk 2
            5, // Fysikk 2, eksamen
            3, // Kjemi
            4, // Informasjonsteknologi 1
            5, // Informasjonsteknologi 2
            4, // Teknologi og forskningslære 1
            3, // Matematikk R1
            4  // Matematikk R2
        ];

        let gpa_uni: f32 = GRADES_UNI.map(|(pts, grade)| (pts * grade as u8) as u16).sum_from(0) as f32 / GRADES_UNI.map(const |(pts, _)| pts as u16).sum_from(0) as f32;

        println!("{}", gpa_uni);

        let gpa_vgs: f32 = GRADES_VGS.map(|grade| grade as u16).sum_from(0) as f32 / GRADES_VGS.len() as f32;

        println!("{}", gpa_vgs);
    }

    /*#[test]
    fn benchmark()
    {
        const N: usize = 64;
        const M: usize = 256;

        assert_eq!(<[[[u8; 2]; N]; M]>::DIMENSIONS, [M, N, 2]);

        let a: [[[u8; 2]; N]; M] = ArrayNdOps::fill_nd(|i| i.map(|i| i as u8));

        let t0 = SystemTime::now();
        for m in 0..M
        {
            for n in 0..N
            {
                //<[u8; N]>::fill(|i| i as u8);
                //a[m].truncate::<{N/2}>();
                //a[m].resize::<{N/2}, _>(|i| [m as u8, i as u8]);
                //let (matrix, _) = a[m].array_spread::<3>();
                for k in 0..2
                {
                    let i = [m, n, k];
                    let _ = *a.get_nd(i).unwrap();
                }
            }
        }
        let t = t0.elapsed().unwrap();
        println!("t = {:?}", t); //10.5832ms
    }*/

    #[test]
    fn reduce()
    {
        /*const A: [[(u8, u8); 3]; 2] = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)]
        ];

        let r: (u8, u8) = A.reduce_nd(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2)).unwrap();

        assert_eq!(r, (3, 6));*/
    }

    #[test]
    fn rotate()
    {
        let mut a = [1, 2, 3, 4, 5];

        a.rotate_left(2);
        println!("{:?}", a);
    }

    #[test]
    fn test_spread_align()
    {
        let str = b"abcdefghijklmnopqrstuvwxyz".map(|c| c as char);

        println!("Alignment char = {}", core::mem::align_of::<char>());
        println!("Alignment padded x3 char = {}", core::mem::align_of::<Padded<char, 3>>());

        println!("Alignment String = {}", core::mem::align_of::<String>());
        println!("Alignment padded x3 String = {}", core::mem::align_of::<Padded<String, 3>>());

        println!("str: {:?}", str);
        println!("spread: {:?}", str.spread_ref::<3>());
        println!("chunks: {:?}", str.chunks_ref::<3>());

        assert_eq!(
            str.spread::<3>(),
            (
                [
                    ['a', 'd', 'g', 'j', 'm', 'p', 's', 'v'],
                    ['b', 'e', 'h', 'k', 'n', 'q', 't', 'w'],
                    ['c', 'f', 'i', 'l', 'o', 'r', 'u', 'x']
                ],
                ['y', 'z']
            )
        );
        assert_eq!(
            str.chunks::<3>(),
            (
                [
                    ['a', 'b', 'c'],
                    ['d', 'e', 'f'],
                    ['g', 'h', 'i'],
                    ['j', 'k', 'l'],
                    ['m', 'n', 'o'],
                    ['p', 'q', 'r'],
                    ['s', 't', 'u'],
                    ['v', 'w', 'x']
                ],
                ['y', 'z']
            )
        );
    }

    /*#[test]
    fn nd_test()
    {
        type T = u8;

        const ND: [[T; 3]; 3] = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];
        let flat: [T; 9] = ND.flatten_nd_array();
        assert_eq!(flat, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let nd_t: [[T; 3]; 3] = ND.transpose();

        let flat_t: [T; 9] = nd_t.flatten_nd_array();
        assert_eq!(flat_t, [1, 4, 7, 2, 5, 8, 3, 6, 9]);
    }*/

    #[test]
    fn generate_impl_nd_array_macro_args()
    {
        const R: usize = 110;

        print!("impl_nd_array!(\n   ");
        let mut c = 0;
        for i in 0usize..256
        {
            c += (i.max(1)).ilog10() as usize + 3;
            if c > R
            {
                print!("\n   ");
                c = 0;
            }
            print!(" _{}", i);
        }
        println!("\n);")
    }
}
