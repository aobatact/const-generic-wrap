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

#[cfg(test)]
mod test{
    use crate::*;
    use typenum::marker_traits::*;
    #[test]
    fn typenum_test(){
        assert_eq!(WrapU32::<260>::U32,260);
        assert_eq!(WrapU32::<260>::U8,4);
        
        assert_eq!(WrapU32::<260>::I32,260);
        assert_eq!(WrapI32::<260>::I32,260);
    }
}