// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use tinyvec::ArrayVec;
use undoredo::Recorder;
use undoredo::aliases::ArrayVecHalfDelta;

#[test]
fn test_recorder_apply_delta_and_reverse() {
    common::test_recorder_apply_delta_and_reverse(Recorder::<
        ArrayVec<[i32; 8]>,
        ArrayVecHalfDelta<[i32; 8]>,
    >::new(ArrayVec::new()));
}

#[test]
fn test_recorder_push_and_pop() {
    common::test_recorder_push_and_pop(
        Recorder::<ArrayVec<[i32; 8]>, ArrayVecHalfDelta<[i32; 8]>>::new(ArrayVec::new()),
    );
}

#[test]
fn test_delta_undo_redo() {
    common::test_delta_undo_redo(ArrayVec::<[i32; 8]>::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
