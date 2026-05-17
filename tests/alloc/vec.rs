// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "../common/mod.rs"]
mod common;

use alloc::{collections::BTreeMap, vec::Vec};
use undoredo::aliases::{VecDelta, VecHalfDelta};
use undoredo::{ApplyDelta, Delta, Recorder, UndoRedo};

#[test]
fn test_recorder_apply_delta_and_reverse() {
    let mut recorder = Recorder::<Vec<i32>, VecHalfDelta<i32>>::new(Vec::new());

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

    let delta: VecDelta<i32> = Delta::with_removed_inserted(
        BTreeMap::from([(2, 20), (6, 60), (5, 50), (4, 40)]),
        BTreeMap::from([(2, 22), (5, 55), (4, 44)]),
    );
    recorder.apply_delta(delta.clone());

    assert_eq!(recorder.get(&0), Some(&0));
    assert_eq!(recorder.get(&1), Some(&10));
    assert_eq!(recorder.get(&2), Some(&22));
    assert_eq!(recorder.get(&3), Some(&30));
    assert_eq!(recorder.get(&4), Some(&44));
    assert_eq!(recorder.get(&5), Some(&55));
    assert_eq!(recorder.get(&6), None);

    recorder.apply_delta(delta.reverse());

    assert_eq!(recorder.get(&0), Some(&0));
    assert_eq!(recorder.get(&1), Some(&10));
    assert_eq!(recorder.get(&2), Some(&20));
    assert_eq!(recorder.get(&3), Some(&30));
    assert_eq!(recorder.get(&4), Some(&40));
    assert_eq!(recorder.get(&5), Some(&50));
    assert_eq!(recorder.get(&6), Some(&60));
}

#[test]
fn test_recorder_push_and_pop() {
    let mut recorder = Recorder::<Vec<i32>, VecHalfDelta<i32>>::new(Vec::new());

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
fn test_delta_undo_redo() {
    let container: Vec<i32> = Vec::new();
    let mut undoredo: UndoRedo<VecDelta<i32>> = UndoRedo::new();

    let mut container = undoredo.edit(container, |recorder| {
        recorder.push(0);
        recorder.push(10);

        recorder.push(20);
        recorder.push(30);
        recorder.push(40);
        recorder.push(50);

        recorder.push(60);
        recorder.pop();
    });

    assert_eq!(undoredo.redo(&mut container), None);

    assert_eq!(container.get(0), Some(&0));
    assert_eq!(container.get(1), Some(&10));
    assert_eq!(container.get(2), Some(&20));
    assert_eq!(container.get(3), Some(&30));
    assert_eq!(container.get(4), Some(&40));
    assert_eq!(container.get(5), Some(&50));
    assert_eq!(container.get(6), None);

    let mut container = undoredo.edit(container, |recorder| {
        recorder.set(1, 11);
        recorder.set(3, 33);
        recorder.pop();
    });

    assert_eq!(container.get(0), Some(&0));
    assert_eq!(container.get(1), Some(&11));
    assert_eq!(container.get(2), Some(&20));
    assert_eq!(container.get(3), Some(&33));
    assert_eq!(container.get(4), Some(&40));
    assert_eq!(container.get(5), None);
    assert_eq!(container.get(6), None);

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(0), Some(&0));
    assert_eq!(container.get(1), Some(&10));
    assert_eq!(container.get(2), Some(&20));
    assert_eq!(container.get(3), Some(&30));
    assert_eq!(container.get(4), Some(&40));
    assert_eq!(container.get(5), Some(&50));
    assert_eq!(container.get(6), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(0), Some(&0));
    assert_eq!(container.get(1), Some(&11));
    assert_eq!(container.get(2), Some(&20));
    assert_eq!(container.get(3), Some(&33));
    assert_eq!(container.get(4), Some(&40));
    assert_eq!(container.get(5), None);
    assert_eq!(container.get(6), None);

    let mut container = undoredo.edit(container, |recorder| {
        recorder.push(50);
        recorder.push(60);
    });

    assert_eq!(container.get(0), Some(&0));
    assert_eq!(container.get(1), Some(&11));
    assert_eq!(container.get(2), Some(&20));
    assert_eq!(container.get(3), Some(&33));
    assert_eq!(container.get(4), Some(&40));
    assert_eq!(container.get(5), Some(&50));
    assert_eq!(container.get(6), Some(&60));

    assert_eq!(undoredo.redo(&mut container), None);

    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(0), Some(&0));
    assert_eq!(container.get(1), Some(&10));
    assert_eq!(container.get(2), Some(&20));
    assert_eq!(container.get(3), Some(&30));
    assert_eq!(container.get(4), Some(&40));
    assert_eq!(container.get(5), Some(&50));
    assert_eq!(container.get(6), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(0), Some(&0));
    assert_eq!(container.get(1), Some(&11));
    assert_eq!(container.get(2), Some(&20));
    assert_eq!(container.get(3), Some(&33));
    assert_eq!(container.get(4), Some(&40));
    assert_eq!(container.get(5), None);
    assert_eq!(container.get(6), None);
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
}
