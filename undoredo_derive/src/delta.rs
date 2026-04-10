// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

use crate::{apply_delta, flush_delta, half_delta};

pub(crate) fn expand_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = match &input.data {
        Data::Struct(_) => {
            let half_id = half_delta::resolve_half_delta_ident(&input)?;
            let delta_alias_ident = format_ident!("{}Delta", name);
            let vis = &input.vis;
            let half_delta = TokenStream2::from(half_delta::expand_half_delta(input.clone())?);
            let delta_alias = quote! {
                #vis type #delta_alias_ident = ::undoredo::Delta<#half_id> #where_clause;
            };
            let apply_delta = TokenStream2::from(apply_delta::expand_apply_delta(input.clone())?);
            let flush_delta = TokenStream2::from(flush_delta::expand_flush_delta(input)?);
            quote! {
                #half_delta
                #delta_alias
                #apply_delta
                #flush_delta
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
