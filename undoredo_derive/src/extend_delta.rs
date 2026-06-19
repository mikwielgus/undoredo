// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Member};

pub(crate) fn expand_extend_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident.clone();
    let half_delta = crate::half_delta::resolve_half_delta_ident(&input)?;

    let mut extend_stmts = Vec::new();

    // Besides inheriting the trait bounds from the input type, we also need to
    // also require having `ExtendDelta` implemented for all fields.
    let mut extra_where_predicates = Vec::new();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => {
                for field in &fields_named.named {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let field_ty = &field.ty;
                    let field_half_delta_ty =
                        crate::half_delta::field_to_half_delta_container(field_ty, &input.generics);
                    let field_member =
                        Member::Named(field.ident.clone().expect("named field must have ident"));

                    extra_where_predicates.push(quote! {
                        #field_ty: ::undoredo::ExtendDelta<#field_half_delta_ty>
                    });

                    extend_stmts.push(quote! {
                        let field_delta = ::undoredo::Delta::with_removed_inserted(
                            removed.#field_member,
                            inserted.#field_member,
                        );
                        ::undoredo::ExtendDelta::extend_delta(&mut self.#field_member, field_delta);
                    });
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let mut half_field_index = 0usize;

                for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let field_ty = &field.ty;
                    let field_half_delta_ty =
                        crate::half_delta::field_to_half_delta_container(field_ty, &input.generics);

                    let field_member_self = Member::Unnamed(Index::from(i));
                    let field_member_half = Member::Unnamed(Index::from(half_field_index));

                    // Increment only if field is not skipped.
                    half_field_index += 1;

                    extra_where_predicates.push(quote! {
                        #field_ty: ::undoredo::ExtendDelta<#field_half_delta_ty>
                    });

                    extend_stmts.push(quote! {
                        let field_delta = ::undoredo::Delta::with_removed_inserted(
                            removed.#field_member_half,
                            inserted.#field_member_half,
                        );
                        ::undoredo::ExtendDelta::extend_delta(&mut self.#field_member_self, field_delta);
                    });
                }
            }
            Fields::Unit => {}
        },
        Data::Enum(_) => {
            let output = quote! {
                impl #impl_generics ::undoredo::ExtendDelta<::undoredo::alloc::collections::BTreeMap<usize, #name #ty_generics>>
                    for #name #ty_generics
                #where_clause
                {
                    fn extend_delta(
                        &mut self,
                        delta: ::undoredo::Delta<::undoredo::alloc::collections::BTreeMap<usize, #name #ty_generics>>,
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
                "derive(ExtendDelta) does not support unions",
            ));
        }
    };

    let mut where_predicates: Vec<proc_macro2::TokenStream> = where_clause
        .map(|clause| {
            clause
                .predicates
                .iter()
                .map(|pred| quote! { #pred })
                .collect()
        })
        .unwrap_or_default();

    // Add the additional `ExtendDelta` trait bounds for every field.
    where_predicates.extend(extra_where_predicates);

    // We don't want to emit any tokens if there is no predicates.
    let where_tokens = if where_predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_predicates,)* }
    };

    let output = quote! {
        impl #impl_generics ::undoredo::ExtendDelta<#half_delta #ty_generics> for #name #ty_generics
        #where_tokens
        {
            fn extend_delta(&mut self, delta: ::undoredo::Delta<#half_delta #ty_generics>) {
                let (removed, inserted) = delta.dissolve();
                #(#extend_stmts)*
            }
        }
    };
    Ok(output.into())
}
