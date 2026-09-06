// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use alloc::collections::BTreeSet;
use undoredo::Recorder;
use undoredo::aliases::BTreeSetHalfDelta;

#[test]
fn test_recorder_apply_delta_on_set() {
    let recorder = Recorder::<BTreeSet<i32>>::new(BTreeSet::new());
    common::test_recorder_apply_delta_on_set(recorder);
}

#[test]
fn test_insert_and_remove_on_set() {
    let recorder = Recorder::<BTreeSet<i32>>::new(BTreeSet::new());
    common::test_insert_and_remove_on_set(recorder);
}

#[test]
fn test_delta_undo_redo_on_set() {
    common::test_delta_undo_redo_on_set::<i32, BTreeSet<i32>, BTreeSetHalfDelta<i32>>(
        BTreeSet::new(),
    );
}

#[test]
fn test_delta_undo_redo_clear() {
    common::test_delta_undo_redo_clear::<i32, (), BTreeSet<i32>, BTreeSetHalfDelta<i32>>(
        BTreeSet::new(),
    );
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo_set::<i32, BTreeSet<i32>>(BTreeSet::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
