// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ApplyEdit)]
pub fn derive_apply_edit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_apply_edit(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_apply_edit(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let mut generics = input.generics;

    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "ApplyEdit can only be derived for structs",
            ));
        }
    };

    let where_clause = generics.make_where_clause();
    where_clause.predicates.push(syn::parse_quote! {
        ::undoredo::Edit<Self>: ::core::clone::Clone
    });

    let mut apply_stmts = Vec::new();

    match fields {
        Fields::Named(named) => {
            for field in named.named {
                push_field_apply_stmt(
                    where_clause,
                    &mut apply_stmts,
                    field.ty,
                    syn::Member::Named(field.ident.expect("named field must have ident")),
                );
            }
        }
        Fields::Unnamed(unnamed) => {
            for (idx, field) in unnamed.unnamed.into_iter().enumerate() {
                push_field_apply_stmt(
                    where_clause,
                    &mut apply_stmts,
                    field.ty,
                    syn::Member::Unnamed(syn::Index::from(idx)),
                );
            }
        }
        Fields::Unit => {}
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let output = quote! {
        impl #impl_generics ::undoredo::ApplyEdit<#name #ty_generics> for #name #ty_generics
        #where_clause
        {
            fn apply_edit(&mut self, edit: &::undoredo::Edit<#name #ty_generics>) {
                #[allow(unused_variables)]
                let (removed, inserted) = edit.clone().dissolve();
                #(#apply_stmts)*
            }
        }
    };

    Ok(output.into())
}

fn push_field_apply_stmt(
    where_clause: &mut syn::WhereClause,
    apply_stmts: &mut Vec<proc_macro2::TokenStream>,
    field_ty: syn::Type,
    field_member: syn::Member,
) {
    where_clause.predicates.push(syn::parse_quote! {
        #field_ty: ::undoredo::ApplyEdit<#field_ty> + ::core::clone::Clone
    });
    apply_stmts.push(quote! {
        {
            let field_edit = ::undoredo::Edit::with_removed_inserted(
                removed.#field_member,
                inserted.#field_member,
            );
            ::undoredo::ApplyEdit::apply_edit(&mut self.#field_member, &field_edit);
        }
    });
}

#[proc_macro_derive(FlushEdit)]
pub fn derive_flush_edit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_flush_edit(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_flush_edit(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let mut generics = input.generics;

    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "FlushEdit can only be derived for structs",
            ));
        }
    };

    let where_clause = generics.make_where_clause();
    let mut bindings = Vec::new();
    let mut removed_fields = Vec::new();
    let mut inserted_fields = Vec::new();

    let (removed_ctor, inserted_ctor) = match fields {
        Fields::Named(named) => {
            for field in named.named {
                let field_ident = field.ident.expect("named field must have ident");

                push_field_flush_parts(
                    where_clause,
                    &mut bindings,
                    &mut removed_fields,
                    &mut inserted_fields,
                    field.ty,
                    syn::Member::Named(field_ident.clone()),
                    Some(field_ident.clone()),
                    syn::Ident::new(
                        &format!("removed_{}", field_ident.clone()),
                        proc_macro2::Span::call_site(),
                    ),
                    syn::Ident::new(
                        &format!("inserted_{}", field_ident),
                        proc_macro2::Span::call_site(),
                    ),
                );
            }

            (
                quote! { #name { #(#removed_fields),* } },
                quote! { #name { #(#inserted_fields),* } },
            )
        }
        Fields::Unnamed(unnamed) => {
            for (idx, field) in unnamed.unnamed.into_iter().enumerate() {
                push_field_flush_parts(
                    where_clause,
                    &mut bindings,
                    &mut removed_fields,
                    &mut inserted_fields,
                    field.ty,
                    syn::Member::Unnamed(syn::Index::from(idx)),
                    None,
                    syn::Ident::new(&format!("removed_{}", idx), proc_macro2::Span::call_site()),
                    syn::Ident::new(&format!("inserted_{}", idx), proc_macro2::Span::call_site()),
                );
            }

            (
                quote! { #name ( #(#removed_fields),* ) },
                quote! { #name ( #(#inserted_fields),* ) },
            )
        }
        Fields::Unit => (quote! { #name }, quote! { #name }),
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        impl #impl_generics ::undoredo::FlushEdit<#name #ty_generics> for #name #ty_generics
        #where_clause
        {
            fn flush_edit(&mut self) -> ::undoredo::Edit<#name #ty_generics> {
                #(#bindings)*
                ::undoredo::Edit::with_removed_inserted(#removed_ctor, #inserted_ctor)
            }
        }
    };

    Ok(output.into())
}

fn push_field_flush_parts(
    where_clause: &mut syn::WhereClause,
    bindings: &mut Vec<proc_macro2::TokenStream>,
    removed_fields: &mut Vec<proc_macro2::TokenStream>,
    inserted_fields: &mut Vec<proc_macro2::TokenStream>,
    field_ty: syn::Type,
    field_member: syn::Member,
    ctor_field_ident: Option<syn::Ident>,
    removed_ident: syn::Ident,
    inserted_ident: syn::Ident,
) {
    where_clause.predicates.push(syn::parse_quote! {
        #field_ty: ::undoredo::FlushEdit<#field_ty>
    });

    bindings.push(quote! {
        let (#removed_ident, #inserted_ident) =
            ::undoredo::FlushEdit::flush_edit(&mut self.#field_member).dissolve();
    });

    if let Some(field_ident) = ctor_field_ident {
        removed_fields.push(quote! { #field_ident: #removed_ident });
        inserted_fields.push(quote! { #field_ident: #inserted_ident });
    } else {
        removed_fields.push(quote! { #removed_ident });
        inserted_fields.push(quote! { #inserted_ident });
    }
}
