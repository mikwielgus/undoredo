// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use alloc::{collections::BTreeMap, vec::Vec};
use undoredo::Recorder;
use undoredo::aliases::VecHalfDelta;

#[test]
fn test_recorder_apply_delta_and_reverse() {
    common::test_recorder_apply_delta_and_reverse(Recorder::<Vec<i32>, VecHalfDelta<i32>>::new(
        Vec::new(),
    ));
}

#[test]
fn test_recorder_push_and_pop() {
    common::test_recorder_push_and_pop(Recorder::<Vec<i32>, VecHalfDelta<i32>>::new(Vec::new()));
}

#[test]
fn test_recorder_swap_remove() {
    common::test_recorder_swap_remove(Recorder::<Vec<i32>, VecHalfDelta<i32>>::new(Vec::new()));
}

#[test]
fn test_delta_undo_redo() {
    common::test_delta_undo_redo(Vec::new());
}

#[test]
fn test_delta_undo_redo_swap_remove() {
    common::test_delta_undo_redo_swap_remove(Vec::new());
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
