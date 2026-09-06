// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "rstar")]

#[path = "common/mod.rs"]
mod common;

use rstar::RTree;
use undoredo::Recorder;

impl common::FromUsize for (i32, i32) {
    fn from_usize(u: usize) -> (i32, i32) {
        (u as i32, 0)
    }
}

#[test]
fn test_recorder_apply_delta_on_set() {
    let recorder = Recorder::<RTree<(i32, i32)>>::new(RTree::new());
    common::test_recorder_apply_delta_on_set(recorder);
}

#[test]
fn test_insert_and_remove_on_set() {
    let recorder = Recorder::<RTree<(i32, i32)>>::new(RTree::new());
    common::test_insert_and_remove_on_set(recorder);
}

#[test]
fn test_delta_undo_redo_on_set() {
    common::test_delta_undo_redo_on_set::<(i32, i32), RTree<(i32, i32)>, RTree<(i32, i32)>>(
        RTree::new(),
    );
}

#[test]
fn test_delta_undo_redo_clear() {
    common::test_delta_undo_redo_clear::<(i32, i32), (), RTree<(i32, i32)>, RTree<(i32, i32)>>(
        RTree::new(),
    );
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo_set::<(i32, i32), RTree<(i32, i32)>>(RTree::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
