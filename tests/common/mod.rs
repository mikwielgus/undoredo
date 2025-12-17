#![allow(dead_code)]

use std::collections::BTreeMap;

use undoredo::{ApplyEdit, Edit, Get, Insert, IntoIter, Push, Recorder, Remove, UndoRedo};

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

pub fn test_apply_edit_at_generated_indexes<
    K: Ord + Clone,
    C: Get<K, Item = i32> + Insert<K> + Remove<K> + Push<K>,
    EC: Get<K, Item = i32> + Insert<K> + Remove<K>,
>(
    mut recorder: Recorder<K, i32, C, EC>,
) {
    let first = recorder.push(10);
    let second = recorder.push(20);
    let third = recorder.push(30);
    let fourth = recorder.push(40);
    let fifth = recorder.push(50);
    let sixth = recorder.push(60);
    recorder.remove(&sixth);

    let edit = Edit::with_removed_inserted(
        BTreeMap::from([(second.clone(), 20)]),
        BTreeMap::from([(third.clone(), 33), (sixth.clone(), 60)]),
    );
    recorder.apply_edit(&edit);

    assert_eq!(recorder.get(&first), Some(&10));
    assert_eq!(recorder.get(&second), None);
    assert_eq!(recorder.get(&third), Some(&33));
    assert_eq!(recorder.get(&fourth), Some(&40));
    assert_eq!(recorder.get(&fifth), Some(&50));
    assert_eq!(recorder.get(&sixth), Some(&60));
}

pub fn test_apply_edit_at_specified_indexes<
    C: Insert<usize, Item = i32> + Remove<usize> + Get<usize>,
    EC: Get<usize, Item = i32> + Insert<usize> + Remove<usize>,
>(
    mut recorder: Recorder<usize, i32, C, EC>,
) {
    recorder.insert(1, 10);
    recorder.insert(2, 20);
    recorder.insert(3, 30);
    recorder.insert(4, 40);
    recorder.insert(5, 50);

    let edit = Edit::with_removed_inserted(
        BTreeMap::from([(2, 20)]),
        BTreeMap::from([(3, 33), (6, 60)]),
    );
    recorder.apply_edit(&edit);

    assert_eq!(recorder.get(&1), Some(&10));
    assert_eq!(recorder.get(&2), None);
    assert_eq!(recorder.get(&3), Some(&33));
    assert_eq!(recorder.get(&4), Some(&40));
    assert_eq!(recorder.get(&5), Some(&50));
    assert_eq!(recorder.get(&6), Some(&60));
}

pub fn test_apply_edit_on_set<
    K: Clone + FromUsize + Ord,
    C: Insert<K, Item = ()> + Remove<K> + Get<K>,
    EC: Get<K, Item = ()> + Insert<K> + Remove<K>,
>(
    mut recorder: Recorder<K, (), C, EC>,
) {
    recorder.insert(K::from_usize(10), ());
    recorder.insert(K::from_usize(20), ());
    recorder.insert(K::from_usize(30), ());
    recorder.insert(K::from_usize(40), ());
    recorder.insert(K::from_usize(50), ());

    let edit = Edit::with_removed_inserted(
        BTreeMap::from([(K::from_usize(20), ())]),
        BTreeMap::from([(K::from_usize(30), ()), (K::from_usize(60), ())]),
    );
    recorder.apply_edit(&edit);

    assert_eq!(recorder.get(&K::from_usize(10)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(20)), None);
    assert_eq!(recorder.get(&K::from_usize(30)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(40)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(50)), Some(&()));
    assert_eq!(recorder.get(&K::from_usize(60)), Some(&()));
}

pub fn test_insert_and_remove_at_generated_indexes<
    K: Clone,
    C: Insert<K, Item = i32> + Remove<K> + Push<K> + Get<K>,
    EC: Get<K, Item = i32> + Insert<K> + Remove<K>,
>(
    mut recorder: Recorder<K, i32, C, EC>,
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
    recorder.insert(sixth.clone(), 60);

    assert_eq!(recorder.get(&first), Some(&11));
    assert_eq!(recorder.get(&second), None);
    assert_eq!(recorder.get(&third), Some(&30));
    assert_eq!(recorder.get(&fourth), None);
    assert_eq!(recorder.get(&fifth), Some(&50));
    assert_eq!(recorder.get(&sixth), Some(&60));
}

pub fn test_insert_and_remove_at_specified_indexes<
    C: Insert<usize, Item = i32> + Remove<usize, Item = i32> + Get<usize>,
    EC: Get<usize, Item = i32> + Insert<usize> + Remove<usize>,
>(
    mut recorder: Recorder<usize, i32, C, EC>,
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
    C: Insert<K, Item = ()> + Remove<K> + Get<K>,
    EC: Get<K, Item = ()> + Insert<K> + Remove<K>,
>(
    mut recorder: Recorder<K, (), C, EC>,
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

pub fn test_edit_undo_redo_at_generated_indexes<
    K: Clone,
    C: Get<K, Item = i32> + Insert<K> + Remove<K> + Push<K> + IntoIter<K>,
    EC: Clone + Default + Get<K, Item = i32> + Insert<K> + IntoIter<K, Key = K> + Remove<K>,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<EC> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);
    assert_eq!(undoredo.redo(&mut container), None);

    let mut indexes = Vec::new();

    let mut container = undoredo.edit(container, |recorder| {
        indexes.push(recorder.push(10));
        // Repeat the same index to start indexing from 1 like in the test with specified indexes.
        indexes.push(indexes[0].clone());

        indexes.push(recorder.push(20));
        indexes.push(recorder.push(30));
        indexes.push(recorder.push(40));
        indexes.push(recorder.push(50));

        indexes.push(recorder.push(60));
        recorder.remove(&indexes[6]);
    });

    assert_eq!(undoredo.redo(&mut container), None);

    assert_eq!(container.get(&indexes[1]), Some(&10));
    assert_eq!(container.get(&indexes[2]), Some(&20));
    assert_eq!(container.get(&indexes[3]), Some(&30));
    assert_eq!(container.get(&indexes[4]), Some(&40));
    assert_eq!(container.get(&indexes[5]), Some(&50));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&indexes[2]);
        recorder.insert(indexes[1].clone(), 11);
        recorder.insert(indexes[3].clone(), 33);
    });

    assert_eq!(container.get(&indexes[1]), Some(&11));
    assert_eq!(container.get(&indexes[2]), None);
    assert_eq!(container.get(&indexes[3]), Some(&33));
    assert_eq!(container.get(&indexes[4]), Some(&40));
    assert_eq!(container.get(&indexes[5]), Some(&50));

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(&indexes[1]), Some(&10));
    assert_eq!(container.get(&indexes[2]), Some(&20));
    assert_eq!(container.get(&indexes[3]), Some(&30));
    assert_eq!(container.get(&indexes[4]), Some(&40));
    assert_eq!(container.get(&indexes[5]), Some(&50));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&indexes[1]), Some(&11));
    assert_eq!(container.get(&indexes[2]), None);
    assert_eq!(container.get(&indexes[3]), Some(&33));
    assert_eq!(container.get(&indexes[4]), Some(&40));
    assert_eq!(container.get(&indexes[5]), Some(&50));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&indexes[3]);
        recorder.insert(indexes[6].clone(), 60);
    });

    assert_eq!(container.get(&indexes[1]), Some(&11));
    assert_eq!(container.get(&indexes[2]), None);
    assert_eq!(container.get(&indexes[3]), None);
    assert_eq!(container.get(&indexes[4]), Some(&40));
    assert_eq!(container.get(&indexes[5]), Some(&50));
    assert_eq!(container.get(&indexes[6]), Some(&60));

    assert_eq!(undoredo.redo(&mut container), None);

    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&indexes[1]), Some(&10));
    assert_eq!(container.get(&indexes[2]), Some(&20));
    assert_eq!(container.get(&indexes[3]), Some(&30));
    assert_eq!(container.get(&indexes[4]), Some(&40));
    assert_eq!(container.get(&indexes[5]), Some(&50));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&indexes[1]), Some(&11));
    assert_eq!(container.get(&indexes[2]), None);
    assert_eq!(container.get(&indexes[3]), Some(&33));
    assert_eq!(container.get(&indexes[4]), Some(&40));
    assert_eq!(container.get(&indexes[5]), Some(&50));
}

pub fn test_edit_undo_redo_at_specified_indexes<
    K: Clone + FromUsize,
    C: Get<K, Item = i32> + Insert<K> + IntoIter<K, Key = K> + Remove<K>,
    EC: Clone + Default + Get<K, Item = i32> + Insert<K> + IntoIter<K, Key = K> + Remove<K>,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<EC> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);
    assert_eq!(undoredo.redo(&mut container), None);

    let mut container = undoredo.edit(container, |recorder| {
        recorder.insert(K::from_usize(1), 10);
        recorder.insert(K::from_usize(2), 20);
        recorder.insert(K::from_usize(3), 30);
        recorder.insert(K::from_usize(4), 40);
        recorder.insert(K::from_usize(5), 50);
    });

    assert_eq!(undoredo.redo(&mut container), None);

    assert_eq!(container.get(&K::from_usize(1)), Some(&10));
    assert_eq!(container.get(&K::from_usize(2)), Some(&20));
    assert_eq!(container.get(&K::from_usize(3)), Some(&30));
    assert_eq!(container.get(&K::from_usize(4)), Some(&40));
    assert_eq!(container.get(&K::from_usize(5)), Some(&50));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&K::from_usize(2));
        recorder.insert(K::from_usize(1), 11);
        recorder.insert(K::from_usize(3), 33);
    });

    assert_eq!(container.get(&K::from_usize(1)), Some(&11));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), Some(&33));
    assert_eq!(container.get(&K::from_usize(4)), Some(&40));
    assert_eq!(container.get(&K::from_usize(5)), Some(&50));

    assert!(undoredo.undo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(1)), Some(&10));
    assert_eq!(container.get(&K::from_usize(2)), Some(&20));
    assert_eq!(container.get(&K::from_usize(3)), Some(&30));
    assert_eq!(container.get(&K::from_usize(4)), Some(&40));
    assert_eq!(container.get(&K::from_usize(5)), Some(&50));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(1)), Some(&11));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), Some(&33));
    assert_eq!(container.get(&K::from_usize(4)), Some(&40));
    assert_eq!(container.get(&K::from_usize(5)), Some(&50));

    let mut container = undoredo.edit(container, |recorder| {
        recorder.remove(&K::from_usize(3));
        recorder.insert(K::from_usize(6), 60);
    });

    assert_eq!(container.get(&K::from_usize(1)), Some(&11));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), None);
    assert_eq!(container.get(&K::from_usize(4)), Some(&40));
    assert_eq!(container.get(&K::from_usize(5)), Some(&50));
    assert_eq!(container.get(&K::from_usize(6)), Some(&60));

    assert_eq!(undoredo.redo(&mut container), None);

    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(1)), Some(&10));
    assert_eq!(container.get(&K::from_usize(2)), Some(&20));
    assert_eq!(container.get(&K::from_usize(3)), Some(&30));
    assert_eq!(container.get(&K::from_usize(4)), Some(&40));
    assert_eq!(container.get(&K::from_usize(5)), Some(&50));

    assert!(undoredo.redo(&mut container).is_some());

    assert_eq!(container.get(&K::from_usize(1)), Some(&11));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), Some(&33));
    assert_eq!(container.get(&K::from_usize(4)), Some(&40));
    assert_eq!(container.get(&K::from_usize(5)), Some(&50));
}

pub fn test_edit_undo_redo_on_set<
    K: Clone + FromUsize,
    C: Get<K, Item = ()> + Insert<K> + IntoIter<K, Key = K> + Remove<K>,
    EC: Clone + Default + Get<K, Item = ()> + Insert<K> + IntoIter<K, Key = K> + Remove<K>,
>(
    mut collection: C,
) {
    let mut undoredo: UndoRedo<EC> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut collection), None);
    assert_eq!(undoredo.redo(&mut collection), None);

    let mut container = undoredo.edit(collection, |recorder| {
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

