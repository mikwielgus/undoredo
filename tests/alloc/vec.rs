// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use alloc::{collections::BTreeMap, vec::Vec};
use undoredo::{ApplyEdit, Edit, Recorder};

#[test]
fn test_apply_edit_and_reverse() {
    let mut recorder = Recorder::<Vec<i32>, BTreeMap<usize, i32>>::new(Vec::new());

    recorder.push(0);
    recorder.push(10);
    recorder.push(20);
    recorder.push(30);
    recorder.push(40);
    recorder.push(50);
    recorder.push(60);

    let edit = Edit::with_removed_inserted(
        BTreeMap::from([(2, 20), (6, 60), (5, 50), (4, 40)]),
        BTreeMap::from([(2, 22), (5, 55), (4, 44)]),
    );
    recorder.apply_edit(&edit);

    assert_eq!(recorder.get(&0), Some(&0));
    assert_eq!(recorder.get(&1), Some(&10));
    assert_eq!(recorder.get(&2), Some(&22));
    assert_eq!(recorder.get(&3), Some(&30));
    assert_eq!(recorder.get(&4), Some(&44));
    assert_eq!(recorder.get(&5), Some(&55));
    assert_eq!(recorder.get(&6), None);

    recorder.apply_edit(&edit.reverse());

    assert_eq!(recorder.get(&0), Some(&0));
    assert_eq!(recorder.get(&1), Some(&10));
    assert_eq!(recorder.get(&2), Some(&20));
    assert_eq!(recorder.get(&3), Some(&30));
    assert_eq!(recorder.get(&4), Some(&40));
    assert_eq!(recorder.get(&5), Some(&50));
    assert_eq!(recorder.get(&6), Some(&60));
}

#[test]
fn test_push_and_pop() {
    let mut recorder = Recorder::<Vec<i32>, BTreeMap<usize, i32>>::new(Vec::new());

    recorder.push(0);
    recorder.push(10);
    recorder.push(20);
    recorder.push(30);
    recorder.push(40);
    recorder.push(50);
    recorder.push(60);

    recorder.pop();
    recorder.pop();

    assert_eq!(recorder.get(&0), Some(&0));
    assert_eq!(recorder.get(&1), Some(&10));
    assert_eq!(recorder.get(&2), Some(&20));
    assert_eq!(recorder.get(&3), Some(&30));
    assert_eq!(recorder.get(&4), Some(&40));
    assert_eq!(recorder.get(&5), None);
    assert_eq!(recorder.get(&6), None);
}
