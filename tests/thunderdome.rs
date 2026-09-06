// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "thunderdome")]

#[path = "common/mod.rs"]
mod common;

use thunderdome::{Arena, Index};
use undoredo::Recorder;

#[test]
fn test_recorder_apply_delta_at_generated_indices() {
    let recorder = Recorder::<Arena<i32>>::new(Arena::new());
    common::test_recorder_apply_delta_at_generated_indices::<Index, _, _>(recorder);
}

#[test]
fn test_insert_and_remove_at_generated_indices() {
    let recorder = Recorder::<Arena<i32>>::new(Arena::new());
    common::test_insert_and_remove_at_generated_indices::<Index, _, _>(recorder);
}

#[test]
fn test_delta_undo_redo_at_generated_indices() {
    common::test_delta_undo_redo_at_generated_indices::<Index, Arena<i32>, Arena<i32>>(Arena::new());
}

#[test]
fn test_delta_undo_redo_clear_at_generated_indices() {
    common::test_delta_undo_redo_clear_at_generated_indices::<Index, Arena<i32>, Arena<i32>>(
        Arena::new(),
    );
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo_vec::<Index, Arena<i32>>(Arena::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
