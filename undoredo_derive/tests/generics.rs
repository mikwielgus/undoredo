// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;

use undoredo::{ApplyDelta, FlushDelta, Recorder};
use undoredo_derive::Delta;

#[derive(Clone, Debug, Delta)]
struct TestGenericStruct<C1 = Recorder<Vec<usize>>, C2 = C1, C3 = C1> {
    first: C1,
    second: C2,
    third: C3,
}

#[test]
fn test_generics_delta_derive_with_three_containers() {
    type RecordedVec = Recorder<Vec<usize>, BTreeMap<usize, usize>>;

    let mut from = TestGenericStruct {
        first: RecordedVec::new(vec![0, 1]),
        second: RecordedVec::new(vec![0, 0]),
        third: RecordedVec::new(vec![1, 1]),
    };
    let mut to = TestGenericStruct {
        first: RecordedVec::new(vec![0, 1]),
        second: RecordedVec::new(vec![0, 0]),
        third: RecordedVec::new(vec![1, 1]),
    };

    from.first.set(1, 0);
    from.second.set(0, 1);
    from.third.set(1, 2);

    let delta: undoredo::Delta<TestGenericStructHalfDelta<RecordedVec, RecordedVec, RecordedVec>> =
        from.flush_delta();
    to.apply_delta(delta);

    assert_eq!(to.first.container(), &vec![0, 0]);
    assert_eq!(to.second.container(), &vec![1, 0]);
    assert_eq!(to.third.container(), &vec![2, 2]);
}
