//! Details of the `request` section of the procedural macro.

use std::collections::btree_map::{BTreeMap, Entry};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, visit::Visit, Attribute, Field, Ident,
    Lifetime, Token,
};

use super::{
    api_metadata::Metadata,
    kw,
    util::{all_cfgs, extract_cfg},
};

/// The result of processing the `request` section of the macro.
pub(crate) struct Request {
    /// The `request` keyword
    pub(super) request_kw: kw::request,

    /// The attributes that will be applied to the struct definition.
    pub(super) attributes: Vec<Attribute>,

    /// The fields of the request.
    pub(super) fields: Punctuated<Field, Token![,]>,
}

impl Request {
    /// The combination of every fields unique lifetime annotation.
    fn all_lifetimes(&self) -> BTreeMap<Lifetime, Option<Attribute>> {
        let mut lifetimes = BTreeMap::new();

        struct Visitor<'lt> {
            field_cfg: Option<Attribute>,
            lifetimes: &'lt mut BTreeMap<Lifetime, Option<Attribute>>,
        }

        impl<'ast> Visit<'ast> for Visitor<'_> {
            fn visit_lifetime(&mut self, lt: &'ast Lifetime) {
                match self.lifetimes.entry(lt.clone()) {
                    Entry::Vacant(v) => {
                        v.insert(self.field_cfg.clone());
                    }
                    Entry::Occupied(mut o) => {
                        let lifetime_cfg = o.get_mut();

                        // If at least one field uses this lifetime and has no cfg attribute, we
                        // don't need a cfg attribute for the lifetime either.
                        *lifetime_cfg = Option::zip(lifetime_cfg.as_ref(), self.field_cfg.as_ref())
                            .map(|(a, b)| {
                                let expr_a = extract_cfg(a);
                                let expr_b = extract_cfg(b);
                                parse_quote! { #[cfg( any( #expr_a, #expr_b ) )] }
                            });
                    }
                }
            }
        }

        for field in &self.fields {
            let field_cfg = if field.attrs.is_empty() { None } else { all_cfgs(&field.attrs) };
            Visitor { lifetimes: &mut lifetimes, field_cfg }.visit_type(&field.ty);
        }

        lifetimes
    }

    pub(super) fn expand(
        &self,
        metadata: &Metadata,
        error_ty: &TokenStream,
        ruma_common: &TokenStream,
    ) -> TokenStream {
        let ruma_macros = quote! { #ruma_common::exports::ruma_macros };

        let docs = format!(
            "Data for a request to the `{}` API endpoint.\n\n{}",
            metadata.name.value(),
            metadata.description.value(),
        );
        let struct_attributes = &self.attributes;

        let request_ident = Ident::new("Request", self.request_kw.span());
        let lifetimes = self.all_lifetimes();
        let lifetimes = lifetimes.iter().map(|(lt, attr)| quote! { #attr #lt });
        let fields = &self.fields;

        quote! {
            #[doc = #docs]
            #[#ruma_macros::request(error = #error_ty)]
            #( #struct_attributes )*
            pub struct #request_ident < #(#lifetimes),* > {
                #fields
            }
        }
    }
}
