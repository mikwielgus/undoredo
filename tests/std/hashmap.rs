// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "std")]

#[path = "../common/mod.rs"]
mod common;

use std::collections::HashMap;

use undoredo::{HashMapHalfDelta, Recorder};

#[test]
fn test_recorder_apply_delta_at_specified_indices() {
    let recorder = Recorder::<HashMap<usize, i32>>::new(HashMap::new());
    common::test_recorder_apply_delta_at_specified_indices(recorder);
}

#[test]
fn test_recorder_insert_and_remove_at_specified_indices() {
    let recorder = Recorder::<HashMap<usize, i32>>::new(HashMap::new());
    common::test_recorder_insert_and_remove_at_specified_indices(recorder);
}

#[test]
fn test_delta_undo_redo_at_specified_indices() {
    common::test_delta_undo_redo_at_specified_indices::<
        usize,
        i32,
        HashMap<usize, i32>,
        HashMapHalfDelta<usize, i32>,
    >(HashMap::new());
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo::<usize, i32, HashMap<usize, i32>>(HashMap::new());
}
