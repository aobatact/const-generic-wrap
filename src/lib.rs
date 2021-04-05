//! Simple wrapper for const generics.

#![no_std]
#![cfg_attr(feature = "unstable", feature(const_evaluatable_checked))]
#![cfg_attr(feature = "unstable", feature(const_generics))]

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapUsize<const T: usize>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapU8<const T: u8>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapU16<const T: u16>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapU32<const T: u32>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapU64<const T: u64>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapIsize<const T: isize>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapI8<const T: i8>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapI16<const T: i16>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapI32<const T: i32>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapI64<const T: i64>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapBool<const T: bool>;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WrapChar<const T: char>;

/*
pub mod ops{
    use crate::*;
    use core::ops::*;
    impl<const T1: isize, const T2: isize> Add<WrapI64<T2>> for WrapI64<T1>{

        type Output = WrapI64<{T1 + T2}>;
        fn add(self, _: Rhs) -> <Self as std::ops::Add<Rhs>>::Output { WrapI64<{T1+T2}> }
    }
}
*/
#[cfg(feature = "typenum")]
mod typenum_bridge {
    use crate::*;

    impl typenum::marker_traits::Bit for WrapBool<false> {
        const U8: u8 = 0;
        const BOOL: bool = false;
        fn new() -> Self {
            WrapBool
        }
        fn to_u8() -> u8 {
            0
        }
        fn to_bool() -> bool {
            false
        }
    }

    impl typenum::marker_traits::Bit for WrapBool<true> {
        const U8: u8 = 1;
        const BOOL: bool = true;
        fn new() -> Self {
            WrapBool
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
        (usize, WrapUsize)
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
        (isize, WrapIsize)
    ];
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
