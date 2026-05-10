// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;

use undoredo::{HalfDelta, Recorder};

#[derive(HalfDelta)]
#[undoredo(half_delta = TestStructHalfDelta)]
struct TestStruct {
    v: Recorder<Vec<i32>>,
}

#[test]
fn test_struct_half_delta() {
    let _: TestStructHalfDelta = TestStructHalfDelta { v: BTreeMap::new() };
}
