#![cfg(feature = "stable-vec")]

#[path = "common/mod.rs"]
mod common;

use stable_vec::StableVec;
use undoredo::Recorder;

#[test]
fn test_apply_edit_at_generated_indexes() {
    let recorder = Recorder::<usize, i32, StableVec<i32>, StableVec<i32>>::new(StableVec::new());
    common::test_apply_edit_at_generated_indexes(recorder);
}

#[test]
fn test_apply_edit_at_specified_indexes() {
    let recorder = Recorder::<usize, i32, StableVec<i32>, StableVec<i32>>::new(StableVec::new());
    common::test_apply_edit_at_specified_indexes(recorder);
}

#[test]
fn test_insert_and_remove_at_generated_indexes() {
    let recorder = Recorder::<usize, i32, StableVec<i32>, StableVec<i32>>::new(StableVec::new());
    common::test_insert_and_remove_at_generated_indexes(recorder);
}

#[test]
fn test_insert_and_remove_at_specified_indexes() {
    let recorder = Recorder::<usize, i32, StableVec<i32>, StableVec<i32>>::new(StableVec::new());
    common::test_insert_and_remove_at_specified_indexes(recorder);
}

#[test]
fn test_edit_undo_redo_at_generated_indexes() {
    common::test_edit_undo_redo_at_generated_indexes::<usize, StableVec<i32>, StableVec<i32>>(
        StableVec::new(),
    );
}

#[test]
fn test_edit_undo_redo_at_specified_indexes() {
    common::test_edit_undo_redo_at_specified_indexes::<usize, StableVec<i32>, StableVec<i32>>(
        StableVec::new(),
    );
}

