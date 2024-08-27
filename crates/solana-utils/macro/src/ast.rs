#![allow(dead_code)]

use proc_macro2::{Ident, Span};
use syn::{Data, DataEnum, DataStruct, DataUnion, DeriveInput, Generics, Index, Member, Type};

pub enum Input<'a> {
    Struct(Struct<'a>),
    Enum(Enum<'a>),
    Union(Union<'a>),
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

pub struct Union<'a> {
    pub ident: Ident,
    pub fields: Vec<Field<'a>>,
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
    pub fn from_syn(node: &'a DeriveInput) -> Self {
        match &node.data {
            Data::Struct(data) => Input::Struct(Struct::from_syn(node, data)),
            Data::Enum(data) => Input::Enum(Enum::from_syn(node, data)),
            Data::Union(data) => Input::Union(Union::from_syn(node, data)),
        }
    }
}

impl<'a> Struct<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Self {
        Struct {
            ident: node.ident.clone(),
            generics: &node.generics,
            fields: Field::multiple_from_syn(&data.fields, Span::call_site()),
        }
    }
}

impl<'a> Enum<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataEnum) -> Self {
        let span = Span::call_site();
        let variants = data.variants.iter().map(|node| Variant::from_syn(node, span)).collect();

        Enum { ident: node.ident.clone(), generics: &node.generics, variants }
    }
}

impl<'a> Union<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataUnion) -> Self {
        Union {
            ident: node.ident.clone(),
            fields: Field::multiple_from_syn(&data.fields.named, Span::call_site()),
        }
    }
}

impl<'a> Variant<'a> {
    fn from_syn(node: &'a syn::Variant, span: Span) -> Self {
        Variant {
            original: node,
            ident: node.ident.clone(),
            fields: Field::multiple_from_syn(&node.fields, span),
        }
    }
}

impl<'a> Field<'a> {
    fn multiple_from_syn<I>(fields: I, span: Span) -> Vec<Self>
    where
        I: IntoIterator<Item = &'a syn::Field>,
    {
        fields.into_iter().enumerate().map(|(i, field)| Field::from_syn(i, field, span)).collect()
    }

    fn from_syn(i: usize, node: &'a syn::Field, span: Span) -> Self {
        Field {
            original: node,
            member: node
                .ident
                .clone()
                .map(Member::Named)
                .unwrap_or_else(|| Member::Unnamed(Index { index: i as u32, span })),
            ty: &node.ty,
        }
    }
}
