// SPDX-FileCopyrightText: 2025 undoredo Developers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;

use crate::map::{Keyed, Map, MapGet, MapInsert, MapIntoIter, MapRemove};

impl<K, V> Map for BTreeMap<K, V> {
    type Item = V;
}

impl<K, V> Keyed for BTreeMap<K, V> {
    type Key = K;
}

impl<K: Ord, V> MapGet<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        BTreeMap::get(self, key)
    }
}

impl<K: Ord, V> MapInsert<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, value);
    }
}

impl<K: Ord, V> MapRemove<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        BTreeMap::remove(self, key)
    }
}

impl<K, V> MapIntoIter<K> for BTreeMap<K, V> {
    type IntoIter = alloc::collections::btree_map::IntoIter<K, V>;

    fn into_iter(self) -> alloc::collections::btree_map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::Recorder;

    #[test]
    fn test_apply_edit_at_specified_indexes() {
        let recorder = Recorder::<usize, i32, BTreeMap<usize, i32>, BTreeMap<usize, i32>>::new(
            BTreeMap::new(),
        );
        crate::recorder::tests::test_apply_edit_at_specified_indexes(recorder);
    }

    #[test]
    fn test_insert_and_remove_at_specified_indexes() {
        let recorder = Recorder::<usize, i32, BTreeMap<usize, i32>, BTreeMap<usize, i32>>::new(
            BTreeMap::new(),
        );
        crate::recorder::tests::test_insert_and_remove_at_specified_indexes(recorder);
    }

    #[test]
    fn test_edit_undo_redo_at_specified_indexes() {
        crate::undoredo::tests::test_edit_undo_redo_at_specified_indexes::<
            usize,
            BTreeMap<usize, i32>,
            BTreeMap<usize, i32>,
        >(BTreeMap::new());
    }
}
