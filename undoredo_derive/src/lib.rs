// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod apply_delta;
mod flush_delta;
mod half_delta;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(HalfDelta, attributes(half_delta))]
pub fn derive_half_delta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    half_delta::expand_half_delta(input).unwrap_or_else(|err| err.to_compile_error().into())
}

#[proc_macro_derive(ApplyDelta, attributes(apply_delta))]
pub fn derive_apply_delta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    apply_delta::expand_apply_delta(input).unwrap_or_else(|err| err.to_compile_error().into())
}
