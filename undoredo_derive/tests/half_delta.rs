// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for `derive(Delta)` (doc comments here are to pass deny(missing_docs),
//! compliance with which we also test here).

#![deny(missing_docs)]
#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use undoredo::aliases::{
    BTreeMapHalfDelta, BTreeSetHalfDelta, HashMapHalfDelta, HashSetHalfDelta, OptionHalfDelta,
    VecHalfDelta,
};
use undoredo::{HalfDelta, Recorder};

/// Struct under test.
#[derive(HalfDelta)]
#[undoredo(half_delta = TestStructHalfDelta)]
pub struct TestStruct {
    hashmap: Recorder<HashMap<i32, i32>>,
    hashset: Recorder<HashSet<i32>>,
    btreemap: Recorder<BTreeMap<i32, i32>>,
    btreeset: Recorder<BTreeSet<i32>>,
    option: Recorder<Option<i32>>,
    vec: Recorder<Vec<i32>>,
}

#[test]
fn test_struct_half_delta() {
    let _: TestStructHalfDelta = TestStructHalfDelta {
        hashmap: HashMapHalfDelta::new(),
        hashset: HashSetHalfDelta::new(),
        btreemap: BTreeMapHalfDelta::new(),
        btreeset: BTreeSetHalfDelta::new(),
        option: OptionHalfDelta::new(),
        vec: VecHalfDelta::new(),
    };
}
