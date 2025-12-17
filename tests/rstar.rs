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
fn test_apply_edit_on_set() {
    let recorder = Recorder::<(i32, i32), (), RTree<(i32, i32)>, RTree<(i32, i32)>>::new(
        RTree::new(),
    );
    common::test_apply_edit_on_set(recorder);
}

#[test]
fn test_insert_and_remove_on_set() {
    let recorder = Recorder::<(i32, i32), (), RTree<(i32, i32)>, RTree<(i32, i32)>>::new(
        RTree::new(),
    );
    common::test_insert_and_remove_on_set(recorder);
}

#[test]
fn test_edit_undo_redo_on_set() {
    common::test_edit_undo_redo_on_set::<(i32, i32), RTree<(i32, i32)>, RTree<(i32, i32)>>(
        RTree::new(),
    );
}

