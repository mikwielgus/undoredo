// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate alloc;

#[path = "alloc/btreemap.rs"]
mod alloc_btreemap;

#[path = "alloc/btreeset.rs"]
mod alloc_btreeset;

#[path = "alloc/vec.rs"]
mod alloc_vec;

#[path = "std/hashmap.rs"]
mod std_hashmap;

#[path = "std/hashset.rs"]
mod std_hashset;

#[cfg(feature = "indexmap")]
#[path = "indexmap/indexmap.rs"]
mod indexmap_indexmap;

#[cfg(feature = "indexmap")]
#[path = "indexmap/indexset.rs"]
mod indexmap_indexset;

#[cfg(feature = "tinyvec")]
#[path = "tinyvec/arrayvec.rs"]
mod tinyvec_arrayvec;

#[cfg(feature = "tinyvec")]
#[path = "tinyvec/tinyvec.rs"]
mod tinyvec_tinyvec;
