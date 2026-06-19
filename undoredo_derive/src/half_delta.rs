// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, Generics, PathArguments, Type};

pub(crate) fn resolve_half_delta_ident(input: &DeriveInput) -> syn::Result<syn::Ident> {
    let default = format_ident!("{}HalfDelta", input.ident);
    let mut name_from_attr: Option<syn::Ident> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("undoredo") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("half_delta") {
                    if name_from_attr.is_some() {
                        return Err(meta.error("duplicate `half_delta` in #[undoredo(...)]"));
                    }

                    let ident: syn::Ident = meta.value()?.parse()?;
                    name_from_attr = Some(ident);

                    return Ok(());
                }

                if meta.path.is_ident("delta") {
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
    let half_delta = resolve_half_delta_ident(&input)?;
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
                    let ty = field_to_half_delta_container(&field.ty, &input.generics);

                    transformed_fields.push(quote! {
                        #field_name: #ty,
                    });
                }

                quote! {
                    #[derive(Clone, Debug, Default)]
                    #vis struct #half_delta #generics {
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

                    let ty = field_to_half_delta_container(&field.ty, &input.generics);
                    transformed_fields.push(quote! { #ty });
                }

                quote! {
                    #[derive(Clone, Debug, Default)]
                    #vis struct #half_delta #generics ( #( #transformed_fields ),* );
                }
            }
            Fields::Unit => quote! {
                #[derive(Clone, Debug, Default)]
                #vis struct #half_delta #generics;
            },
        },
        Data::Enum(_) => panic!("derive(HalfDelta) does not support enums"),
        Data::Union(_) => panic!("derive(HalfDelta) does not support unions"),
    };

    Ok(output.into())
}

pub(crate) fn field_to_half_delta_container(ty: &Type, generics: &Generics) -> TokenStream2 {
    let Type::Path(type_path) = ty else {
        return ty.to_token_stream();
    };

    let Some(last) = type_path.path.segments.last() else {
        return ty.to_token_stream();
    };

    let name = last.ident.to_string();

    if name != "Recorder" {
        if name == "PhantomData" {
            return ty.to_token_stream();
        }

        if generics
            .type_params()
            .any(|type_param| type_param.ident == last.ident)
        {
            return ty.to_token_stream();
        }

        let mut half_path = type_path.path.clone();
        if let Some(last_mut) = half_path.segments.last_mut() {
            last_mut.ident = format_ident!("{}HalfDelta", last_mut.ident);
        }

        return quote! { #half_path };
    }

    let PathArguments::AngleBracketed(ab) = &last.arguments else {
        return ty.to_token_stream();
    };

    if ab.args.is_empty() {
        return ty.to_token_stream();
    }

    let GenericArgument::Type(container_ty) = &ab.args[0] else {
        return ty.to_token_stream();
    };

    container_ty_to_half_delta_alias(container_ty).unwrap_or_else(|| ty.to_token_stream())
}

fn container_ty_to_half_delta_alias(container_ty: &Type) -> Option<TokenStream2> {
    let Type::Path(container_path) = container_ty else {
        return None;
    };

    let container_last = container_path.path.segments.last()?;

    match &container_last.arguments {
        PathArguments::AngleBracketed(container_ab) => match container_ab.args.len() {
            1 => {
                let GenericArgument::Type(ty) = &container_ab.args[0] else {
                    return None;
                };

                let container_half = format_ident!("{}HalfDelta", container_last.ident);
                Some(quote! { ::undoredo::aliases::#container_half<#ty> })
            }
            // `BiBTreeMap` and `BiHashMap` accept two generic parameters
            // (arguments), so we need a separate code branch for them.
            2 => {
                let GenericArgument::Type(left_ty) = &container_ab.args[0] else {
                    return None;
                };

                let GenericArgument::Type(right_ty) = &container_ab.args[1] else {
                    return None;
                };

                let container_half = format_ident!("{}HalfDelta", container_last.ident);
                Some(quote! { ::undoredo::aliases::#container_half<#left_ty, #right_ty> })
            }
            _ => None,
        },
        _ => Some(quote! { ::undoredo::aliases::BTreeMapHalfDelta<usize, #container_ty> }),
    }
}
