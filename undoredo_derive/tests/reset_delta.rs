// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::marker::PhantomData;

use undoredo::{HalfDelta, Recorder, ResetDelta};

#[derive(Clone, HalfDelta, ResetDelta)]
struct TestNamedFieldStruct {
    v: Recorder<Vec<i32>>,
    #[undoredo(skip)]
    not_in_delta: usize,
    tag: PhantomData<()>,
}

#[derive(Clone, HalfDelta, ResetDelta)]
struct TestTupleStruct(
    Recorder<Vec<i32>>,
    #[undoredo(skip)] PhantomData<u8>,
    Recorder<Vec<i32>>,
);

#[test]
fn named_reset_restores_recorded_changes() {
    let mut s = TestNamedFieldStruct {
        v: Recorder::new(vec![1, 2, 3]),
        not_in_delta: 99,
        tag: PhantomData,
    };

    s.v.set(1, 4);
    s.reset_delta();

    assert_eq!(*s.v.container(), vec![1, 2, 3]);
    assert_eq!(s.not_in_delta, 99);
}

#[test]
fn tuple_reset_ignores_skipped_fields() {
    let mut s = TestTupleStruct(
        Recorder::new(vec![10]),
        PhantomData,
        Recorder::new(vec![20]),
    );

    s.0.set(0, 11);
    s.2.set(0, 21);

    s.reset_delta();

    assert_eq!(*s.0.container(), vec![10]);
    assert_eq!(*s.2.container(), vec![20]);
}

#[test]
fn named_half_delta_has_no_skipped_field() {
    let _: TestNamedFieldStructHalfDelta = TestNamedFieldStructHalfDelta {
        v: BTreeMap::new(),
        tag: PhantomData,
    };
}
