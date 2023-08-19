#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macor_use]
extern crate std;
fn main() {{ ::std::io::_print(format_args!("Hello, World!\n")); }; }
