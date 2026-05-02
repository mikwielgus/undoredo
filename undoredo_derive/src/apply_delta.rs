// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Member};

pub(crate) fn expand_apply_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident.clone();
    let half_delta_name = crate::half_delta::resolve_half_delta_ident(&input)?;

    let mut apply_stmts = Vec::new();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => {
                for field in &fields_named.named {
                    let field_member =
                        Member::Named(field.ident.clone().expect("named field must have ident"));
                    apply_stmts.push(quote! {
                        let field_delta = ::undoredo::Delta::with_removed_inserted(
                            removed.#field_member,
                            inserted.#field_member,
                        );
                        ::undoredo::ApplyDelta::apply_delta(&mut self.#field_member, field_delta);
                    });
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                for (i, _field) in fields_unnamed.unnamed.iter().enumerate() {
                    let field_member = Member::Unnamed(Index::from(i));
                    apply_stmts.push(quote! {
                        let field_delta = ::undoredo::Delta::with_removed_inserted(
                            removed.#field_member,
                            inserted.#field_member,
                        );
                        ::undoredo::ApplyDelta::apply_delta(&mut self.#field_member, field_delta);
                    });
                }
            }
            Fields::Unit => {}
        },
        Data::Enum(_) => {
            let output = quote! {
                impl #impl_generics ::undoredo::ApplyDelta<::std::collections::BTreeMap<usize, #name #ty_generics>>
                    for #name #ty_generics
                #where_clause
                {
                    fn apply_delta(
                        &mut self,
                        delta: ::undoredo::Delta<::std::collections::BTreeMap<usize, #name #ty_generics>>,
                    ) {
                        let (_removed, mut inserted) = delta.dissolve();
                        if let Some(value) = inserted.remove(&0) {
                            *self = value;
                        }
                    }
                }
            };
            return Ok(output.into());
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "derive(ApplyDelta) does not support unions",
            ));
        }
    };

    let output = quote! {
        impl #impl_generics ::undoredo::ApplyDelta<#half_delta_name #ty_generics> for #name #ty_generics
        #where_clause
        {
            fn apply_delta(&mut self, delta: ::undoredo::Delta<#half_delta_name #ty_generics>) {
                let (removed, inserted) = delta.dissolve();
                #(#apply_stmts)*
            }
        }
    };
    Ok(output.into())
}
