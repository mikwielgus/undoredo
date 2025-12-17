#![cfg(feature = "thunderdome")]

#[path = "common/mod.rs"]
mod common;

use thunderdome::{Arena, Index};
use undoredo::Recorder;

#[test]
fn test_apply_edit_at_generated_indexes() {
    let recorder = Recorder::<Index, i32, Arena<i32>, Arena<i32>>::new(Arena::new());
    common::test_apply_edit_at_generated_indexes(recorder);
}

#[test]
fn test_insert_and_remove_at_generated_indexes() {
    let recorder = Recorder::<Index, i32, Arena<i32>, Arena<i32>>::new(Arena::new());
    common::test_insert_and_remove_at_generated_indexes(recorder);
}

#[test]
fn test_edit_undo_redo_at_generated_indexes() {
    common::test_edit_undo_redo_at_generated_indexes::<Index, Arena<i32>, Arena<i32>>(
        Arena::new(),
    );
}

