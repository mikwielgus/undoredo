// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;

use undoredo::{ApplyDelta, Delta, Recorder};
use undoredo_derive::{ApplyDelta, HalfDelta};

#[derive(HalfDelta, ApplyDelta)]
struct TestStruct {
    v: Recorder<Vec<i32>>,
}

#[test]
fn test_apply_delta_struct() {
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
    assert_eq!(*s.v.container(), vec![1, 2, 7]);
}
