use proc_macro2::{Span, TokenStream};
use proc_macro2_diagnostics::Diagnostic;
use quote::{quote, quote_spanned};
use syn::DeriveInput;

use crate::Result;
use crate::ast::{Enum, Field, Input, Struct};

pub fn derive(input: &DeriveInput) -> TokenStream {
    match try_expand(input) {
        Ok(expanded) => expanded,
        // If there are any errors, emit a fallback dummy implementation of BorshSize
        // to minimise spurious knock-on errors in other code that uses the type's BorshSize.
        Err(error) => fallback(input, error),
    }
}

fn try_expand(input: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(input)?;

    input.validate()?;

    Ok(match input {
        Input::Struct(input) => impl_struct(input),
        Input::Enum(input) => impl_enum(input),
    })
}

fn fallback(input: &DeriveInput, error: Diagnostic) -> TokenStream {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let error = error.emit_as_item_tokens();

    quote! {
        #error

        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl #impl_generics ::borsh_size::BorshSize for #ty #ty_generics #where_clause {
            const MIN_SIZE: usize = 0;
            const MAX_SIZE: ::core::option::Option<usize> = ::core::option::Option::None;

            fn borsh_size(&self) -> usize {
                ::core::unreachable!()
            }
        }
    }
}

fn impl_struct(input: Struct) -> TokenStream {
    let min_size = input.fields.min_size();
    let max_size = input.fields.max_size();
    let borsh_size = input.fields.borsh_size(Some(quote! { &self. }));

    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl #impl_generics ::borsh_size::BorshSize for #ty #ty_generics #where_clause {
            const MIN_SIZE: usize = #min_size;
            const MAX_SIZE: ::core::option::Option<usize> = #max_size;

            fn borsh_size(&self) -> usize {
                #borsh_size
            }
        }
    }
}

fn impl_enum(input: Enum) -> TokenStream {
    let variants_min_sizes = input.variants.iter().map(|variant| variant.fields.min_size());
    let variants_max_sizes = input.variants.iter().map(|variant| variant.fields.max_size());

    let min_size = quote! {{
        let mut min_variant_size = usize::MAX;

        #({
            let variant_size = #variants_min_sizes;
            if variant_size < min_variant_size {
                min_variant_size = variant_size;
            }
        })*

        1 + min_variant_size
    }};

    let max_size = quote! {{
        let mut max_variant_size = ::core::option::Option::Some(0);

        #({
            let variant_size = #variants_max_sizes;

            match (max_variant_size, variant_size) {
                (::core::option::Option::Some(max), ::core::option::Option::Some(size)) => if size > max {
                    max_variant_size = ::core::option::Option::Some(size);
                }
                (_, ::core::option::Option::None) => {
                    max_variant_size = ::core::option::Option::None;
                }
                _ => {}
            }
        })*

        match max_variant_size {
            Some(max) => Some(1 + max),
            None => None,
        }
    }};

    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let borsh_size = {
        let borsh_size_branches = input.variants.iter().map(|variant| {
            let (destructured, fields) = variant.destructured();

            let borsh_size = fields.borsh_size(None);

            quote! { #ty::#destructured => { #borsh_size } }
        });

        quote! {
            1 + match self {
                #(#borsh_size_branches)*
            }
        }
    };

    quote! {
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl #impl_generics ::borsh_size::BorshSize for #ty #ty_generics #where_clause {
            const MIN_SIZE: usize = #min_size;
            const MAX_SIZE: ::core::option::Option<usize> = #max_size;

            fn borsh_size(&self) -> usize {
                #borsh_size
            }
        }
    }
}

trait FieldsSizes {
    fn min_size(&self) -> TokenStream;
    fn max_size(&self) -> TokenStream;
    fn borsh_size(&self, member_prefix: Option<TokenStream>) -> TokenStream;
}

impl FieldsSizes for [Field<'_>] {
    fn min_size(&self) -> TokenStream {
        if self.is_empty() {
            return quote! { 0 };
        }

        let min_sizes = self.iter().map(|field| {
            let field_ty = field.ty;
            let span = field.span().resolved_at(Span::call_site());

            quote_spanned! { span =>
                <#field_ty as ::borsh_size::BorshSize>::MIN_SIZE
            }
        });

        quote! { #(#min_sizes)+* }
    }

    fn max_size(&self) -> TokenStream {
        if self.is_empty() {
            return quote! { ::core::option::Option::Some(0) };
        }

        let max_sizes = self.iter().map(|field| {
            let field_ty = field.ty;
            let span = field.span().resolved_at(Span::call_site());

            quote_spanned! { span =>
                <#field_ty as ::borsh_size::BorshSize>::MAX_SIZE
            }
        });

        quote! {{
            let sum = ::core::option::Option::Some(0);

            #(
                let sum = match (sum, #max_sizes) {
                    (::core::option::Option::Some(sum), ::core::option::Option::Some(size)) => ::core::option::Option::Some(sum + size),
                    _ => ::core::option::Option::None,
                };
            )*

            sum
        }}
    }

    fn borsh_size(&self, member_prefix: Option<TokenStream>) -> TokenStream {
        if self.is_empty() {
            return quote! { 0 };
        }

        let borsh_sizes = self.iter().map(|field| {
            let member = &field.member;
            let span = field.span().resolved_at(Span::call_site());

            quote_spanned! { span =>
                ::borsh_size::BorshSize::borsh_size(#member_prefix #member)
            }
        });

        quote! {
            if <Self as ::borsh_size::BorshSizeProperties>::IS_FIXED_SIZE {
                Self::MIN_SIZE
            } else {
                let mut size = 0;
                #(size += #borsh_sizes;)*
                size
            }
        }
    }
}
