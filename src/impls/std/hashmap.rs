// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{collections::HashMap, hash::Hash};

use crate::map::{Get, Insert, IntoIter, Keyed, Map, Remove};

impl<K, V> Map for HashMap<K, V> {
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
