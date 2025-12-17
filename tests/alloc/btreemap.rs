// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use std::collections::BTreeMap;
use undoredo::Recorder;

#[test]
fn test_apply_edit_at_specified_indexes() {
    let recorder =
        Recorder::<usize, i32, BTreeMap<usize, i32>, BTreeMap<usize, i32>>::new(BTreeMap::new());
    common::test_apply_edit_at_specified_indexes(recorder);
}

#[test]
fn test_insert_and_remove_at_specified_indexes() {
    let recorder =
        Recorder::<usize, i32, BTreeMap<usize, i32>, BTreeMap<usize, i32>>::new(BTreeMap::new());
    common::test_insert_and_remove_at_specified_indexes(recorder);
}

#[test]
fn test_edit_undo_redo_at_specified_indexes() {
    common::test_edit_undo_redo_at_specified_indexes::<
        usize,
        BTreeMap<usize, i32>,
        BTreeMap<usize, i32>,
    >(BTreeMap::new());
}


