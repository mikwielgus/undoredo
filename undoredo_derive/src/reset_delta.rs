// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index};

pub(crate) fn expand_reset_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident.clone();

    let mut reset_stmts = Vec::new();

    // Besides inheriting the trait bounds from the input type, we also need to
    // also require having `ResetDelta` implemented for all fields.
    let mut extra_where_predicates = Vec::new();

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => {
                for field in &fields_named.named {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let field_ident = field.ident.as_ref().expect("named field must have ident");
                    let field_ty = &field.ty;
                    let field_half_delta_ty =
                        crate::half_delta::field_to_half_delta_container(field_ty, &input.generics);

                    extra_where_predicates.push(quote! {
                        #field_ty: ::undoredo::ResetDelta<#field_half_delta_ty>
                    });

                    reset_stmts.push(quote! {
                        ::undoredo::ResetDelta::<#field_half_delta_ty>::reset_delta(&mut self.#field_ident);
                    });
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let field_index = Index::from(i);
                    let field_ty = &field.ty;
                    let field_half_delta_ty =
                        crate::half_delta::field_to_half_delta_container(field_ty, &input.generics);

                    extra_where_predicates.push(quote! {
                        #field_ty: ::undoredo::ResetDelta<#field_half_delta_ty>
                    });

                    reset_stmts.push(quote! {
                        ::undoredo::ResetDelta::<#field_half_delta_ty>::reset_delta(&mut self.#field_index);
                    });
                }
            }
            Fields::Unit => {}
        },
        Data::Enum(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "derive(ResetDelta) does not support enums",
            ));
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "derive(ResetDelta) does not support unions",
            ));
        }
    };

    let half_delta = crate::half_delta::resolve_half_delta_ident(&input)?;
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
    where_predicates.extend(extra_where_predicates);

    let where_tokens = if where_predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_predicates,)* }
    };

    let output = quote! {
        impl #impl_generics ::undoredo::ResetDelta<#half_delta #ty_generics> for #name #ty_generics
        #where_tokens
        {
            fn reset_delta(&mut self) {
                #(#reset_stmts)*
            }
        }
    };

    Ok(output.into())
}
