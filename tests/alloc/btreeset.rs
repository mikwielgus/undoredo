// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use alloc::collections::BTreeSet;
use undoredo::Recorder;

#[test]
fn test_apply_edit_on_set() {
    let recorder = Recorder::<BTreeSet<i32>>::new(BTreeSet::new());
    common::test_apply_edit_on_set(recorder);
}

#[test]
fn test_insert_and_remove_on_set() {
    let recorder = Recorder::<BTreeSet<i32>>::new(BTreeSet::new());
    common::test_insert_and_remove_on_set(recorder);
}

#[test]
fn test_edit_undo_redo_on_set() {
    common::test_edit_undo_redo_on_set::<i32, BTreeSet<i32>, BTreeSet<i32>>(BTreeSet::new());
}
