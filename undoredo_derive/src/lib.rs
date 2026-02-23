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
