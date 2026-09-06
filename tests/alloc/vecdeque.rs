// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use alloc::collections::{BTreeMap, VecDeque};
use undoredo::Recorder;
use undoredo::aliases::VecDequeHalfDelta;

#[test]
fn test_recorder_apply_delta_and_reverse() {
    common::test_recorder_apply_delta_and_reverse(
        Recorder::<VecDeque<i32>, VecDequeHalfDelta<i32>>::new(VecDeque::new()),
    );
}

#[test]
fn test_recorder_push_and_pop() {
    common::test_recorder_push_and_pop(Recorder::<VecDeque<i32>, VecDequeHalfDelta<i32>>::new(
        VecDeque::new(),
    ));
}

#[test]
fn test_delta_undo_redo() {
    common::test_delta_undo_redo(VecDeque::new());
}

#[test]
fn test_delta_undo_redo_clear() {
    common::test_delta_undo_redo_clear_at_generated_indices::<
        usize,
        VecDeque<i32>,
        VecDequeHalfDelta<i32>,
    >(VecDeque::new());
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
