use std::{collections::HashMap, hash::Hash};

use crate::collection::{Collection, Get, Insert, IntoIter, Keyed, Remove};

impl<K, V> Collection for HashMap<K, V> {
    type Item = V;
}

impl<K, V> Keyed for HashMap<K, V> {
    type Key = K;
}

impl<K: Eq + Hash, V> Get<K> for HashMap<K, V> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }
}

impl<K: Eq + Hash, V> Insert<K> for HashMap<K, V> {
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        HashMap::insert(self, key, value);
    }
}

impl<K: Eq + Hash, V> Remove<K> for HashMap<K, V> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        HashMap::remove(self, key)
    }
}

impl<K, V> IntoIter<K> for HashMap<K, V> {
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> std::collections::hash_map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::Recorder;

    #[test]
    fn test_apply_edit_at_specified_indexes() {
        let recorder = Recorder::<usize, i32>::new(HashMap::new());
        crate::recorder::tests::test_apply_edit_at_specified_indexes(recorder);
    }

    #[test]
    fn test_insert_and_remove_at_specified_indexes() {
        let recorder = Recorder::<usize, i32>::new(HashMap::new());
        crate::recorder::tests::test_insert_and_remove_at_specified_indexes(recorder);
    }

    #[test]
    fn test_edit_undo_redo_at_specified_indexes() {
        crate::undoredo::tests::test_edit_undo_redo_at_specified_indexes::<
            usize,
            HashMap<usize, i32>,
            HashMap<usize, i32>,
        >(HashMap::new());
    }
}
