// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

pub(crate) fn resolve_half_delta_ident(input: &DeriveInput) -> syn::Result<syn::Ident> {
    let default = format_ident!("{}HalfDelta", input.ident);
    let mut name_from_attr: Option<syn::Ident> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("undoredo") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("half_delta_name") {
                    if name_from_attr.is_some() {
                        return Err(meta.error("duplicate `half_delta_name` in #[undoredo(...)]"));
                    }

                    let ident: syn::Ident = meta.value()?.parse()?;
                    name_from_attr = Some(ident);

                    return Ok(());
                }

                if meta.path.is_ident("delta_name") {
                    let _: syn::Ident = meta.value()?.parse()?;
                    return Ok(());
                }

                Err(meta.error("unrecognized undoredo attribute key"))
            })?;
        }
    }

    Ok(name_from_attr.unwrap_or(default))
}

pub(crate) fn expand_half_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let vis = &input.vis;
    let half_delta_name = resolve_half_delta_ident(&input)?;
    let generics = &input.generics;

    let output = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => {
                let mut transformed_fields = Vec::new();

                for field in &fields_named.named {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let field_name = &field.ident;
                    let ty = field_to_half_delta_container(&field.ty);

                    transformed_fields.push(quote! {
                        #field_name: #ty,
                    });
                }

                quote! {
                    #[derive(Clone, Debug)]
                    #vis struct #half_delta_name #generics {
                        #( #transformed_fields )*
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let mut transformed_fields = Vec::new();

                for field in &fields_unnamed.unnamed {
                    if crate::field_attrs::field_has_skip(field)? {
                        continue;
                    }

                    let ty = field_to_half_delta_container(&field.ty);
                    transformed_fields.push(quote! { #ty });
                }

                quote! {
                    #[derive(Clone, Debug)]
                    #vis struct #half_delta_name #generics ( #( #transformed_fields ),* );
                }
            }
            Fields::Unit => quote! {
                #[derive(Clone, Debug)]
                #vis struct #half_delta_name #generics;
            },
        },
        Data::Enum(_) => panic!("derive(HalfDelta) does not support enums"),
        Data::Union(_) => panic!("derive(HalfDelta) does not support unions"),
    };

    Ok(output.into())
}

pub(crate) fn field_to_half_delta_container(ty: &Type) -> TokenStream2 {
    let Type::Path(type_path) = ty else {
        return ty.to_token_stream();
    };

    let Some(last) = type_path.path.segments.last() else {
        return ty.to_token_stream();
    };

    let name = last.ident.to_string();

    let PathArguments::AngleBracketed(ab) = &last.arguments else {
        return ty.to_token_stream();
    };

    if name != "Recorder" || ab.args.is_empty() {
        return ty.to_token_stream();
    }

    let GenericArgument::Type(container_ty) = &ab.args[0] else {
        return ty.to_token_stream();
    };

    let Type::Path(container_path) = container_ty else {
        return ty.to_token_stream();
    };
    let Some(container_last) = container_path.path.segments.last() else {
        return ty.to_token_stream();
    };
    let container_name = container_last.ident.to_string();

    match &container_last.arguments {
        PathArguments::AngleBracketed(container_ab) => {
            if container_ab.args.len() != 1 {
                return ty.to_token_stream();
            }

            let GenericArgument::Type(ref t) = container_ab.args[0] else {
                return ty.to_token_stream();
            };

            if container_name == "Vec" || container_name == "StableVec" {
                quote! { ::undoredo::alloc::collections::BTreeMap<usize, #t> }
            } else if container_name == "Arena" {
                quote! { ::undoredo::alloc::collections::BTreeMap<::thunderdome::Index, #t> }
            } else if container_name == "RTree" {
                quote! { ::undoredo::alloc::collections::BTreeSet<#t> }
            } else {
                // Scalars and enums use a `BTreeMap` with a single element that
                // is a recorded version of the container itself.
                quote! { ::undoredo::alloc::collections::BTreeMap<usize, #container_ty> }
            }
        }
        _ => quote! { ::undoredo::alloc::collections::BTreeMap<usize, #container_ty> },
    }
}
