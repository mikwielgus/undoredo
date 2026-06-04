// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use undoredo::aliases::VecHalfDelta;
use undoredo::{ApplyDelta, Delta, HalfDelta, MergeDelta, Recorder};

#[derive(HalfDelta, ApplyDelta, MergeDelta)]
#[undoredo(half_delta = TestStructHalfDelta)]
struct TestStruct {
    v: Recorder<Vec<i32>>,
}

#[test]
fn test_merge_delta_struct() {
    let mut s = TestStruct {
        v: Recorder::new(vec![1, 2, 3]),
    };

    let first = Delta::with_removed_inserted(
        TestStructHalfDelta {
            v: VecHalfDelta::from([(2, 3)]),
        },
        TestStructHalfDelta {
            v: VecHalfDelta::from([(2, 5)]),
        },
    );
    let second = Delta::with_removed_inserted(
        TestStructHalfDelta {
            v: VecHalfDelta::new(),
        },
        TestStructHalfDelta {
            v: VecHalfDelta::from([(2, 7)]),
        },
    );

    s.apply_delta(first.merge_delta(second));
    assert_eq!(*s.v.container(), vec![1, 2, 7]);
}
