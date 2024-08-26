use proc_macro2::{Ident, Span};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Generics, Index, Member, Type};

use crate::Result;

pub enum Input<'a> {
    Struct(Struct<'a>),
    Enum(Enum<'a>),
}

pub struct Struct<'a> {
    pub ident: Ident,
    pub generics: &'a Generics,
    pub fields: Vec<Field<'a>>,
}

pub struct Enum<'a> {
    pub ident: Ident,
    pub generics: &'a Generics,
    pub variants: Vec<Variant<'a>>,
}

pub struct Variant<'a> {
    pub original: &'a syn::Variant,
    pub ident: Ident,
    pub fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub member: Member,
    pub ty: &'a Type,
}

impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            Data::Enum(data) => Enum::from_syn(node, data).map(Input::Enum),
            Data::Union(_) => {
                Err(Span::call_site().error("this trait cannot be derived for unions"))
            }
        }
    }
}

impl<'a> Struct<'a> {
    pub fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Result<Self> {
        Ok(Struct {
            ident: node.ident.clone(),
            generics: &node.generics,
            fields: Field::multiple_from_syn(&data.fields, Span::call_site())?,
        })
    }
}

impl<'a> Enum<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataEnum) -> Result<Self> {
        let span = Span::call_site();
        let variants = data
            .variants
            .iter()
            .map(|node| Variant::from_syn(node, span))
            .collect::<Result<_>>()?;
        Ok(Enum { ident: node.ident.clone(), generics: &node.generics, variants })
    }
}

impl<'a> Variant<'a> {
    fn from_syn(node: &'a syn::Variant, span: Span) -> Result<Self> {
        Ok(Variant {
            original: node,
            ident: node.ident.clone(),
            fields: Field::multiple_from_syn(&node.fields, span)?,
        })
    }
}

impl<'a> Field<'a> {
    fn multiple_from_syn(fields: &'a Fields, span: Span) -> Result<Vec<Self>> {
        fields.iter().enumerate().map(|(i, field)| Field::from_syn(i, field, span)).collect()
    }

    fn from_syn(i: usize, node: &'a syn::Field, span: Span) -> Result<Self> {
        Ok(Field {
            original: node,
            member: node
                .ident
                .clone()
                .map(Member::Named)
                .unwrap_or_else(|| Member::Unnamed(Index { index: i as u32, span })),
            ty: &node.ty,
        })
    }
}
