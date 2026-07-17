// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate alloc;

#[path = "alloc/btreemap.rs"]
mod btreemap;

#[path = "alloc/btreeset.rs"]
mod btreeset;

#[path = "alloc/vec.rs"]
mod vec;

#[path = "std/hashmap.rs"]
mod hashmap;

#[path = "std/hashset.rs"]
mod hashset;

#[cfg(feature = "indexmap")]
#[path = "indexmap/indexmap.rs"]
mod indexmap;

#[cfg(feature = "indexmap")]
#[path = "indexmap/indexset.rs"]
mod indexset;
