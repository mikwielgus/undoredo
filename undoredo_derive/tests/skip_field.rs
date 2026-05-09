// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::marker::PhantomData;

use undoredo::delta::Delta;
use undoredo::delta::VecHalfDelta;
use undoredo::{ApplyDelta, FlushDelta as FlushDeltaTrait, Recorder};
use undoredo_derive::{ApplyDelta, FlushDelta, HalfDelta};

#[derive(Clone, HalfDelta, ApplyDelta, FlushDelta)]
struct TestNamedFieldStruct {
    v: Recorder<Vec<i32>>,
    #[undoredo(skip)]
    _tag: usize,
}

#[derive(Clone, HalfDelta, ApplyDelta, FlushDelta)]
struct TestTupleStruct(
    Recorder<Vec<i32>>,
    #[undoredo(skip)] PhantomData<u8>,
    Recorder<Vec<i32>>,
);

#[test]
fn named_skip_apply_delta_plain_field_without_flush_bounds() {
    let mut s = TestNamedFieldStruct {
        v: Recorder::new(vec![1, 2, 3]),
        _tag: 99,
    };
    let d = Delta::with_removed_inserted(
        TestNamedFieldStructHalfDelta {
            v: VecHalfDelta::from([(1, 10)]),
        },
        TestNamedFieldStructHalfDelta {
            v: VecHalfDelta::from([(1, 4)]),
        },
    );
    s.apply_delta(d);
    assert_eq!(*s.v.container(), vec![1, 4, 3]);
    assert_eq!(s._tag, 99);
}

#[test]
fn named_skip_flush_half_delta_has_no_skipped_field() {
    let mut s = TestNamedFieldStruct {
        v: Recorder::new(vec![1, 2, 3]),
        _tag: 42,
    };
    let d = FlushDeltaTrait::flush_delta(&mut s);
    let (removed, inserted) = d.dissolve();
    assert_eq!(removed.v, BTreeMap::new());
    assert_eq!(inserted.v, BTreeMap::new());
}

#[test]
fn tuple_skip_half_delta_indexes_align_with_dense_tuple() {
    let mut s = TestTupleStruct(
        Recorder::new(vec![100]),
        PhantomData,
        Recorder::new(vec![200]),
    );

    let d = Delta::with_removed_inserted(
        TestTupleStructHalfDelta(VecHalfDelta::new(), VecHalfDelta::from([(0, 200)])),
        TestTupleStructHalfDelta(VecHalfDelta::new(), VecHalfDelta::from([(0, 7)])),
    );
    s.apply_delta(d);
    assert_eq!(*s.0.container(), vec![100]);
    assert_eq!(*s.2.container(), vec![7]);

    let d2 = Delta::with_removed_inserted(
        TestTupleStructHalfDelta(VecHalfDelta::from([(0, 100)]), VecHalfDelta::new()),
        TestTupleStructHalfDelta(VecHalfDelta::from([(0, 50)]), VecHalfDelta::new()),
    );
    s.apply_delta(d2);
    assert_eq!(*s.0.container(), vec![50]);
    assert_eq!(*s.2.container(), vec![7]);
}
