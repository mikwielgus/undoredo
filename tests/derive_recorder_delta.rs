// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "derive")]

use std::collections::{BTreeMap, BTreeSet};

use undoredo::Recorder;

#[derive(undoredo::CompositeDelta)]
struct FlatStruct {
    map: Recorder<BTreeMap<i32, i32>, BTreeMap<i32, i32>>,
    set: Recorder<BTreeSet<i32>>,
    plain: i32,
}

#[test]
fn test_derive_composite_delta_on_named_struct() {
    let _delta = FlatStructCompositeDelta {
        map: BTreeMap::from([(1, 10)]),
        set: BTreeMap::from([(1, ())]),
        plain: 42,
    };
}
