// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "std")]

#[path = "../common/mod.rs"]
mod common;

use std::collections::HashMap;

use undoredo::Recorder;

#[test]
fn test_apply_edit_at_specified_indices() {
    let recorder = Recorder::<HashMap<usize, i32>>::new(HashMap::new());
    common::test_apply_edit_at_specified_indices(recorder);
}

#[test]
fn test_insert_and_remove_at_specified_indices() {
    let recorder = Recorder::<HashMap<usize, i32>>::new(HashMap::new());
    common::test_insert_and_remove_at_specified_indices(recorder);
}

#[test]
fn test_edit_undo_redo_at_specified_indices() {
    common::test_edit_undo_redo_at_specified_indices::<
        usize,
        i32,
        HashMap<usize, i32>,
        HashMap<usize, i32>,
    >(HashMap::new());
}
