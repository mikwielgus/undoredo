// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for `derive(HalfDelta)` (doc comments here are to pass deny(missing_docs)).

#![deny(missing_docs)]
#![allow(dead_code)]

use std::collections::BTreeMap;

use undoredo::{HalfDelta, Recorder};

/// Struct under test.
#[derive(HalfDelta)]
#[undoredo(half_delta = TestStructHalfDelta)]
pub struct TestStruct {
    v: Recorder<Vec<i32>>,
}

#[test]
fn test_struct_half_delta() {
    let _: TestStructHalfDelta = TestStructHalfDelta { v: BTreeMap::new() };
}
