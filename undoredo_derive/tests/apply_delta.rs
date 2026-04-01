// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;

use undoredo::{ApplyDelta, Delta};
use undoredo_derive::{ApplyDelta, HalfDelta};

#[derive(HalfDelta, ApplyDelta)]
struct TestStruct {
    v: Vec<i32>,
}

#[test]
fn test_apply_delta_struct() {
    let mut s = TestStruct { v: vec![1, 2, 3] };

    let d = Delta::with_removed_inserted(
        TestStructHalfDelta {
            v: BTreeMap::from([(2, 3)]),
        },
        TestStructHalfDelta {
            v: BTreeMap::from([(2, 7)]),
        },
    );

    s.apply_delta(&d);
    assert_eq!(s.v, vec![1, 2, 7]);
}

#[derive(Clone, HalfDelta, ApplyDelta, Debug, PartialEq)]
enum TestEnum {
    Unit,
    Tuple(Vec<i32>, Vec<i32>),
    Fields { i: i32, u: u32 },
}

#[test]
fn test_apply_delta_enum() {
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Unit;
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Tuple(Vec::new(), Vec::new());
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Fields { i: 0, u: 0 };

    let mut e = TestEnum::Tuple(Vec::new(), Vec::new());
    let d = Delta::with_removed_inserted(
        TestEnumHalfDelta::Unit,
        TestEnumHalfDelta::Fields { i: 1, u: 2 },
    );
    e.apply_delta(&d);

    assert_eq!(e, TestEnum::Fields { i: 1, u: 2 });
}
