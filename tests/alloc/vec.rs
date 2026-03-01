// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use alloc::{collections::BTreeMap, vec::Vec};
use undoredo::{ApplyDelta, Delta, Recorder, UndoRedo};

#[test]
fn test_apply_delta_and_reverse() {
    let mut recorder = Recorder::<Vec<i32>, BTreeMap<usize, i32>>::new(Vec::new());

    recorder.push(0);
    recorder.push(10);
    recorder.push(20);
    recorder.push(30);
    recorder.push(40);
    recorder.push(50);
    recorder.push(60);

    assert_eq!(recorder.get(&0), Some(&0));
    assert_eq!(recorder.get(&1), Some(&10));
    assert_eq!(recorder.get(&2), Some(&20));
    assert_eq!(recorder.get(&3), Some(&30));
    assert_eq!(recorder.get(&4), Some(&40));
    assert_eq!(recorder.get(&5), Some(&50));
    assert_eq!(recorder.get(&6), Some(&60));

    let delta = Delta::with_removed_inserted(
        BTreeMap::from([(2, 20), (6, 60), (5, 50), (4, 40)]),
        BTreeMap::from([(2, 22), (5, 55), (4, 44)]),
    );
    recorder.apply_delta(&delta);

    assert_eq!(recorder.get(&0), Some(&0));
    assert_eq!(recorder.get(&1), Some(&10));
    assert_eq!(recorder.get(&2), Some(&22));
    assert_eq!(recorder.get(&3), Some(&30));
    assert_eq!(recorder.get(&4), Some(&44));
    assert_eq!(recorder.get(&5), Some(&55));
    assert_eq!(recorder.get(&6), None);

    recorder.apply_delta(&delta.reverse());

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

#[test]
fn test_undo_redo() {
    let collection: Vec<i32> = Vec::new();
    let mut undoredo: UndoRedo<BTreeMap<usize, i32>> = UndoRedo::new();

    let mut collection = undoredo.delta(collection, |recorder| {
        recorder.push(0);
        recorder.push(10);

        recorder.push(20);
        recorder.push(30);
        recorder.push(40);
        recorder.push(50);

        recorder.push(60);
        recorder.pop();
    });

    assert_eq!(undoredo.redo(&mut collection), None);

    assert_eq!(collection.get(0), Some(&0));
    assert_eq!(collection.get(1), Some(&10));
    assert_eq!(collection.get(2), Some(&20));
    assert_eq!(collection.get(3), Some(&30));
    assert_eq!(collection.get(4), Some(&40));
    assert_eq!(collection.get(5), Some(&50));
    assert_eq!(collection.get(6), None);

    let mut collection = undoredo.delta(collection, |recorder| {
        recorder.insert(1, 11);
        recorder.insert(3, 33);
        recorder.pop();
    });

    assert_eq!(collection.get(0), Some(&0));
    assert_eq!(collection.get(1), Some(&11));
    assert_eq!(collection.get(2), Some(&20));
    assert_eq!(collection.get(3), Some(&33));
    assert_eq!(collection.get(4), Some(&40));
    assert_eq!(collection.get(5), None);
    assert_eq!(collection.get(6), None);

    assert!(undoredo.undo(&mut collection).is_some());

    assert_eq!(collection.get(0), Some(&0));
    assert_eq!(collection.get(1), Some(&10));
    assert_eq!(collection.get(2), Some(&20));
    assert_eq!(collection.get(3), Some(&30));
    assert_eq!(collection.get(4), Some(&40));
    assert_eq!(collection.get(5), Some(&50));
    assert_eq!(collection.get(6), None);

    assert!(undoredo.redo(&mut collection).is_some());

    assert_eq!(collection.get(0), Some(&0));
    assert_eq!(collection.get(1), Some(&11));
    assert_eq!(collection.get(2), Some(&20));
    assert_eq!(collection.get(3), Some(&33));
    assert_eq!(collection.get(4), Some(&40));
    assert_eq!(collection.get(5), None);
    assert_eq!(collection.get(6), None);

    let mut collection = undoredo.delta(collection, |recorder| {
        recorder.push(50);
        recorder.push(60);
    });

    assert_eq!(collection.get(0), Some(&0));
    assert_eq!(collection.get(1), Some(&11));
    assert_eq!(collection.get(2), Some(&20));
    assert_eq!(collection.get(3), Some(&33));
    assert_eq!(collection.get(4), Some(&40));
    assert_eq!(collection.get(5), Some(&50));
    assert_eq!(collection.get(6), Some(&60));

    assert_eq!(undoredo.redo(&mut collection), None);

    assert!(undoredo.undo(&mut collection).is_some());
    assert!(undoredo.undo(&mut collection).is_some());
    assert!(undoredo.undo(&mut collection).is_some());
    assert_eq!(undoredo.undo(&mut collection), None);

    assert!(undoredo.redo(&mut collection).is_some());

    assert_eq!(collection.get(0), Some(&0));
    assert_eq!(collection.get(1), Some(&10));
    assert_eq!(collection.get(2), Some(&20));
    assert_eq!(collection.get(3), Some(&30));
    assert_eq!(collection.get(4), Some(&40));
    assert_eq!(collection.get(5), Some(&50));
    assert_eq!(collection.get(6), None);

    assert!(undoredo.redo(&mut collection).is_some());

    assert_eq!(collection.get(0), Some(&0));
    assert_eq!(collection.get(1), Some(&11));
    assert_eq!(collection.get(2), Some(&20));
    assert_eq!(collection.get(3), Some(&33));
    assert_eq!(collection.get(4), Some(&40));
    assert_eq!(collection.get(5), None);
    assert_eq!(collection.get(6), None);
}
