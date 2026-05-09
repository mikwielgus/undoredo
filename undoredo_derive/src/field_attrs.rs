// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{Field, Result};

/// True when `field` is under `#[undoredo(skip)]`.
pub(crate) fn field_has_skip(field: &Field) -> Result<bool> {
    let mut skip = false;

    for attr in &field.attrs {
        if attr.path().is_ident("undoredo") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    skip = true;
                    return Ok(());
                }

                Err(meta.error(
                    "unrecognized undoredo field attribute (only `skip` is supported on fields)",
                ))
            })?;
        }
    }

    Ok(skip)
}
