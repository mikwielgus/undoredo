// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use indexmap::IndexSet;

use undoredo::Recorder;
use undoredo::aliases::IndexSetHalfDelta;

#[test]
fn test_recorder_apply_delta_on_set() {
    let recorder = Recorder::<IndexSet<i32>>::new(IndexSet::new());
    common::test_recorder_apply_delta_on_set(recorder);
}

#[test]
fn test_insert_and_remove_on_set() {
    let recorder = Recorder::<IndexSet<i32>>::new(IndexSet::new());
    common::test_insert_and_remove_on_set(recorder);
}

#[test]
fn test_delta_undo_redo_on_set() {
    common::test_delta_undo_redo_on_set::<i32, IndexSet<i32>, IndexSetHalfDelta<i32>>(
        IndexSet::new(),
    );
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo_set::<i32, IndexSet<i32>>(IndexSet::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
