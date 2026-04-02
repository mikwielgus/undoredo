// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;

use maplike::Assign;
use undoredo::{ApplyDelta, Delta, FlushDelta, Recorder};
use undoredo_derive::undoredo;

#[derive(undoredo)]
struct TestStruct {
    v: Recorder<Vec<i32>>,
}

#[test]
fn test_undoredo_struct_derives() {
    let mut s = TestStruct {
        v: Recorder::new(vec![1, 2, 3]),
    };

    let d = Delta::with_removed_inserted(
        TestStructHalfDelta {
            v: BTreeMap::from([(2, 3)]),
        },
        TestStructHalfDelta {
            v: BTreeMap::from([(2, 7)]),
        },
    );
    s.apply_delta(&d);

    assert_eq!(s.v.as_ref(), &vec![1, 2, 7]);
}

#[derive(Clone, Debug, PartialEq, undoredo)]
enum TestEnum {
    Unit,
    Fields { i: i32, u: u32 },
}

#[test]
fn test_undoredo_enum_assign() {
    let mut e = TestEnum::Unit;
    e.assign(TestEnum::Fields { i: 1, u: 2 });

    assert_eq!(e, TestEnum::Fields { i: 1, u: 2 });
}
