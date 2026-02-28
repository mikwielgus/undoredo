// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "derive")]

use std::collections::{BTreeMap, BTreeSet};

use undoredo::Recorder;

#[derive(undoredo::CompositeEdit)]
struct FlatStruct {
    map: Recorder<BTreeMap<i32, i32>, BTreeMap<i32, i32>>,
    set: Recorder<BTreeSet<i32>>,
    plain: i32,
}

#[test]
fn test_derive_composite_edit_on_named_struct() {
    let _edit = FlatStructCompositeEdit {
        map: BTreeMap::from([(1, 10)]),
        set: BTreeSet::from([1]),
        plain: 42,
    };
}
