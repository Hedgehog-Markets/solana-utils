extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use syn::{parse_macro_input, DeriveInput};

mod ast;
mod expand;
mod props;
mod valid;

type Result<T, E = Diagnostic> = std::result::Result<T, E>;

#[proc_macro_derive(BorshSize)]
pub fn derive_borsh_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input).into()
}
