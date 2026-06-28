// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use maplike::Assign;
use undoredo::aliases::VecHalfDelta;
use undoredo::{ApplyDelta, Delta, Recorder, ResetDelta};

#[derive(Delta)]
#[undoredo(half_delta = TestStructHalfDelta)]
#[undoredo(delta = TestStructDelta)]
struct TestStruct {
    v: Recorder<Vec<i32>>,
}

#[test]
fn test_delta_derive_struct() {
    let mut s = TestStruct {
        v: Recorder::new(vec![1, 2, 3]),
    };

    let d = TestStructDelta::with_removed_inserted(
        TestStructHalfDelta {
            v: VecHalfDelta::from([(2, 3)]),
        },
        TestStructHalfDelta {
            v: VecHalfDelta::from([(2, 7)]),
        },
    );
    s.apply_delta(d);

    assert_eq!(s.v.as_ref(), &vec![1, 2, 7]);
}

#[test]
fn test_delta_derive_struct_provides_reset_delta() {
    let mut s = TestStruct {
        v: Recorder::new(vec![1, 2, 3]),
    };

    s.v.set(2, 7);
    s.reset_delta();

    assert_eq!(*s.v.container(), vec![1, 2, 3]);
}

#[derive(Clone, Debug, PartialEq, Delta)]
enum TestEnum {
    Unit,
    Fields { i: i32, u: u32 },
}

#[test]
fn test_delta_derive_enum_assign() {
    let mut e = TestEnum::Unit;
    e.assign(TestEnum::Fields { i: 1, u: 2 });

    assert_eq!(e, TestEnum::Fields { i: 1, u: 2 });
}
