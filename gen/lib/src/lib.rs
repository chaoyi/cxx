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

mod gen;
mod syntax;

use crate::gen::Opt;
use proc_macro2::TokenStream;

pub fn generate_header_and_cc(rust_source: TokenStream) -> Result<(Vec<u8>, Vec<u8>),String> {
    gen::do_generate_from_tokens(rust_source, Opt::default()).map_err(|e| format!("{}", e))
}
