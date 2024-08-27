extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod ast;
mod variant_name;

#[proc_macro_derive(VariantName)]
pub fn derive_variant_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    variant_name::derive(&input).into()
}
