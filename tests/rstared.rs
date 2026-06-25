// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "rstared")]

use std::collections::HashMap;

use rstar::primitives::Rectangle;
use rstared::RTreed;
use undoredo::Recorder;

#[path = "common/mod.rs"]
mod common;

impl common::FromUsize for (i32, i32) {
    fn from_usize(u: usize) -> (i32, i32) {
        (u as i32, 0)
    }
}

impl common::FromUsize for Rectangle<(i32, i32)> {
    fn from_usize(u: usize) -> Rectangle<(i32, i32)> {
        Rectangle::from_corners((u as i32, 0), (u as i32, 0))
    }
}

#[test]
fn test_recorder_apply_delta_on_set() {
    let rect_hashmap: HashMap<i32, Rectangle<(i32, i32)>> = HashMap::new();
    let recorder = Recorder::<
        RTreed<HashMap<i32, Rectangle<(i32, i32)>>>,
        HashMap<i32, Rectangle<(i32, i32)>>,
    >::new(RTreed::new(rect_hashmap));
    common::test_recorder_apply_delta_at_specified_indices(recorder);
}

#[test]
fn test_snapshot_undo_redo() {
    let rect_hashmap: HashMap<i32, Rectangle<(i32, i32)>> = HashMap::new();
    common::test_snapshot_undo_redo::<i32, Rectangle<(i32, i32)>, _>(RTreed::new(rect_hashmap));
}

#[test]
fn test_tree_checkout_between_branches() {
    common::test_tree_checkout_between_branches();
}
