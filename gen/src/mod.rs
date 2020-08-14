// Functionality that is shared between the cxx_build::bridge entry point and
// the cxxbridge CLI command.

mod error;
mod find;
pub(super) mod include;
pub(super) mod out;
mod write;

#[cfg(test)]
mod tests;

use self::error::{format_err, Error, Result};
use crate::syntax::namespace::Namespace;
use crate::syntax::report::Errors;
use crate::syntax::{self, check, Types};
use std::fs;
use std::clone::Clone;
use std::path::Path;
use syn::{File, Item};
use proc_macro2::TokenStream;

struct Input {
    namespace: Namespace,
    module: Vec<Item>,
}

#[derive(Default,Clone)]
pub(super) struct Opt {
    /// Any additional headers to #include
    pub include: Vec<String>,
    /// Whether to set __attribute__((visibility("default")))
    /// or similar annotations on function implementations.
    pub cxx_impl_annotations: Option<String>,
}

pub(super) fn do_generate_bridge(path: &Path, opt: Opt) -> Vec<u8> {
    let header = false;
    generate_from_path(path, opt, header)
}

pub(super) fn do_generate_header(path: &Path, opt: Opt) -> Vec<u8> {
    let header = true;
    generate_from_path(path, opt, header)
}

pub(super) fn do_generate_from_tokens(tokens: TokenStream, opt: Opt) -> std::result::Result<(Vec<u8>,Vec<u8>),String> {
    let syntax = syn::parse2::<File>(tokens).map_err(|e| format!("{}", e))?;
    match generate(syntax, opt, true, true) {
        Ok((Some(hdr), Some(cxx))) => Ok((hdr, cxx)),
        Err(err) => Err(format!("{}", err)),
        _ => panic!("Unexpected generation")
    }
}

fn generate_from_path(path: &Path, opt: Opt, header: bool) -> Vec<u8> {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => format_err(path, "", Error::Io(err)),
    };
    let syntax = match syn::parse_file(&source) {
        Ok(out) => out,
        Err(err) => format_err(path, "", Error::Syn(err)),
    };
    match generate(syntax, opt, header, !header) {
        Ok((Some(hdr), None)) => hdr,
        Ok((None, Some(cxx))) => cxx,
        Err(err) => format_err(path, &source, err),
        _ => panic!("Unexpected generation"),
    }
}

fn generate(syntax: File, opt: Opt, gen_header: bool, gen_cxx: bool) -> Result<(Option<Vec<u8>>,Option<Vec<u8>>)> {
    proc_macro2::fallback::force();
    let ref mut errors = Errors::new();
    let bridge = find::find_bridge_mod(syntax)?;
    let ref namespace = bridge.namespace;
    let ref apis = syntax::parse_items(errors, bridge.module);
    let ref types = Types::collect(errors, apis);
    errors.propagate()?;
    check::typecheck(errors, namespace, apis, types);
    errors.propagate()?;
    let hdr = if gen_header {
        Some(write::gen(namespace, apis, types, opt.clone(), true).content())
    } else {
        None
    };
    let cxx = if gen_cxx {
        Some(write::gen(namespace, apis, types, opt, false).content())
    } else {
        None
    };
    Ok((hdr, cxx))
}
