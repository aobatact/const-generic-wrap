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

/// Trait that can be a wrapped const generic or a owned value.
pub trait ConstOrValue<T>: Into<T> {
    /// get wheter the type is const generic wrapper.
    const IS_CONST_WRAP: bool;
}

impl<T> ConstOrValue<T> for T {
    const IS_CONST_WRAP: bool = false;
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
            const IS_CONST_WRAP : bool = true;
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

#[cfg(feature = "typenum")]
mod typenum_bridge;

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
