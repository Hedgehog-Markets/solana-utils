use proc_macro2::Span;
use proc_macro2_diagnostics::SpanDiagnosticExt;
use syn::spanned::Spanned;

use crate::ast::{Enum, Input};
use crate::Result;

impl Input<'_> {
    pub fn validate(&self) -> Result<()> {
        match self {
            Input::Struct(_) => Ok(()),
            Input::Enum(input) => input.validate(),
        }
    }
}

impl Enum<'_> {
    pub fn validate(&self) -> Result<()> {
        if self.variants.is_empty() {
            return Err(
                Span::call_site().error("this trait does not support enums with no variants")
            );
        }

        if let Some(variant) = self.variants.get(256) {
            return Err(Span::call_site()
                .error("this trait does not support enums with more than 256 variants")
                .span_help(variant.original.span(), "this is the 257th variant"));
        }

        Ok(())
    }
}
