//! The CXX code generator for constructing and compiling C++ code.
//!
//! This is intended to be embedded into higher-level code generators.
//! ```

#![allow(
    clippy::inherent_to_string,
    clippy::needless_doctest_main,
    clippy::new_without_default,
    clippy::toplevel_ref_arg
)]

mod error;
mod gen;
mod paths;
mod syntax;

use crate::gen::Opt;
use std::path::Path;

pub fn generate_header_and_cc(rust_source_file: &Path) -> (Vec<u8>, Vec<u8>) {
    let header = gen::do_generate_header(rust_source_file, Opt::default());
    let bridge = gen::do_generate_bridge(rust_source_file, Opt::default());
    (header, bridge)
}
