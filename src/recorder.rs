// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use crate::{
    edit::Edit,
    map::{Get, Insert, Map, Push, Remove},
};

pub struct Recorder<K, V = (), C = BTreeMap<K, V>, EC = C> {
    collection: C,
    edit: Edit<EC>,
    key_marker: PhantomData<K>,
    value_marker: PhantomData<V>,
}

impl<K, V, C, EC: Default> Recorder<K, V, C, EC> {
    #[inline(always)]
    pub fn new(container: C) -> Self {
        Self::new_with_edit(container, Default::default())
    }

    #[inline(always)]
    pub fn flush(&mut self) -> Edit<EC> {
        core::mem::replace(&mut self.edit, Edit::new())
    }
}

impl<K, V, C, EC> Recorder<K, V, C, EC> {
    #[inline(always)]
    fn new_with_edit(container: C, edit: Edit<EC>) -> Self {
        Self {
            collection: container,
            edit,
            key_marker: PhantomData,
            value_marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn collection(&self) -> &C {
        &self.collection
    }

    #[inline(always)]
    pub fn dissolve(self) -> (C, Edit<EC>) {
        (self.collection, self.edit)
    }
}

impl<K, V, C: Default, EC: Default> Default for Recorder<K, V, C, EC> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            collection: Default::default(),
            edit: Default::default(),
            key_marker: PhantomData,
            value_marker: PhantomData,
        }
    }
}

impl<K, V, C, EC> Map for Recorder<K, V, C, EC> {
    type Item = V;
}

impl<K, V, C: Get<K, Item = V>, EC> Get<K> for Recorder<K, V, C, EC> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        self.collection.get(key)
    }
}

impl<K: Clone, V: Clone, C: Get<K, Item = V> + Insert<K>, EC: Get<K, Item = V> + Insert<K>>
    Insert<K> for Recorder<K, V, C, EC>
{
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        if self.edit.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.collection.get(&key) {
                self.edit
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.edit.inserted.insert(key.clone(), value.clone());
        self.collection.insert(key, value.clone());
    }
}

impl<K: Clone, V: Clone, C: Remove<K, Item = V>, EC: Insert<K, Item = V> + Remove<K>> Remove<K>
    for Recorder<K, V, C, EC>
{
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.collection.remove(key)?;

        if self.edit.inserted.remove(key).is_none() {
            self.edit.removed.insert(key.clone(), value.clone());
        }

        Some(value)
    }
}

impl<K: Clone, V: Clone, C: Push<K, Item = V>, EC: Insert<K, Item = V>> Push<K>
    for Recorder<K, V, C, EC>
{
    #[inline(always)]
    fn push(&mut self, value: V) -> K {
        let key = self.collection.push(value.clone());
        self.edit.inserted.insert(key.clone(), value);

        key
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use alloc::collections::BTreeMap;

    use crate::{
        Edit, Recorder,
        edit::ApplyEdit,
        map::{Get, Insert, Push, Remove},
    };

    pub(crate) fn test_apply_edit_at_generated_indexes<
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

        let edit = Edit {
            removed: BTreeMap::from([(second.clone(), 20)]),
            inserted: BTreeMap::from([(third.clone(), 33), (sixth.clone(), 60)]),
        };
        recorder.apply_edit(&edit);

        assert_eq!(recorder.get(&first), Some(&10));
        assert_eq!(recorder.get(&second), None);
        assert_eq!(recorder.get(&third), Some(&33));
        assert_eq!(recorder.get(&fourth), Some(&40));
        assert_eq!(recorder.get(&fifth), Some(&50));
        assert_eq!(recorder.get(&sixth), Some(&60));
    }

    pub(crate) fn test_apply_edit_at_specified_indexes<
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

        let edit = Edit {
            removed: BTreeMap::from([(2, 20)]),
            inserted: BTreeMap::from([(3, 33), (6, 60)]),
        };
        recorder.apply_edit(&edit);

        assert_eq!(recorder.get(&1), Some(&10));
        assert_eq!(recorder.get(&2), None);
        assert_eq!(recorder.get(&3), Some(&33));
        assert_eq!(recorder.get(&4), Some(&40));
        assert_eq!(recorder.get(&5), Some(&50));
        assert_eq!(recorder.get(&6), Some(&60));
    }

    pub(crate) fn test_insert_and_remove_at_generated_indexes<
        K: Ord + Clone,
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

    pub(crate) fn test_insert_and_remove_at_specified_indexes<
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
}
