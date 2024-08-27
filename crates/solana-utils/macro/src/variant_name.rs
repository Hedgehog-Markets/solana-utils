use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error};

use crate::ast::{Enum, Input};

pub fn derive(input: &DeriveInput) -> TokenStream {
    let msg = match Input::from_syn(input) {
        Input::Enum(input) => return impl_enum(input),
        Input::Struct(_) => "this trait cannot be derived for structs",
        Input::Union(_) => "this trait cannot be derived for unions",
    };
    Error::new(Span::call_site(), msg).to_compile_error()
}

fn impl_enum(input: Enum) -> TokenStream {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let variants = input.variants.iter().map(|variant| {
        let variant = &variant.ident;
        let name = variant.to_string();

        quote! { #ty::#variant { .. } => { #name } }
    });

    quote! {
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl #impl_generics ::solana_utils::VariantName for #ty #ty_generics #where_clause {
            fn variant_name(&self) -> &'static str {
                match self {
                    #(#variants)*
                }
            }
        }
    }
}
