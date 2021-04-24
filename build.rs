use std::env;
use std::fs::File;
use std::io::*;
use std::path::*;
fn main() {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("wrappers.rs");
    let mut out_file = File::create(out_path).unwrap();

    const NTYPES: [&str; 12] = [
        "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize", "bool", "char",
    ];

    /*
    let u_types = &NTYPES[..5];
    let i_types = &NTYPES[5..10];
    let n_types = &NTYPES[10..];
    */

    for ty in NTYPES.iter() {
        let tw = ty.to_ascii_uppercase();
        out_file
            .write_fmt(format_args!(
"///Wrapper for [`{ty}`]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Wrap{tw}<const T: {ty}>;
impl<const T: {ty}> From<Wrap{tw}<T>> for {ty}{{
    fn from(_ : Wrap{tw}<T>) -> {ty} {{ T }}
}}
/*
impl MayBeWrapped for {ty}{{
    type Output = {ty};
    fn get(self) -> Self::Output {{ self }}
}}
impl<const T: {ty}> MayBeWrapped for Wrap{tw}<T>{{
    type Output = {ty};
    fn get(self) -> Self::Output {{ T }}
}}
*/
",
                tw = tw,
                ty = ty
            ))
            .unwrap();
    }
}
