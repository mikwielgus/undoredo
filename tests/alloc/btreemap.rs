// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use alloc::collections::BTreeMap;
use undoredo::delta::BTreeMapHalfDelta;
use undoredo::Recorder;

#[test]
fn test_recorder_apply_delta_at_specified_indices() {
    let recorder = Recorder::<BTreeMap<usize, i32>>::new(BTreeMap::new());
    common::test_recorder_apply_delta_at_specified_indices(recorder);
}

#[test]
fn test_recorder_insert_and_remove_at_specified_indices() {
    let recorder = Recorder::<BTreeMap<usize, i32>>::new(BTreeMap::new());
    common::test_recorder_insert_and_remove_at_specified_indices(recorder);
}

#[test]
fn test_delta_undo_redo_at_specified_indices() {
    common::test_delta_undo_redo_at_specified_indices::<
        usize,
        i32,
        BTreeMap<usize, i32>,
        BTreeMapHalfDelta<usize, i32>,
    >(BTreeMap::new());
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
}
