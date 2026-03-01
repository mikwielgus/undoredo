// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

#[proc_macro_derive(ApplyEdit)]
pub fn derive_apply_edit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_apply_edit(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_apply_edit(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let mut generics = input.generics;
    let edit_name = composite_edit_ident(&name);

    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "ApplyEdit can only be derived for structs",
            ));
        }
    };

    let generics_for_edit = generics.clone();
    let (_, edit_ty_generics, _) = generics_for_edit.split_for_impl();

    let where_clause = generics.make_where_clause();
    where_clause.predicates.push(syn::parse_quote! {
        ::undoredo::Edit<#edit_name #edit_ty_generics>: ::core::clone::Clone
    });

    let mut apply_stmts = Vec::new();

    match fields {
        Fields::Named(named) => {
            for field in named.named {
                push_field_apply_stmt(
                    where_clause,
                    &mut apply_stmts,
                    field.ty.clone(),
                    recorder_type_to_edit_collection_type(field.ty),
                    syn::Member::Named(field.ident.expect("named field must have ident")),
                );
            }
        }
        Fields::Unnamed(unnamed) => {
            for (index, field) in unnamed.unnamed.into_iter().enumerate() {
                push_field_apply_stmt(
                    where_clause,
                    &mut apply_stmts,
                    field.ty.clone(),
                    recorder_type_to_edit_collection_type(field.ty),
                    syn::Member::Unnamed(syn::Index::from(index)),
                );
            }
        }
        Fields::Unit => {}
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let output = quote! {
        impl #impl_generics ::undoredo::ApplyEdit<#edit_name #ty_generics> for #name #ty_generics
        #where_clause
        {
            fn apply_edit(&mut self, edit: &::undoredo::Edit<#edit_name #ty_generics>) {
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
    edit_field_ty: syn::Type,
    field_member: syn::Member,
) {
    where_clause.predicates.push(syn::parse_quote! {
        #field_ty: ::undoredo::ApplyEdit<#edit_field_ty> + ::core::clone::Clone
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
    let edit_name = composite_edit_ident(&name);

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
                    field.ty.clone(),
                    recorder_type_to_edit_collection_type(field.ty),
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
                quote! { #edit_name { #(#removed_fields),* } },
                quote! { #edit_name { #(#inserted_fields),* } },
            )
        }
        Fields::Unnamed(unnamed) => {
            for (index, field) in unnamed.unnamed.into_iter().enumerate() {
                push_field_flush_parts(
                    where_clause,
                    &mut bindings,
                    &mut removed_fields,
                    &mut inserted_fields,
                    field.ty.clone(),
                    recorder_type_to_edit_collection_type(field.ty),
                    syn::Member::Unnamed(syn::Index::from(index)),
                    None,
                    syn::Ident::new(
                        &format!("removed_{}", index),
                        proc_macro2::Span::call_site(),
                    ),
                    syn::Ident::new(
                        &format!("inserted_{}", index),
                        proc_macro2::Span::call_site(),
                    ),
                );
            }

            (
                quote! { #edit_name ( #(#removed_fields),* ) },
                quote! { #edit_name ( #(#inserted_fields),* ) },
            )
        }
        Fields::Unit => (quote! { #edit_name }, quote! { #edit_name }),
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        impl #impl_generics ::undoredo::FlushEdit<#edit_name #ty_generics> for #name #ty_generics
        #where_clause
        {
            fn flush_edit(&mut self) -> ::undoredo::Edit<#edit_name #ty_generics> {
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
    edit_field_ty: syn::Type,
    field_member: syn::Member,
    ctor_field_ident: Option<syn::Ident>,
    removed_ident: syn::Ident,
    inserted_ident: syn::Ident,
) {
    where_clause.predicates.push(syn::parse_quote! {
        #field_ty: ::undoredo::FlushEdit<#edit_field_ty>
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

#[proc_macro_derive(CompositeEdit)]
pub fn derive_composite_edit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_composite_edit(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_composite_edit(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let vis = input.vis;
    let generics = input.generics;
    let edit_name = syn::Ident::new(
        &format!("{}CompositeEdit", name),
        proc_macro2::Span::call_site(),
    );

    let data_struct = match input.data {
        Data::Struct(data) => data,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "CompositeEdit can only be derived for structs",
            ));
        }
    };

    let fields = match data_struct.fields {
        Fields::Named(named) => {
            let mapped = named
                .named
                .into_iter()
                .map(|mut field| {
                    field.ty = recorder_type_to_edit_collection_type(field.ty);
                    field
                })
                .collect();
            Fields::Named(syn::FieldsNamed {
                brace_token: named.brace_token,
                named: mapped,
            })
        }
        Fields::Unnamed(unnamed) => {
            let mapped = unnamed
                .unnamed
                .into_iter()
                .map(|mut field| {
                    field.ty = recorder_type_to_edit_collection_type(field.ty);
                    field
                })
                .collect();
            Fields::Unnamed(syn::FieldsUnnamed {
                paren_token: unnamed.paren_token,
                unnamed: mapped,
            })
        }
        Fields::Unit => Fields::Unit,
    };

    Ok(quote! {
        #[derive(Clone)]
        #vis struct #edit_name #generics #fields
    }
    .into())
}

fn composite_edit_ident(name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &format!("{}CompositeEdit", name),
        proc_macro2::Span::call_site(),
    )
}

fn recorder_type_to_edit_collection_type(field_ty: Type) -> Type {
    match field_ty {
        Type::Path(mut ty_path) => {
            if let Some(last_segment) = ty_path.path.segments.last_mut() {
                if last_segment.ident == "Recorder" {
                    if let PathArguments::AngleBracketed(args) = &mut last_segment.arguments {
                        let type_args = args
                            .args
                            .iter()
                            .filter_map(|arg| match arg {
                                GenericArgument::Type(ty) => Some(ty.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>();

                        if type_args.len() >= 2 {
                            return type_args[1].clone();
                        }

                        if let Some(collection_ty) = type_args.first() {
                            return syn::parse_quote! {
                                <::undoredo::Recorder<#collection_ty> as ::undoredo::RecorderEditCollection>::EditCollection
                            };
                        }
                    }
                }
            }

            Type::Path(ty_path)
        }
        other => other,
    }
}
