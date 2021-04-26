//! Simple wrapper for const generics.
//! # Usage
//! Currently 'the type of const parameters must not depend on other generic parameters' (E0770).
//! ```compile_fail
//! struct A<N, const C : N>(N);
//! ```
//! With this crate we can solve this by wrapping cosnt generic.
//! ```
//! # use core::mem;
//! # use core::ops::Add;
//! use const_generic_wrap::*;
//! struct A<N, C>(N, C) where C : ConstWrap<BaseType = N>;
//! // WrapU32 is ZST, so the size of A is as same as u32.
//! assert_eq!(mem::size_of::<WrapU32<12>>(), 0);
//! assert_eq!(mem::size_of::<A::<u32, WrapU32<12>>>(), mem::size_of::<u32>());
//!
//! // you can selectively use const or non const
//! struct B<N, C>(N, C) where C : ConstOrValue<N>; // or it can be C : Into<N>
//! fn add_b<N, C>(v : B<N, C>) -> N where N : Add<Output = N>, C : ConstOrValue<N>{
//!     v.0 + v.1.into()
//! }
//! let b_non_const = B(31, 11);
//! let b_const = B(31, WrapI32::<11>);
//! assert_eq!(add_b(b_non_const), add_b(b_const));
//! ```

#![no_std]
#![cfg_attr(feature = "unstable", feature(const_evaluatable_checked))]
#![cfg_attr(feature = "unstable", feature(const_generics))]

use core::cmp::Ordering;
pub(crate) use seal::ConstSeal;
mod seal {
    /// Seal the ConstWrap not to be implemented with outer type.
    pub trait ConstSeal {}
}

/// Marker that shows it wraps const generic.
pub trait ConstWrap:
    Clone + Copy + Default + Eq + core::hash::Hash + PartialEq + PartialOrd + Ord + ConstSeal
{
    /// Type which is wrapped.
    type BaseType;
    /// Value which is wrapped.
    const VALUE: Self::BaseType;
}

pub trait ConstOrValue<T>: Into<T> {
    const IS_CONST: bool;
    fn get_val(self) -> T {
        self.into()
    }
}

impl<T> ConstOrValue<T> for T {
    const IS_CONST: bool = false;
    fn get_val(self) -> T {
        self
    }
}

macro_rules! wrap_impl {
    ($tb: ty, $t : ident) => {
        /// Const generic wrapper.
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct $t<const T: $tb>;
        impl<const T: $tb> ConstSeal for $t<T>{}
        impl<const T: $tb> ConstWrap for $t<T> {
            type BaseType = $tb;
            const VALUE : $tb = T;
        }
        impl<const T: $tb> ConstOrValue<$tb> for $t<T> {
            const IS_CONST : bool = true;
        }
        impl<const T: $tb> From<$t<T>> for $tb {
            fn from(_ : $t<T>) -> $tb { T }
        }
        impl<'a, const T: $tb> From<$t<T>> for &'a $tb {
            fn from(_ : $t<T>) -> &'a $tb { &T }
        }
        impl<const T: $tb> PartialEq<$tb> for $t<T> {
            fn eq(&self, other: &$tb) -> bool { T.eq(other)}
        }
        impl<const T: $tb> PartialOrd<$tb> for $t<T> {
            fn partial_cmp(&self, other: &$tb) -> Option<Ordering> { T.partial_cmp(other)}
        }
    };
    [$(($tb: ty, $t : tt)),*$(,)*] => {
        $(
            wrap_impl!($tb, $t);
        )*
    };
}

wrap_impl![
    (u8, WrapU8),
    (u16, WrapU16),
    (u32, WrapU32),
    (u64, WrapU64),
    (usize, WrapUSIZE),
    (i8, WrapI8),
    (i16, WrapI16),
    (i32, WrapI32),
    (i64, WrapI64),
    (isize, WrapISIZE),
    (bool, WrapBOOL),
    (char, WrapCHAR),
];

/*
#[cfg(feature = "unstable")]
pub mod ops{
    use crate::*;
    use core::ops::*;
    impl<const T1: i32, const T2: i32> Add<WrapI32<T2>> for WrapI32<T1>{
        type Output = WrapI32<{T1+T2}>;
        fn add(self, _: WrapI32<T2>) -> Self::Output { WrapI32::<{T1+T2}> }
    }
}
*/
#[cfg(feature = "typenum")]
mod typenum_bridge {
    use crate::*;

    impl typenum::marker_traits::Bit for WrapBOOL<false> {
        const U8: u8 = 0;
        const BOOL: bool = false;
        fn new() -> Self {
            WrapBOOL
        }
        fn to_u8() -> u8 {
            0
        }
        fn to_bool() -> bool {
            false
        }
    }

    impl typenum::marker_traits::Bit for WrapBOOL<true> {
        const U8: u8 = 1;
        const BOOL: bool = true;
        fn new() -> Self {
            WrapBOOL
        }
        fn to_u8() -> u8 {
            1
        }
        fn to_bool() -> bool {
            true
        }
    }

    macro_rules! impl_unsigned {
        ($tb: ty, $t : ident) => {
            impl<const T: $tb> typenum::marker_traits::Unsigned for $t<T>{

                const U8: u8 = T as u8;
                const U16: u16 = T as u16;
                const U32: u32 = T as u32;
                const U64: u64 = T as u64;
                const USIZE : usize = T as usize;
                const I8:  i8 =  T as i8;
                const I16: i16 = T as i16;
                const I32: i32 = T as i32;
                const I64: i64 = T as i64;
                const ISIZE : isize = T as isize;

                fn to_u8() -> u8 { Self::U8 }
                fn to_u16() -> u16 { Self::U16 }
                fn to_u32() -> u32 { Self::U32 }
                fn to_u64() -> u64 { Self::U64 }
                fn to_usize() -> usize { Self::USIZE }
                fn to_i8() ->  i8 { Self::I8 }
                fn to_i16() -> i16 { Self::I16 }
                fn to_i32() -> i32 { Self::I32 }
                fn to_i64() -> i64 { Self::I64 }
                fn to_isize() -> isize { Self::ISIZE }
            }

            impl typenum::marker_traits::Zero for $t<0>{}

            #[cfg(feature="unstable")]
            impl<const T: $tb> typenum::marker_traits::NonZero for $t<T> where WrapBool<{T != 0}> : typenum::type_operators::Same<WrapBool<true>>  {}

        };
        [$(($tb: ty, $t : tt)),*$(,)*] => {
            $(
                impl_unsigned!($tb, $t);
            )*
        }
    }

    impl_unsigned![
        (u8, WrapU8),
        (u16, WrapU16),
        (u32, WrapU32),
        (u64, WrapU64),
        (usize, WrapUSIZE)
    ];

    macro_rules! impl_signed {
        ($tb: ty, $t : ident) => {
            impl<const T: $tb> typenum::marker_traits::Integer for $t<T>{
                const I8:  i8 =  T as i8;
                const I16: i16 = T as i16;
                const I32: i32 = T as i32;
                const I64: i64 = T as i64;
                const ISIZE : isize = T as isize;

                fn to_i8() ->  i8 { Self::I8 }
                fn to_i16() -> i16 { Self::I16 }
                fn to_i32() -> i32 { Self::I32 }
                fn to_i64() -> i64 { Self::I64 }
                fn to_isize() -> isize { Self::ISIZE }
            }

            impl typenum::marker_traits::Zero for $t<0>{}

            #[cfg(feature="unstable")]
            impl<const T: $tb> typenum::marker_traits::NonZero for $t<T> where WrapBool<{T != 0}> : typenum::type_operators::Same<WrapBool<true>>  {}
        };
        [$(($tb: ty, $t : tt)),*$(,)*] => {
            $(
                impl_signed!($tb, $t);
            )*
        }
    }

    impl_signed![
        (i8, WrapI8),
        (i16, WrapI16),
        (i32, WrapI32),
        (i64, WrapI64),
        (isize, WrapISIZE)
    ];
}

#[cfg(test)]
mod tests {

    use crate::*;
    #[test]
    fn wrap_unwrap() {
        let n3 = WrapI32::<3>;
        assert_eq!(0, core::mem::size_of_val(&n3));
        let m: i32 = n3.into();
        assert_eq!(3, m);
    }
}
