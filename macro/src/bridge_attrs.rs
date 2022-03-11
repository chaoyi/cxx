use proc_macro2::TokenStream;

use crate::syntax::namespace::Namespace;
use syn::parse::{Result, Parser};

pub(super) fn parse_bridge_attrs(args: TokenStream) -> Result<(Namespace,Option<TokenStream>)> {
    Namespace::parse_bridge_attr_namespace.parse(args).map(|ns| (ns, None))
}