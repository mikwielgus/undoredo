// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Index, Member};

pub(crate) fn expand_apply_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let mut half_delta_name = format_ident!("{}HalfDelta", name);

    for attr in &input.attrs {
        if attr.path().is_ident("apply_delta") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value: syn::LitStr = meta.value()?.parse()?;
                    half_delta_name = format_ident!("{}", value.value());
                    Ok(())
                } else {
                    Err(meta.error("unsupported apply_delta attribute key"))
                }
            })?;
        }
    }

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
                        ::undoredo::ApplyDelta::apply_delta(&mut self.#field_member, &field_delta);
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
                        ::undoredo::ApplyDelta::apply_delta(&mut self.#field_member, &field_delta);
                    });
                }
            }
            Fields::Unit => {}
        },
        Data::Enum(_data) => {
            let output = quote! {
                impl #impl_generics ::undoredo::ApplyDelta<#half_delta_name> for #name #ty_generics
                #where_clause
                {
                    fn apply_delta(&mut self, delta: &::undoredo::Delta<#half_delta_name>) {
                        let (_, inserted) = delta.clone().dissolve();
                        *self = inserted.clone();
                    }
                }
            };
            return Ok(output.into());
        }
        _ => (),
    };

    let output = quote! {
        impl #impl_generics ::undoredo::ApplyDelta<#half_delta_name> for #name #ty_generics
        #where_clause
        {
            fn apply_delta(&mut self, delta: &::undoredo::Delta<#half_delta_name>) {
                let (removed, inserted) = delta.clone().dissolve();
                #(#apply_stmts)*
            }
        }
    };
    Ok(output.into())
}
