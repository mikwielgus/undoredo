// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(dead_code)]

// TODO: Tests for `ExtendDelta`.

use std::collections::BTreeMap;
use std::vec::Vec;

use undoredo::aliases::{BTreeMapHalfDelta, BTreeSetHalfDelta};
use undoredo::maplike::abc::{Container, Keyed};
use undoredo::maplike::iter::IntoIter;
use undoredo::maplike::ops::{Get, Insert, Len, Pop, Push, Remove, Set, SwapRemove};
use undoredo::{ApplyDelta, Delta, HistoryTree, Recorder, Snapshot, UndoRedo};

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
    C: Keyed<Key = K> + Map<i32> + Get<K> + Insert<K> + Remove<K, Output = Option<i32>> + Push<K>,
    DC: Clone
        + Keyed<Key = K>
        + Map<i32>
        + Get<K>
        + Insert<K>
        + IntoIter<K>
        + Remove<K, Output = Option<i32>>,
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
    C: Keyed<Key = K> + Map<V> + Insert<K> + Remove<K> + Get<K>,
    DC: Clone
        + Keyed<Key = K>
        + Map<V>
        + Get<K>
        + Insert<K>
        + IntoIter<K>
        + Remove<K, Output = Option<V>>,
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
    C: Keyed<Key = K> + Map<()> + Insert<K> + Remove<K> + Get<K>,
    DC: Clone
        + Keyed<Key = K>
        + Map<()>
        + Get<K>
        + Insert<K>
        + IntoIter<K>
        + Remove<K, Output = Option<()>>,
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
    C: Keyed<Key = K> + Map<i32> + Insert<K> + Remove<K, Output = Option<i32>> + Push<K> + Get<K>,
    DC: Keyed<Key = K> + Map<i32> + Get<K> + Insert<K> + Remove<K, Output = Option<i32>>,
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
    C: Keyed<Key = usize>
        + Map<i32>
        + Insert<usize>
        + Remove<usize, Output = Option<i32>>
        + Get<usize>,
    DC: Keyed<Key = usize>
        + Map<i32>
        + Get<usize>
        + Insert<usize>
        + Remove<usize, Output = Option<i32>>,
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
    C: Keyed<Key = K> + Map<()> + Insert<K> + Remove<K, Output = Option<()>> + Get<K>,
    DC: Keyed<Key = K> + Map<()> + Get<K> + Insert<K> + Remove<K, Output = Option<()>>,
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
    C: Keyed<Key = K>
        + Map<i32>
        + Get<K>
        + Insert<K>
        + Remove<K, Output = Option<i32>>
        + Push<K>
        + IntoIter<K>
        + ApplyDelta<DC>,
    DC: Clone
        + Default
        + Keyed<Key = K>
        + Map<i32>
        + Get<K>
        + Insert<K>
        + IntoIter<K>
        + Remove<K, Output = Option<i32>>,
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
    C: Keyed<Key = K>
        + Map<V>
        + Get<K>
        + Insert<K>
        + IntoIter<K>
        + Remove<K, Output = Option<V>>
        + ApplyDelta<DC>,
    DC: Clone
        + Default
        + Keyed<Key = K>
        + Map<V>
        + Get<K>
        + Insert<K>
        + IntoIter<K>
        + Remove<K, Output = Option<V>>,
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
    C: Keyed<Key = K>
        + Map<()>
        + Get<K>
        + Insert<K>
        + IntoIter<K>
        + Remove<K, Output = Option<()>>
        + ApplyDelta<DC>,
    DC: Clone
        + Default
        + Keyed<Key = K>
        + Map<()>
        + Get<K>
        + Insert<K>
        + IntoIter<K>
        + Remove<K, Output = Option<()>>,
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
    C: Keyed<Key = K> + Map<V> + Get<K> + Insert<K> + IntoIter<K> + Remove<K> + Clone,
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
    C: Keyed<Key = K> + Map<i32> + Get<K> + Insert<K> + Remove<K> + Push<K> + IntoIter<K> + Clone,
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
    C: Keyed<Key = K> + Map<()> + Get<K> + Insert<K> + IntoIter<K> + Remove<K> + Clone,
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

pub fn test_history_tree_command_checkout() {
    let mut history_tree = HistoryTree::<(), u8>::new();
    let mut state = ();

    history_tree.cmd_commit(1, &mut state);
    history_tree.cmd_commit(3, &mut state);
    let left_leaf = history_tree.curr_node();

    assert_eq!(history_tree.undo(&mut state), Some(3));
    assert_eq!(history_tree.undo(&mut state), Some(1));

    history_tree.cmd_commit(2, &mut state);
    history_tree.cmd_commit(4, &mut state);
    let right_leaf = history_tree.curr_node();

    assert_eq!(history_tree.checkout(&mut state, left_leaf), vec![1, 3]);
    assert_eq!(history_tree.curr_node(), left_leaf);

    assert_eq!(history_tree.checkout(&mut state, right_leaf), vec![2, 4]);
    assert_eq!(history_tree.curr_node(), right_leaf);

    // Checkouting to the same node again results in no commands emitted.
    assert_eq!(history_tree.checkout(&mut state, right_leaf), vec![]);
    assert_eq!(history_tree.curr_node(), right_leaf);
}

pub fn test_recorder_apply_delta_and_reverse<
    C: Keyed<Key = usize> + Map<i32> + Get<usize> + Push<usize> + ApplyDelta<BTreeMap<usize, i32>>,
>(
    mut recorder: Recorder<C, BTreeMap<usize, i32>>,
) {
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

    let delta = Delta::<BTreeMap<usize, i32>>::with_removed_inserted(
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

pub fn test_recorder_push_and_pop<
    C: Keyed<Key = usize> + Map<i32> + Get<usize> + Push<usize> + Pop + Len,
    DC: Keyed<Key = usize>
        + Map<i32>
        + Get<usize>
        + Insert<usize>
        + Remove<usize, Output = Option<i32>>,
>(
    mut recorder: Recorder<C, DC>,
) {
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

pub fn test_recorder_swap_remove<
    C: Keyed<Key = usize>
        + Map<i32>
        + Get<usize>
        + Push<usize>
        + Len
        + SwapRemove<usize, Output = i32>,
    DC: Keyed<Key = usize>
        + Map<i32>
        + Get<usize>
        + Insert<usize>
        + Remove<usize, Output = Option<i32>>,
>(
    mut recorder: Recorder<C, DC>,
) {
    recorder.push(10);
    recorder.push(20);
    recorder.push(30);
    recorder.push(40);
    recorder.push(50);

    assert_eq!(recorder.swap_remove(&1), 20);
    assert_eq!(recorder.get(&0), Some(&10));
    assert_eq!(recorder.get(&1), Some(&50));
    assert_eq!(recorder.get(&2), Some(&30));
    assert_eq!(recorder.get(&3), Some(&40));
    assert_eq!(recorder.get(&4), None);

    assert_eq!(recorder.swap_remove(&3), 40);
    assert_eq!(recorder.get(&0), Some(&10));
    assert_eq!(recorder.get(&1), Some(&50));
    assert_eq!(recorder.get(&2), Some(&30));
    assert_eq!(recorder.get(&3), None);
    assert_eq!(recorder.get(&4), None);
}

pub fn test_delta_undo_redo_swap_remove<
    C: Keyed<Key = usize>
        + Map<i32>
        + Get<usize>
        + Push<usize>
        + Len
        + SwapRemove<usize, Output = i32>
        + ApplyDelta<BTreeMap<usize, i32>>,
>(
    container: C,
) {
    let mut undoredo: UndoRedo<Delta<BTreeMap<usize, i32>>> = UndoRedo::new();

    let container = undoredo.edit(container, |recorder| {
        recorder.push(10);
        recorder.push(20);
        recorder.push(30);
        recorder.push(40);
        recorder.push(50);
    });

    let mut container = undoredo.edit(container, |recorder| {
        recorder.swap_remove(&1);
    });

    assert_eq!(container.get(&0), Some(&10));
    assert_eq!(container.get(&1), Some(&50));
    assert_eq!(container.get(&2), Some(&30));
    assert_eq!(container.get(&3), Some(&40));
    assert_eq!(container.get(&4), None);

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(&0), Some(&10));
    assert_eq!(container.get(&1), Some(&20));
    assert_eq!(container.get(&2), Some(&30));
    assert_eq!(container.get(&3), Some(&40));
    assert_eq!(container.get(&4), Some(&50));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&0), Some(&10));
    assert_eq!(container.get(&1), Some(&50));
    assert_eq!(container.get(&2), Some(&30));
    assert_eq!(container.get(&3), Some(&40));
    assert_eq!(container.get(&4), None);

    let mut container = undoredo.edit(container, |recorder| {
        recorder.swap_remove(&3);
    });

    assert_eq!(container.get(&0), Some(&10));
    assert_eq!(container.get(&1), Some(&50));
    assert_eq!(container.get(&2), Some(&30));
    assert_eq!(container.get(&3), None);

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(&0), Some(&10));
    assert_eq!(container.get(&1), Some(&50));
    assert_eq!(container.get(&2), Some(&30));
    assert_eq!(container.get(&3), Some(&40));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&0), Some(&10));
    assert_eq!(container.get(&1), Some(&50));
    assert_eq!(container.get(&2), Some(&30));
    assert_eq!(container.get(&3), None);
}

pub fn test_delta_undo_redo<
    C: Keyed<Key = usize>
        + Map<i32>
        + Get<usize>
        + Push<usize>
        + Set<usize>
        + Pop
        + Len
        + ApplyDelta<BTreeMap<usize, i32>>,
>(
    container: C,
) {
    let mut undoredo: UndoRedo<Delta<BTreeMap<usize, i32>>> = UndoRedo::new();

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

    assert_eq!(container.get(&0), Some(&0));
    assert_eq!(container.get(&1), Some(&10));
    assert_eq!(container.get(&2), Some(&20));
    assert_eq!(container.get(&3), Some(&30));
    assert_eq!(container.get(&4), Some(&40));
    assert_eq!(container.get(&5), Some(&50));
    assert_eq!(container.get(&6), None);

    let mut container = undoredo.edit(container, |recorder| {
        recorder.set(1, 11);
        recorder.set(3, 33);
        recorder.pop();
    });

    assert_eq!(container.get(&0), Some(&0));
    assert_eq!(container.get(&1), Some(&11));
    assert_eq!(container.get(&2), Some(&20));
    assert_eq!(container.get(&3), Some(&33));
    assert_eq!(container.get(&4), Some(&40));
    assert_eq!(container.get(&5), None);
    assert_eq!(container.get(&6), None);

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(&0), Some(&0));
    assert_eq!(container.get(&1), Some(&10));
    assert_eq!(container.get(&2), Some(&20));
    assert_eq!(container.get(&3), Some(&30));
    assert_eq!(container.get(&4), Some(&40));
    assert_eq!(container.get(&5), Some(&50));
    assert_eq!(container.get(&6), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&0), Some(&0));
    assert_eq!(container.get(&1), Some(&11));
    assert_eq!(container.get(&2), Some(&20));
    assert_eq!(container.get(&3), Some(&33));
    assert_eq!(container.get(&4), Some(&40));
    assert_eq!(container.get(&5), None);
    assert_eq!(container.get(&6), None);

    let mut container = undoredo.edit(container, |recorder| {
        recorder.push(50);
        recorder.push(60);
    });

    assert_eq!(container.get(&0), Some(&0));
    assert_eq!(container.get(&1), Some(&11));
    assert_eq!(container.get(&2), Some(&20));
    assert_eq!(container.get(&3), Some(&33));
    assert_eq!(container.get(&4), Some(&40));
    assert_eq!(container.get(&5), Some(&50));
    assert_eq!(container.get(&6), Some(&60));

    assert_eq!(undoredo.redo(&mut container), None);

    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&0), Some(&0));
    assert_eq!(container.get(&1), Some(&10));
    assert_eq!(container.get(&2), Some(&20));
    assert_eq!(container.get(&3), Some(&30));
    assert_eq!(container.get(&4), Some(&40));
    assert_eq!(container.get(&5), Some(&50));
    assert_eq!(container.get(&6), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&0), Some(&0));
    assert_eq!(container.get(&1), Some(&11));
    assert_eq!(container.get(&2), Some(&20));
    assert_eq!(container.get(&3), Some(&33));
    assert_eq!(container.get(&4), Some(&40));
    assert_eq!(container.get(&5), None);
    assert_eq!(container.get(&6), None);
}
