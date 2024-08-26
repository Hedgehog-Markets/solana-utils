use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Ident, Member};

use crate::ast::{Field, Variant};

impl<'a> Variant<'a> {
    pub fn destructured(&self) -> (TokenStream, Vec<Field<'a>>) {
        let mut destructured = Vec::with_capacity(self.fields.len());
        let mut mapped = Vec::with_capacity(self.fields.len());

        for (i, field) in self.fields.iter().enumerate() {
            let member = &field.member;
            let mapped_ident = Ident::new(&format!("__{i}"), member.span());

            destructured.push(quote! { #member: #mapped_ident });

            mapped.push(Field {
                original: field.original,
                member: Member::Named(mapped_ident),
                ty: field.ty,
            });
        }

        let variant_ident = &self.ident;

        (quote! { #variant_ident { #(#destructured),* } }, mapped)
    }
}

impl<'a> Field<'a> {
    pub fn span(&self) -> Span {
        let vis_span = self.original.vis.span();
        let ty_span = self.ty.span();

        vis_span.join(ty_span).unwrap_or(ty_span)
    }
}
