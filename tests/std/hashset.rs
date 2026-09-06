// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "std")]

#[path = "../common/mod.rs"]
mod common;

use std::collections::HashSet;

use undoredo::Recorder;
use undoredo::aliases::HashSetHalfDelta;

#[test]
fn test_recorder_apply_delta_on_set() {
    let recorder = Recorder::<HashSet<i32>>::new(HashSet::new());
    common::test_recorder_apply_delta_on_set(recorder);
}

#[test]
fn test_insert_and_remove_on_set() {
    let recorder = Recorder::<HashSet<i32>>::new(HashSet::new());
    common::test_insert_and_remove_on_set(recorder);
}

#[test]
fn test_delta_undo_redo_on_set() {
    common::test_delta_undo_redo_on_set::<i32, HashSet<i32>, HashSetHalfDelta<i32>>(HashSet::new());
}

#[test]
fn test_delta_undo_redo_clear() {
    common::test_delta_undo_redo_clear::<i32, (), HashSet<i32>, HashSetHalfDelta<i32>>(
        HashSet::new(),
    );
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo_set::<i32, HashSet<i32>>(HashSet::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
