// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use indexmap::IndexMap;

use undoredo::Recorder;
use undoredo::aliases::IndexMapHalfDelta;

#[test]
fn test_recorder_apply_delta_at_specified_indices() {
    let recorder = Recorder::<IndexMap<usize, i32>>::new(IndexMap::new());
    common::test_recorder_apply_delta_at_specified_indices(recorder);
}

#[test]
fn test_recorder_insert_and_remove_at_specified_indices() {
    let recorder = Recorder::<IndexMap<usize, i32>>::new(IndexMap::new());
    common::test_recorder_insert_and_remove_at_specified_indices(recorder);
}

#[test]
fn test_delta_undo_redo_at_specified_indices() {
    common::test_delta_undo_redo_at_specified_indices::<
        usize,
        i32,
        IndexMap<usize, i32>,
        IndexMapHalfDelta<usize, i32>,
    >(IndexMap::new());
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo::<usize, i32, IndexMap<usize, i32>>(IndexMap::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
