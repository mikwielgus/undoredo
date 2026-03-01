// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "std")]

#[path = "../common/mod.rs"]
mod common;

use std::collections::HashSet;

use undoredo::Recorder;

#[test]
fn test_apply_delta_on_set() {
    let recorder = Recorder::<HashSet<i32>>::new(HashSet::new());
    common::test_apply_delta_on_set(recorder);
}

#[test]
fn test_insert_and_remove_on_set() {
    let recorder = Recorder::<HashSet<i32>>::new(HashSet::new());
    common::test_insert_and_remove_on_set(recorder);
}

#[test]
fn test_undo_redo_on_set() {
    common::test_undo_redo_on_set::<i32, HashSet<i32>, HashSet<i32>>(HashSet::new());
}
