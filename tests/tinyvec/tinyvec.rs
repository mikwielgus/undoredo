// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use tinyvec::TinyVec;
use undoredo::Recorder;
use undoredo::aliases::TinyVecHalfDelta;

#[test]
fn test_recorder_apply_delta_and_reverse() {
    common::test_recorder_apply_delta_and_reverse(Recorder::<
        TinyVec<[i32; 8]>,
        TinyVecHalfDelta<[i32; 8]>,
    >::new(TinyVec::new()));
}

#[test]
fn test_recorder_push_and_pop() {
    common::test_recorder_push_and_pop(
        Recorder::<TinyVec<[i32; 8]>, TinyVecHalfDelta<[i32; 8]>>::new(TinyVec::new()),
    );
}

#[test]
fn test_delta_undo_redo() {
    common::test_delta_undo_redo(TinyVec::<[i32; 8]>::new());
}

#[test]
fn test_delta_undo_redo_clear() {
    common::test_delta_undo_redo_clear_at_generated_indices::<
        usize,
        TinyVec<[i32; 8]>,
        TinyVecHalfDelta<[i32; 8]>,
    >(TinyVec::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
