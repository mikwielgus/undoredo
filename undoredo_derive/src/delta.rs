// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

use crate::{apply_delta, discard_delta, flush_delta, half_delta};

fn resolve_delta_ident(input: &DeriveInput) -> syn::Result<syn::Ident> {
    let default = format_ident!("{}Delta", input.ident);
    let mut name_from_attr: Option<syn::Ident> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("undoredo") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("half_delta") {
                    let _: syn::Ident = meta.value()?.parse()?;
                    return Ok(());
                }

                if meta.path.is_ident("delta") {
                    if name_from_attr.is_some() {
                        return Err(meta.error("duplicate `delta` in #[undoredo(...)]"));
                    }

                    let ident: syn::Ident = meta.value()?.parse()?;
                    name_from_attr = Some(ident);

                    return Ok(());
                }

                Err(meta.error("unrecognized undoredo attribute key"))
            })?;
        }
    }

    Ok(name_from_attr.unwrap_or(default))
}

pub(crate) fn expand_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let generics = input.generics.clone();

    let output = match &input.data {
        Data::Struct(_) => {
            let half_id = half_delta::resolve_half_delta_ident(&input)?;
            let delta_alias_ident = resolve_delta_ident(&input)?;
            let vis = &input.vis;
            let half_delta = TokenStream2::from(half_delta::expand_half_delta(input.clone())?);
            let delta_alias = quote! {
                #vis type #delta_alias_ident #generics = ::undoredo::Delta<#half_id #ty_generics>;
            };
            let apply_delta = TokenStream2::from(apply_delta::expand_apply_delta(input.clone())?);
            let flush_delta = TokenStream2::from(flush_delta::expand_flush_delta(input.clone())?);
            let discard_delta = TokenStream2::from(discard_delta::expand_discard_delta(input)?);
            quote! {
                #half_delta
                #delta_alias
                #apply_delta
                #flush_delta
                #discard_delta
            }
        }
        Data::Enum(_) => {
            let apply = TokenStream2::from(apply_delta::expand_apply_delta(input.clone())?);
            quote! {
                impl #impl_generics ::maplike::Container for #name #ty_generics #where_clause {
                    type Key = usize;
                    type Value = Self;
                }

                impl #impl_generics ::maplike::Assign for #name #ty_generics #where_clause {
                    fn assign(&mut self, value: Self) {
                        *self = value;
                    }
                }

                #apply
            }
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "derive(Delta) does not support unions",
            ));
        }
    };

    Ok(output.into())
}
