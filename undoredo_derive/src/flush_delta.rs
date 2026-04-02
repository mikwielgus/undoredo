// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Index};

pub(crate) fn expand_flush_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let mut half_delta_name = format_ident!("{}HalfDelta", name);

    for attr in &input.attrs {
        if attr.path().is_ident("flush_delta") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value: syn::LitStr = meta.value()?.parse()?;
                    half_delta_name = format_ident!("{}", value.value());
                    Ok(())
                } else {
                    Err(meta.error("unsupported flush_delta attribute key"))
                }
            })?;
        }
    }

    let mut flush_stmts = Vec::new();

    let (removed_ctor, inserted_ctor) = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => {
                let mut removed_fields = Vec::new();
                let mut inserted_fields = Vec::new();

                for field in &fields_named.named {
                    let field_ident = field.ident.as_ref().expect("named field must have ident");
                    let removed_ident = format_ident!("removed_{}", field_ident);
                    let inserted_ident = format_ident!("inserted_{}", field_ident);

                    flush_stmts.push(quote! {
                        let (#removed_ident, #inserted_ident) =
                            ::undoredo::FlushDelta::flush_delta(&mut self.#field_ident).dissolve();
                    });

                    removed_fields.push(quote! { #field_ident: #removed_ident });
                    inserted_fields.push(quote! { #field_ident: #inserted_ident });
                }

                (
                    quote! { #half_delta_name { #(#removed_fields),* } },
                    quote! { #half_delta_name { #(#inserted_fields),* } },
                )
            }
            Fields::Unnamed(fields_unnamed) => {
                let mut removed_fields = Vec::new();
                let mut inserted_fields = Vec::new();

                for (index, _field) in fields_unnamed.unnamed.iter().enumerate() {
                    let field_index = Index::from(index);
                    let removed_ident = format_ident!("removed_{}", index);
                    let inserted_ident = format_ident!("inserted_{}", index);

                    flush_stmts.push(quote! {
                        let (#removed_ident, #inserted_ident) =
                            ::undoredo::FlushDelta::flush_delta(&mut self.#field_index).dissolve();
                    });

                    removed_fields.push(quote! { #removed_ident });
                    inserted_fields.push(quote! { #inserted_ident });
                }

                (
                    quote! { #half_delta_name( #(#removed_fields),* ) },
                    quote! { #half_delta_name( #(#inserted_fields),* ) },
                )
            }
            Fields::Unit => (quote! { #half_delta_name }, quote! { #half_delta_name }),
        },
        Data::Enum(_) => (
            quote! { self.clone() },
            quote! { self.clone() },
        ),
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "FlushDelta does not support unions",
            ));
        }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        impl #impl_generics ::undoredo::FlushDelta<#half_delta_name> for #name #ty_generics
        #where_clause
        {
            fn flush_delta(&mut self) -> ::undoredo::Delta<#half_delta_name> {
                #(#flush_stmts)*
                ::undoredo::Delta::with_removed_inserted(
                    #removed_ctor,
                    #inserted_ctor,
                )
            }
        }
    };

    Ok(output.into())
}
