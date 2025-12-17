// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeSet;

use crate::maplike::{Get, Insert, IntoIter, Keyed, Map, Remove};

impl<K> Map for BTreeSet<K> {
    type Item = ();
}

impl<K> Keyed for BTreeSet<K> {
    type Key = K;
}

impl<K: Ord> Get<K> for BTreeSet<K> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        BTreeSet::get(self, key).map(|_| &())
    }
}

impl<K: Ord> Insert<K> for BTreeSet<K> {
    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) {
        BTreeSet::insert(self, key);
    }
}

impl<K: Ord> Remove<K> for BTreeSet<K> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<()> {
        BTreeSet::remove(self, key).then_some(())
    }
}

pub struct MapIntoIter<K>(alloc::collections::btree_set::IntoIter<K>);

impl<K> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K> IntoIter<K> for BTreeSet<K> {
    type IntoIter = MapIntoIter<K>;

    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
