// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::vec::Vec;

use undoredo::{
    ApplyDelta, BTreeMapHalfDelta, BTreeSetHalfDelta, Container, Delta, Get, Insert, IntoIter,
    Push, Recorder, Remove, Snapshot, UndoRedo,
};

pub(crate) trait Keyed<K>: Container<Key = K> {}
impl<T: Container<Key = K>, K> Keyed<K> for T {}

pub(crate) trait Map<V>: Container<Value = V> {}
impl<T: Container<Value = V>, V> Map<V> for T {}

pub trait FromUsize {
    fn from_usize(u: usize) -> Self;
}

impl FromUsize for i32 {
    fn from_usize(u: usize) -> i32 {
        u.try_into().unwrap()
    }
}

impl FromUsize for usize {
    fn from_usize(u: usize) -> usize {
        u
    }
}

pub fn test_recorder_apply_delta_at_generated_indices<
    K: Ord + Clone,
    C: Keyed<K> + Map<i32> + Get<K> + Insert<K> + Remove<K> + Push<K>,
    DC: Clone + Keyed<K> + Map<i32> + Get<K> + Insert<K> + IntoIter<K> + Remove<K>,
>(
    mut recorder: Recorder<C, DC>,
) where
    C: ApplyDelta<BTreeMapHalfDelta<K, i32>>,
{
    let first = recorder.push(10);
    let second = recorder.push(20);
    let third = recorder.push(30);
    let fourth = recorder.push(40);
    let fifth = recorder.push(50);
    let sixth = recorder.push(60);
    recorder.remove(&sixth);

    let delta = Delta::with_removed_inserted(
        BTreeMap::from([(second.clone(), 20)]),
        BTreeMap::from([(third.clone(), 33), (sixth.clone(), 66)]),
    );
    recorder.apply_delta(delta);

    assert_eq!(recorder.get(&first), Some(&10));
    assert_eq!(recorder.get(&second), None);
    assert_eq!(recorder.get(&third), Some(&33));
    assert_eq!(recorder.get(&fourth), Some(&40));
    assert_eq!(recorder.get(&fifth), Some(&50));
    assert_eq!(recorder.get(&sixth), Some(&66));
}

pub fn test_recorder_apply_delta_at_specified_indices<
    K: Clone + FromUsize + std::fmt::Debug + PartialEq + Ord,
    V: Clone + FromUsize + std::fmt::Debug + PartialEq + Ord,
    C: Keyed<K> + Map<V> + Insert<K> + Remove<K> + Get<K>,
    DC: Clone + Keyed<K> + Map<V> + Get<K> + Insert<K> + IntoIter<K> + Remove<K>,
>(
    mut recorder: Recorder<C, DC>,
) where
    C: ApplyDelta<BTreeMapHalfDelta<K, V>>,
{
    recorder.insert(K::from_usize(1), V::from_usize(10));
    recorder.insert(K::from_usize(2), V::from_usize(20));
    recorder.insert(K::from_usize(3), V::from_usize(30));
    recorder.insert(K::from_usize(4), V::from_usize(40));
    recorder.insert(K::from_usize(5), V::from_usize(50));

    let delta = Delta::with_removed_inserted(
        BTreeMap::from([(K::from_usize(2), V::from_usize(20))]),
        BTreeMap::from([
            (K::from_usize(3), V::from_usize(33)),
            (K::from_usize(6), V::from_usize(66)),
        ]),
    );
    recorder.apply_delta(delta);

    assert_eq!(recorder.get(&K::from_usize(1)), Some(&V::from_usize(10)));
    assert_eq!(recorder.get(&K::from_usize(2)), None);
    assert_eq!(recorder.get(&K::from_usize(3)), Some(&V::from_usize(33)));
    assert_eq!(recorder.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(recorder.get(&K::from_usize(5)), Some(&V::from_usize(50)));
    assert_eq!(recorder.get(&K::from_usize(6)), Some(&V::from_usize(66)));
}

pub fn test_recorder_apply_delta_on_set<
    K: Clone + FromUsize + Ord,
    C: Keyed<K> + Map<()> + Insert<K> + Remove<K> + Get<K>,
    DC: Clone + Keyed<K> + Map<()> + Get<K> + Insert<K> + IntoIter<K> + Remove<K>,
>(
    mut recorder: Recorder<C, DC>,
) where
    C: ApplyDelta<BTreeSetHalfDelta<K>>,
{
    recorder.insert(K::from_usize(10), ());
    recorder.insert(K::from_usize(20), ());
    recorder.insert(K::from_usize(30), ());
    recorder.insert(K::from_usize(40), ());
    recorder.insert(K::from_usize(50), ());

    let delta = Delta::with_removed_inserted(
        BTreeMap::from([(K::from_usize(20), ())]),
        BTreeMap::from([(K::from_usize(30), ()), (K::from_usize(60), ())]),
    );
    recorder.apply_delta(delta);

    assert_eq!(recorder.get(&K::from_usize(10)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(20)), None);
    assert_eq!(recorder.get(&K::from_usize(30)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(40)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(50)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(60)), Some(&()));
}

pub fn test_insert_and_remove_at_generated_indices<
    K: Clone,
    C: Keyed<K> + Map<i32> + Insert<K> + Remove<K> + Push<K> + Get<K>,
    DC: Keyed<K> + Map<i32> + Get<K> + Insert<K> + Remove<K>,
>(
    mut recorder: Recorder<C, DC>,
) {
    let first = recorder.push(10);
    let second = recorder.push(20);
    let third = recorder.push(30);
    let fourth = recorder.push(40);
    let fifth = recorder.push(50);
    let sixth = recorder.push(60);
    recorder.remove(&sixth);

    recorder.remove(&second);
    recorder.insert(first.clone(), 11);
    recorder.remove(&fourth);
    recorder.insert(sixth.clone(), 66);

    assert_eq!(recorder.get(&first), Some(&11));
    assert_eq!(recorder.get(&second), None);
    assert_eq!(recorder.get(&third), Some(&30));
    assert_eq!(recorder.get(&fourth), None);
    assert_eq!(recorder.get(&fifth), Some(&50));
    assert_eq!(recorder.get(&sixth), Some(&66));
}

pub fn test_recorder_insert_and_remove_at_specified_indices<
    C: Keyed<usize> + Map<i32> + Insert<usize> + Remove<usize> + Get<usize>,
    DC: Keyed<usize> + Map<i32> + Get<usize> + Insert<usize> + Remove<usize>,
>(
    mut recorder: Recorder<C, DC>,
) {
    recorder.insert(1, 10);
    recorder.insert(2, 20);
    recorder.insert(3, 30);
    recorder.insert(4, 40);
    recorder.insert(5, 50);
    recorder.remove(&2);
    recorder.insert(1, 11);
    recorder.remove(&4);
    recorder.insert(6, 60);

    assert_eq!(recorder.get(&1), Some(&11));
    assert_eq!(recorder.get(&2), None);
    assert_eq!(recorder.get(&3), Some(&30));
    assert_eq!(recorder.get(&4), None);
    assert_eq!(recorder.get(&5), Some(&50));
    assert_eq!(recorder.get(&6), Some(&60));
}

pub fn test_insert_and_remove_on_set<
    K: Clone + FromUsize,
    C: Keyed<K> + Map<()> + Insert<K> + Remove<K> + Get<K>,
    DC: Keyed<K> + Map<()> + Get<K> + Insert<K> + Remove<K>,
>(
    mut recorder: Recorder<C, DC>,
) {
    recorder.insert(K::from_usize(10), ());
    recorder.insert(K::from_usize(20), ());
    recorder.insert(K::from_usize(30), ());
    recorder.insert(K::from_usize(40), ());
    recorder.insert(K::from_usize(50), ());
    recorder.insert(K::from_usize(60), ());

    recorder.remove(&K::from_usize(20));
    recorder.remove(&K::from_usize(40));

    assert_eq!(recorder.get(&K::from_usize(10)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(20)), None);
    assert_eq!(recorder.get(&K::from_usize(30)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(40)), None);
    assert_eq!(recorder.get(&K::from_usize(50)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(60)), Some(&()));
}

pub fn test_delta_undo_redo_at_generated_indices<
    K: Clone,
    C: Keyed<K> + Map<i32> + Get<K> + Insert<K> + Remove<K> + Push<K> + IntoIter<K> + ApplyDelta<DC>,
    DC: Clone + Default + Keyed<K> + Map<i32> + Get<K> + Insert<K> + IntoIter<K> + Remove<K>,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<Delta<DC>> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);
    assert_eq!(undoredo.redo(&mut container), None);

    let mut indices = Vec::new();

    let mut container = undoredo.edit(container, |recorder| {
        indices.push(recorder.push(10));
        // Repeat the same index to start indexing from 1 like in the test with specified indices.
        indices.push(indices[0].clone());

        indices.push(recorder.push(20));
        indices.push(recorder.push(30));
        indices.push(recorder.push(40));
        indices.push(recorder.push(50));

        indices.push(recorder.push(60));
        recorder.remove(&indices[6]);
    });

    assert_eq!(undoredo.redo(&mut container), None);

    assert_eq!(container.get(&indices[1]), Some(&10));
    assert_eq!(container.get(&indices[2]), Some(&20));
    assert_eq!(container.get(&indices[3]), Some(&30));
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&indices[2]);
        recorder.insert(indices[1].clone(), 11);
        recorder.insert(indices[3].clone(), 33);
    });

    assert_eq!(container.get(&indices[1]), Some(&11));
    assert_eq!(container.get(&indices[2]), None);
    assert_eq!(container.get(&indices[3]), Some(&33));
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(&indices[1]), Some(&10));
    assert_eq!(container.get(&indices[2]), Some(&20));
    assert_eq!(container.get(&indices[3]), Some(&30));
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&indices[1]), Some(&11));
    assert_eq!(container.get(&indices[2]), None);
    assert_eq!(container.get(&indices[3]), Some(&33));
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&indices[3]);
        recorder.insert(indices[6].clone(), 60);
    });

    assert_eq!(container.get(&indices[1]), Some(&11));
    assert_eq!(container.get(&indices[2]), None);
    assert_eq!(container.get(&indices[3]), None);
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));
    assert_eq!(container.get(&indices[6]), Some(&60));

    assert_eq!(undoredo.redo(&mut container), None);

    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&indices[1]), Some(&10));
    assert_eq!(container.get(&indices[2]), Some(&20));
    assert_eq!(container.get(&indices[3]), Some(&30));
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&indices[1]), Some(&11));
    assert_eq!(container.get(&indices[2]), None);
    assert_eq!(container.get(&indices[3]), Some(&33));
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));
}

pub fn test_delta_undo_redo_at_specified_indices<
    K: Clone + FromUsize + std::fmt::Debug + PartialEq,
    V: Clone + FromUsize + std::fmt::Debug + PartialEq,
    C: Keyed<K> + Map<V> + Get<K> + Insert<K> + IntoIter<K> + Remove<K> + ApplyDelta<DC>,
    DC: Clone + Default + Keyed<K> + Map<V> + Get<K> + Insert<K> + IntoIter<K> + Remove<K>,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<Delta<DC>> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);
    assert_eq!(undoredo.redo(&mut container), None);

    let mut container = undoredo.edit(container, |recorder| {
        recorder.insert(K::from_usize(1), V::from_usize(10));
        recorder.insert(K::from_usize(2), V::from_usize(20));
        recorder.insert(K::from_usize(3), V::from_usize(30));
        recorder.insert(K::from_usize(4), V::from_usize(40));
        recorder.insert(K::from_usize(5), V::from_usize(50));
    });

    assert_eq!(undoredo.redo(&mut container), None);

    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(10)));
    assert_eq!(container.get(&K::from_usize(2)), Some(&V::from_usize(20)));
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(30)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&K::from_usize(2));
        recorder.insert(K::from_usize(1), V::from_usize(11));
        recorder.insert(K::from_usize(3), V::from_usize(33));
    });

    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(11)));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(33)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(10)));
    assert_eq!(container.get(&K::from_usize(2)), Some(&V::from_usize(20)));
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(30)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(11)));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(33)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&K::from_usize(3));
        recorder.insert(K::from_usize(6), V::from_usize(60));
    });

    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(11)));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), None);
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));
    assert_eq!(container.get(&K::from_usize(6)), Some(&V::from_usize(60)));

    assert_eq!(undoredo.redo(&mut container), None);

    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(10)));
    assert_eq!(container.get(&K::from_usize(2)), Some(&V::from_usize(20)));
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(30)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(11)));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(33)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));
}

pub fn test_delta_undo_redo_on_set<
    K: Clone + FromUsize,
    C: Keyed<K> + Map<()> + Get<K> + Insert<K> + IntoIter<K> + Remove<K> + ApplyDelta<DC>,
    DC: Clone + Default + Keyed<K> + Map<()> + Get<K> + Insert<K> + IntoIter<K> + Remove<K>,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<Delta<DC>> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);
    assert_eq!(undoredo.redo(&mut container), None);

    let mut container = undoredo.edit(container, |recorder| {
        recorder.insert(K::from_usize(10), ());
        recorder.insert(K::from_usize(20), ());
        recorder.insert(K::from_usize(30), ());
        recorder.insert(K::from_usize(40), ());
        recorder.insert(K::from_usize(50), ());
    });

    assert_eq!(undoredo.redo(&mut container), None);

    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), Some(&()));
    assert_eq!(container.get(&K::from_usize(30)), Some(&()));
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&K::from_usize(20));
    });

    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), None);
    assert_eq!(container.get(&K::from_usize(30)), Some(&()));
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), Some(&()));
    assert_eq!(container.get(&K::from_usize(30)), Some(&()));
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), None);
    assert_eq!(container.get(&K::from_usize(30)), Some(&()));
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&K::from_usize(30));
        recorder.insert(K::from_usize(60), ());
    });

    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), None);
    assert_eq!(container.get(&K::from_usize(30)), None);
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));
    assert_eq!(container.get(&K::from_usize(60)), Some(&()));

    assert_eq!(undoredo.redo(&mut container), None);

    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), Some(&()));
    assert_eq!(container.get(&K::from_usize(30)), Some(&()));
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), None);
    assert_eq!(container.get(&K::from_usize(30)), Some(&()));
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));
}

pub fn test_snapshot_undo_redo<
    K: Clone + FromUsize + std::fmt::Debug + PartialEq,
    V: Clone + FromUsize + std::fmt::Debug + PartialEq,
    C: Keyed<K> + Map<V> + Get<K> + Insert<K> + IntoIter<K> + Remove<K> + Clone,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<Snapshot<C>> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);
    assert_eq!(undoredo.redo(&mut container), None);

    container.insert(K::from_usize(1), V::from_usize(10));
    container.insert(K::from_usize(2), V::from_usize(20));
    container.insert(K::from_usize(3), V::from_usize(30));
    container.insert(K::from_usize(4), V::from_usize(40));
    container.insert(K::from_usize(5), V::from_usize(50));

    undoredo.commit(&mut container);

    container.remove(&K::from_usize(2));
    container.insert(K::from_usize(1), V::from_usize(11));
    container.insert(K::from_usize(3), V::from_usize(33));

    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(10)));
    assert_eq!(container.get(&K::from_usize(2)), Some(&V::from_usize(20)));
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(30)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());
    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(11)));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(33)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    assert_eq!(undoredo.redo(&mut container), None);
}

pub fn test_snapshot_undo_redo_vec<
    K: Clone,
    C: Keyed<K> + Map<i32> + Get<K> + Insert<K> + Remove<K> + Push<K> + IntoIter<K> + Clone,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<Snapshot<C>> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);

    let mut indices = Vec::new();
    indices.push(container.push(10));
    indices.push(indices[0].clone());
    indices.push(container.push(20));
    indices.push(container.push(30));
    indices.push(container.push(40));
    indices.push(container.push(50));
    indices.push(container.push(60));
    container.remove(&indices[6]);

    undoredo.commit(&mut container);

    container.remove(&indices[2]);
    container.insert(indices[1].clone(), 11);
    container.insert(indices[3].clone(), 33);

    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(undoredo.undo(&mut container), None);

    assert_eq!(container.get(&indices[1]), Some(&10));
    assert_eq!(container.get(&indices[2]), Some(&20));
    assert_eq!(container.get(&indices[3]), Some(&30));
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&indices[1]), Some(&11));
    assert_eq!(container.get(&indices[2]), None);
    assert_eq!(container.get(&indices[3]), Some(&33));
    assert_eq!(container.get(&indices[4]), Some(&40));
    assert_eq!(container.get(&indices[5]), Some(&50));

    assert_eq!(undoredo.redo(&mut container), None);
}

pub fn test_snapshot_undo_redo_set<
    K: Clone + FromUsize,
    C: Keyed<K> + Map<()> + Get<K> + Insert<K> + IntoIter<K> + Remove<K> + Clone,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<Snapshot<C>> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);

    container.insert(K::from_usize(10), ());
    container.insert(K::from_usize(20), ());
    container.insert(K::from_usize(30), ());
    container.insert(K::from_usize(40), ());
    container.insert(K::from_usize(50), ());

    undoredo.commit(&mut container);

    container.remove(&K::from_usize(20));

    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), Some(&()));
    assert_eq!(container.get(&K::from_usize(30)), Some(&()));
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));

    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());
    assert_eq!(container.get(&K::from_usize(10)), Some(&()));
    assert_eq!(container.get(&K::from_usize(20)), None);
    assert_eq!(container.get(&K::from_usize(30)), Some(&()));
    assert_eq!(container.get(&K::from_usize(40)), Some(&()));
    assert_eq!(container.get(&K::from_usize(50)), Some(&()));

    assert_eq!(undoredo.redo(&mut container), None);
}
