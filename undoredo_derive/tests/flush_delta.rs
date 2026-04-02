// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;

use undoredo::{FlushDelta, Recorder};
use undoredo_derive::{FlushDelta, HalfDelta};

#[derive(Debug, PartialEq, HalfDelta, FlushDelta)]
struct TestStruct {
    v: Recorder<Vec<i32>>,
}

#[test]
fn test_flush_delta_struct() {
    let mut s = TestStruct {
        v: Recorder::new(vec![1, 2, 3]),
    };
    let d = s.flush_delta();

    let (removed, inserted) = d.dissolve();
    assert_eq!(removed.v, BTreeMap::new());
    assert_eq!(inserted.v, BTreeMap::new());
}

#[derive(Clone, Debug, PartialEq, HalfDelta, FlushDelta)]
enum TestEnum {
    Unit,
    Tuple(i32),
    Fields { i: i32, u: u32 },
}

#[test]
fn test_flush_delta_enum() {
    let mut e = TestEnum::Fields { i: 1, u: 2 };
    let d = e.flush_delta();

    let (removed, inserted) = d.dissolve();
    assert_eq!(removed, TestEnum::Fields { i: 1, u: 2 });
    assert_eq!(inserted, TestEnum::Fields { i: 1, u: 2 });
    assert_eq!(e, TestEnum::Fields { i: 1, u: 2 });
}
