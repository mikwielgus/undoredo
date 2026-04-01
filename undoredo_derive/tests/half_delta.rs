// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;

use undoredo_derive::HalfDelta;

#[derive(HalfDelta)]
struct TestStruct {
    v: Vec<i32>,
}

#[test]
fn test_struct_half_delta() {
    let _: TestStructHalfDelta = TestStructHalfDelta { v: BTreeMap::new() };
}

#[derive(HalfDelta)]
enum TestEnum {
    Unit,
    Tuple(Vec<i32>, Vec<i32>),
    Fields { i: i32, u: u32 },
}

#[test]
fn test_enum_half_delta() {
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Unit;
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Tuple(Vec::new(), Vec::new());
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Fields { i: 0, u: 0 };
}
