// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "stable-vec")]

#[path = "common/mod.rs"]
mod common;

use stable_vec::StableVec;
use undoredo::Recorder;

#[test]
fn test_apply_edit_at_generated_indices() {
    let recorder = Recorder::<StableVec<i32>>::new(StableVec::new());
    common::test_apply_edit_at_generated_indices::<usize, _, _>(recorder);
}

#[test]
fn test_apply_edit_at_specified_indices() {
    let recorder = Recorder::<StableVec<i32>>::new(StableVec::new());
    common::test_apply_edit_at_specified_indices(recorder);
}

#[test]
fn test_insert_and_remove_at_generated_indices() {
    let recorder = Recorder::<StableVec<i32>>::new(StableVec::new());
    common::test_insert_and_remove_at_generated_indices::<usize, _, _>(recorder);
}

#[test]
fn test_insert_and_remove_at_specified_indices() {
    let recorder = Recorder::<StableVec<i32>>::new(StableVec::new());
    common::test_insert_and_remove_at_specified_indices(recorder);
}

#[test]
fn test_undo_redo_at_generated_indices() {
    common::test_undo_redo_at_generated_indices::<usize, StableVec<i32>, StableVec<i32>>(
        StableVec::new(),
    );
}

#[test]
fn test_undo_redo_at_specified_indices() {
    common::test_undo_redo_at_specified_indices::<usize, i32, StableVec<i32>, StableVec<i32>>(
        StableVec::new(),
    );
}
