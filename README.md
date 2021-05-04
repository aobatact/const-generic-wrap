# const_generic_wrap

Simple wrapper for const generics.
[![Doc](https://docs.rs/const-generic-wrap/badge.svg)](https://docs.rs/const-generic-wrap)
[![Crate](https://img.shields.io/crates/v/const-generic-wrap.svg)](https://crates.io/crates/const-generic-wrap)

# Usage
Currently 'the type of const parameters must not depend on other generic parameters' (E0770).
```compile_fail
struct A<N, const C : N>(N);
```
With this crate we can solve this by wrapping cosnt generic.
```
use const_generic_wrap::*;
struct A<N, C>(N, C) where C : ConstWrap<BaseType = N>;
// WrapU32 is ZST, so the size of A is as same as u32.
assert_eq!(mem::size_of::<WrapU32<12>>(), 0);
assert_eq!(mem::size_of::<A::<u32, WrapU32<12>>>(), mem::size_of::<u32>());
// you can selectively use const or non const
struct B<N, C>(N, C) where C : ConstOrValue<N>; // or it can be C : Into<N>
fn add_b<N, C>(v : B<N, C>) -> N where N : Add<Output = N>, C : ConstOrValue<N>{
    v.0 + v.1.into()
}
let b_non_const = B(31, 11);
let b_const = B(31, WrapI32::<11>);
assert_eq!(add_b(b_non_const), add_b(b_const));
```