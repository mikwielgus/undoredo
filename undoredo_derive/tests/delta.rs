// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for `derive(Delta)` (doc comments here are to pass deny(missing_docs),
//! compliance with which we also test here).

#![deny(missing_docs)]
#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use maplike::ops::Assign;
use undoredo::aliases::{
    BTreeMapHalfDelta, BTreeSetHalfDelta, HashMapHalfDelta, HashSetHalfDelta, OptionHalfDelta,
    VecHalfDelta,
};
use undoredo::{ApplyDelta, Delta, Recorder, ResetDelta};

/// Struct under test for `derive(Delta)`.
#[derive(Delta)]
#[undoredo(half_delta = TestStructHalfDelta)]
#[undoredo(delta = TestStructDelta)]
pub struct TestStruct {
    hashmap: Recorder<HashMap<i32, i32>, HashMapHalfDelta<i32, i32>>,
    hashset: Recorder<HashSet<i32>, HashSetHalfDelta<i32>>,
    btreemap: Recorder<BTreeMap<i32, i32>>,
    btreeset: Recorder<BTreeSet<i32>>,
    option: Recorder<Option<i32>>,
    vec: Recorder<Vec<i32>>,
}

#[test]
fn test_struct_delta() {
    let _: TestStructDelta = TestStructDelta::with_removed_inserted(
        TestStructHalfDelta {
            hashmap: HashMapHalfDelta::new(),
            hashset: HashSetHalfDelta::new(),
            btreemap: BTreeMapHalfDelta::new(),
            btreeset: BTreeSetHalfDelta::new(),
            option: OptionHalfDelta::new(),
            vec: VecHalfDelta::new(),
        },
        TestStructHalfDelta {
            hashmap: HashMapHalfDelta::new(),
            hashset: HashSetHalfDelta::new(),
            btreemap: BTreeMapHalfDelta::new(),
            btreeset: BTreeSetHalfDelta::new(),
            option: OptionHalfDelta::new(),
            vec: VecHalfDelta::new(),
        },
    );
}

#[test]
fn test_delta_derive_struct() {
    let mut s = TestStruct {
        hashmap: Recorder::new(HashMap::from([(1, 2)])),
        hashset: Recorder::new(HashSet::from([1])),
        btreemap: Recorder::new(BTreeMap::from([(1, 2)])),
        btreeset: Recorder::new(BTreeSet::from([1])),
        option: Recorder::new(Some(1)),
        vec: Recorder::new(vec![1, 2, 3]),
    };

    let d = TestStructDelta::with_removed_inserted(
        TestStructHalfDelta {
            hashmap: HashMapHalfDelta::new(),
            hashset: HashSetHalfDelta::new(),
            btreemap: BTreeMapHalfDelta::new(),
            btreeset: BTreeSetHalfDelta::new(),
            option: OptionHalfDelta::new(),
            vec: VecHalfDelta::from([(2, 3)]),
        },
        TestStructHalfDelta {
            hashmap: HashMapHalfDelta::new(),
            hashset: HashSetHalfDelta::new(),
            btreemap: BTreeMapHalfDelta::new(),
            btreeset: BTreeSetHalfDelta::new(),
            option: OptionHalfDelta::new(),
            vec: VecHalfDelta::from([(2, 7)]),
        },
    );
    s.apply_delta(d);

    assert_eq!(s.vec.as_ref(), &vec![1, 2, 7]);
}

#[test]
fn test_delta_derive_struct_provides_reset_delta() {
    let mut s = TestStruct {
        hashmap: Recorder::new(HashMap::from([(1, 2)])),
        hashset: Recorder::new(HashSet::from([1])),
        btreemap: Recorder::new(BTreeMap::from([(1, 2)])),
        btreeset: Recorder::new(BTreeSet::from([1])),
        option: Recorder::new(Some(1)),
        vec: Recorder::new(vec![1, 2, 3]),
    };

    s.vec.set(2, 7);
    s.reset_delta();

    assert_eq!(*s.vec.container(), vec![1, 2, 3]);
}

/// Enum under test for `derive(Delta)`.
#[derive(Clone, Debug, PartialEq, Delta)]
pub enum TestEnum {
    /// Unit variant.
    Unit,
    /// Variant with named fields.
    Fields {
        /// Signed field.
        i: i32,
        /// Unsigned field.
        u: u32,
    },
}

#[test]
fn test_delta_derive_enum_assign() {
    let mut e = TestEnum::Unit;
    e.assign(TestEnum::Fields { i: 1, u: 2 });

    assert_eq!(e, TestEnum::Fields { i: 1, u: 2 });
}
