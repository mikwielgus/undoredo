// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::{apply_delta, flush_delta, half_delta};

pub(crate) fn expand_undoredo(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = match &input.data {
        Data::Struct(_) => {
            let half = TokenStream2::from(half_delta::expand_half_delta(input.clone())?);
            let apply = TokenStream2::from(apply_delta::expand_apply_delta(input.clone())?);
            let flush = TokenStream2::from(flush_delta::expand_flush_delta(input)?);
            quote! {
                #half
                #apply
                #flush
            }
        }
        Data::Enum(_) => {
            quote! {
                impl #impl_generics ::maplike::Assign for #name #ty_generics #where_clause {
                    fn assign(&mut self, value: Self) {
                        *self = value;
                    }
                }
            }
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "derive(UndoRedo) does not support unions",
            ));
        }
    };

    Ok(output.into())
}
