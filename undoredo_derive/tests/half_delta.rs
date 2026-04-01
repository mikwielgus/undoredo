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
    Tuple(Vec<i32>),
    Fields { n: i32 },
}

#[test]
fn test_enum_half_delta() {
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Unit;
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Tuple(BTreeMap::new());
    let _: TestEnumHalfDelta = TestEnumHalfDelta::Fields { n: 0 };
}

#[repr(C)]
#[derive(HalfDelta)]
union TestUnion {
    u: u32,
    f: f32,
}

#[test]
fn test_union_half_delta() {
    let _: TestUnionHalfDelta = TestUnionHalfDelta { u: 0 };
}
