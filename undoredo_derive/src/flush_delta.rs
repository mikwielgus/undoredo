// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Index};

pub(crate) fn expand_flush_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident.clone();
    let half_delta = crate::half_delta::resolve_half_delta_ident(&input)?;

    let mut flush_stmts = Vec::new();

    // Besides inheriting the trait bounds from the input type, we also need to
    // also require having `FlushDelta` implemented for all fields.
    let mut extra_where_predicates = Vec::new();

    let (removed_ctor, inserted_ctor) = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => {
                let mut removed_fields = Vec::new();
                let mut inserted_fields = Vec::new();

                for field in &fields_named.named {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let field_ident = field.ident.as_ref().expect("named field must have ident");
                    let field_ty = &field.ty;
                    let field_half_delta_ty =
                        crate::half_delta::field_to_half_delta_container(field_ty, &input.generics);

                    let removed_ident = format_ident!("removed_{}", field_ident);
                    let inserted_ident = format_ident!("inserted_{}", field_ident);

                    extra_where_predicates.push(quote! {
                        #field_ty: ::undoredo::FlushDelta<#field_half_delta_ty>
                    });

                    flush_stmts.push(quote! {
                        let (#removed_ident, #inserted_ident) =
                            ::undoredo::FlushDelta::flush_delta(&mut self.#field_ident).dissolve();
                    });

                    removed_fields.push(quote! { #field_ident: #removed_ident });
                    inserted_fields.push(quote! { #field_ident: #inserted_ident });
                }

                (
                    quote! { #half_delta { #(#removed_fields),* } },
                    quote! { #half_delta { #(#inserted_fields),* } },
                )
            }
            Fields::Unnamed(fields_unnamed) => {
                let mut removed_fields = Vec::new();
                let mut inserted_fields = Vec::new();

                let mut half_field_index = 0usize;

                for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let field_index = Index::from(i);
                    let field_ty = &field.ty;
                    let field_half_delta_ty =
                        crate::half_delta::field_to_half_delta_container(field_ty, &input.generics);

                    let removed_ident = format_ident!("removed_{}", half_field_index);
                    let inserted_ident = format_ident!("inserted_{}", half_field_index);

                    // Increment only for non-skipped fields.
                    half_field_index += 1;

                    extra_where_predicates.push(quote! {
                        #field_ty: ::undoredo::FlushDelta<#field_half_delta_ty>
                    });

                    flush_stmts.push(quote! {
                        let (#removed_ident, #inserted_ident) =
                            ::undoredo::FlushDelta::flush_delta(&mut self.#field_index).dissolve();
                    });

                    removed_fields.push(quote! { #removed_ident });
                    inserted_fields.push(quote! { #inserted_ident });
                }

                (
                    quote! { #half_delta( #(#removed_fields),* ) },
                    quote! { #half_delta( #(#inserted_fields),* ) },
                )
            }
            Fields::Unit => (quote! { #half_delta }, quote! { #half_delta }),
        },
        Data::Enum(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "derive(FlushDelta) does not support enums",
            ));
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "derive(FlushDelta) does not support unions",
            ));
        }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut where_predicates: Vec<proc_macro2::TokenStream> = where_clause
        .map(|clause| {
            clause
                .predicates
                .iter()
                .map(|pred| quote! { #pred })
                .collect()
        })
        .unwrap_or_default();

    // Add the additional `ApplyDelta` trait bounds for every field.
    where_predicates.extend(extra_where_predicates);

    // We don't want to emit any tokens if there is no predicates.
    let where_tokens = if where_predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_predicates,)* }
    };

    let output = quote! {
        impl #impl_generics ::undoredo::FlushDelta<#half_delta #ty_generics> for #name #ty_generics
        #where_tokens
        {
            fn flush_delta(&mut self) -> ::undoredo::Delta<#half_delta #ty_generics> {
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
