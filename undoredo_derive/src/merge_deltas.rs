// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Index, Member};

pub(crate) fn expand_merge_deltas(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident.clone();
    let half_delta = crate::half_delta::resolve_half_delta_ident(&input)?;

    let mut merge_stmts = Vec::new();
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
                    let field_member = Member::Named(field_ident.clone());

                    let merged_ident = format_ident!("merged_{}", field_ident);
                    let removed_ident = format_ident!("removed_{}", field_ident);
                    let inserted_ident = format_ident!("inserted_{}", field_ident);

                    extra_where_predicates.push(quote! {
                        ::undoredo::Delta<#field_half_delta_ty>: ::undoredo::MergeDeltas<#field_half_delta_ty>
                    });

                    merge_stmts.push(quote! {
                        let #merged_ident = {
                            let self_field_delta = ::undoredo::Delta::with_removed_inserted(
                                self_removed.#field_member,
                                self_inserted.#field_member,
                            );
                            let other_field_delta = ::undoredo::Delta::with_removed_inserted(
                                other_removed.#field_member,
                                other_inserted.#field_member,
                            );
                            ::undoredo::MergeDeltas::merge_deltas(self_field_delta, other_field_delta)
                        };
                        let (#removed_ident, #inserted_ident) = #merged_ident.dissolve();
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

                for field in &fields_unnamed.unnamed {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let field_ty = &field.ty;
                    let field_half_delta_ty =
                        crate::half_delta::field_to_half_delta_container(field_ty, &input.generics);
                    let field_member_half = Member::Unnamed(Index::from(half_field_index));

                    let merged_ident = format_ident!("merged_{}", half_field_index);
                    let removed_ident = format_ident!("removed_{}", half_field_index);
                    let inserted_ident = format_ident!("inserted_{}", half_field_index);

                    half_field_index += 1;

                    extra_where_predicates.push(quote! {
                        ::undoredo::Delta<#field_half_delta_ty>: ::undoredo::MergeDeltas<#field_half_delta_ty>
                    });

                    merge_stmts.push(quote! {
                        let #merged_ident = {
                            let self_field_delta = ::undoredo::Delta::with_removed_inserted(
                                self_removed.#field_member_half,
                                self_inserted.#field_member_half,
                            );
                            let other_field_delta = ::undoredo::Delta::with_removed_inserted(
                                other_removed.#field_member_half,
                                other_inserted.#field_member_half,
                            );
                            ::undoredo::MergeDeltas::merge_deltas(self_field_delta, other_field_delta)
                        };
                        let (#removed_ident, #inserted_ident) = #merged_ident.dissolve();
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
                "derive(MergeDeltas) does not support enums",
            ));
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &name,
                "derive(MergeDeltas) does not support unions",
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

    where_predicates.extend(extra_where_predicates);

    let where_tokens = if where_predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_predicates,)* }
    };

    let output = quote! {
        impl #impl_generics ::undoredo::MergeDeltas<#half_delta #ty_generics>
            for ::undoredo::Delta<#half_delta #ty_generics>
        #where_tokens
        {
            fn merge_deltas(
                self,
                other: ::undoredo::Delta<#half_delta #ty_generics>,
            ) -> Self {
                let (self_removed, self_inserted) = self.dissolve();
                let (other_removed, other_inserted) = other.dissolve();
                #(#merge_stmts)*
                Self::with_removed_inserted(#removed_ctor, #inserted_ctor)
            }
        }
    };

    Ok(output.into())
}
