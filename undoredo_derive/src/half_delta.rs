// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

pub(crate) fn expand_half_delta(input: DeriveInput) -> syn::Result<TokenStream> {
    let vis = &input.vis;
    let name = input.ident;
    let mut half_delta_name = format_ident!("{}HalfDelta", name);

    for attr in &input.attrs {
        if attr.path().is_ident("half_delta") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value: syn::LitStr = meta.value()?.parse()?;
                    half_delta_name = format_ident!("{}", value.value());
                    Ok(())
                } else {
                    Err(meta.error("unsupported half_delta attribute key"))
                }
            })?;
        }
    }

    let output = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => {
                let transformed_fields = fields_named.named.iter().map(|field| {
                    let field_name = &field.ident;
                    let ty = transform_field_type(&field.ty);
                    quote! {
                        #field_name: #ty,
                    }
                });
                quote! {
                    #[derive(Clone, Debug)]
                    #vis struct #half_delta_name {
                        #( #transformed_fields )*
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let transformed_fields = fields_unnamed.unnamed.iter().map(|field| {
                    let ty = transform_field_type(&field.ty);
                    quote! { #ty }
                });
                quote! {
                    #[derive(Clone, Debug)]
                    #vis struct #half_delta_name ( #( #transformed_fields ),* );
                }
            }
            Fields::Unit => quote! {
                #[derive(Clone, Debug)]
                #vis struct #half_delta_name;
            },
        },
        Data::Enum(_) => panic!("derive(HalfDelta) does not support enums"),
        Data::Union(_) => panic!("derive(HalfDelta) does not support unions"),
    };

    Ok(output.into())
}

fn transform_field_type(ty: &Type) -> TokenStream2 {
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

    let PathArguments::AngleBracketed(container_ab) = &container_last.arguments else {
        return ty.to_token_stream();
    };
    if container_ab.args.len() != 1 {
        return ty.to_token_stream();
    }

    let GenericArgument::Type(ref t) = container_ab.args[0] else {
        return ty.to_token_stream();
    };

    if container_name == "Vec" || container_name == "StableVec" {
        quote! { ::std::collections::BTreeMap<usize, #t> }
    } else if container_name == "Arena" {
        quote! { ::std::collections::BTreeMap<::thunderdome::Index, #t> }
    } else if container_name == "RTree" {
        quote! { ::std::collections::BTreeSet<#t> }
    } else {
        ty.to_token_stream()
    }
}
