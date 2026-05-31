// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod apply_delta;
mod delta;
mod reset_delta;
mod field_attrs;
mod flush_delta;
mod half_delta;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Generates a half-delta type for the given `struct` or `enum`.
#[proc_macro_derive(HalfDelta, attributes(undoredo))]
pub fn derive_half_delta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    half_delta::expand_half_delta(input).unwrap_or_else(|err| err.to_compile_error().into())
}

/// Generates an impl of the trait `ApplyDelta`.
#[proc_macro_derive(ApplyDelta, attributes(undoredo))]
pub fn derive_apply_delta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    apply_delta::expand_apply_delta(input).unwrap_or_else(|err| err.to_compile_error().into())
}

/// Generates an impl of the trait `FlushDelta`.
#[proc_macro_derive(FlushDelta, attributes(undoredo))]
pub fn derive_flush_delta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    flush_delta::expand_flush_delta(input).unwrap_or_else(|err| err.to_compile_error().into())
}

/// Generates an impl of the trait `ResetDelta`.
#[proc_macro_derive(ResetDelta, attributes(undoredo))]
pub fn derive_reset_delta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    reset_delta::expand_reset_delta(input).unwrap_or_else(|err| err.to_compile_error().into())
}

/// Generates an impl of the trait `ApplyDelta`.
#[proc_macro_derive(Delta, attributes(undoredo))]
pub fn derive_delta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    delta::expand_delta(input).unwrap_or_else(|err| err.to_compile_error().into())
}
