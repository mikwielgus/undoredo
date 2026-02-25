// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "thunderdome")]

#[path = "common/mod.rs"]
mod common;

use thunderdome::{Arena, Index};
use undoredo::Recorder;

#[test]
fn test_apply_edit_at_generated_indices() {
    let recorder = Recorder::<Arena<i32>>::new(Arena::new());
    common::test_apply_edit_at_generated_indices::<Index, _, _>(recorder);
}

#[test]
fn test_insert_and_remove_at_generated_indices() {
    let recorder = Recorder::<Arena<i32>>::new(Arena::new());
    common::test_insert_and_remove_at_generated_indices::<Index, _, _>(recorder);
}

#[test]
fn test_edit_undo_redo_at_generated_indices() {
    common::test_edit_undo_redo_at_generated_indices::<Index, Arena<i32>, Arena<i32>>(Arena::new());
}
