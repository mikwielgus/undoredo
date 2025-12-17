// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rstar::{RTree, RTreeObject};

use crate::map::{Get, Insert, IntoIter, Keyed, Map, Remove};

impl<K: RTreeObject> Map for RTree<K> {
    type Item = ();
}

impl<K: RTreeObject> Keyed for RTree<K> {
    type Key = K;
}

impl<K: RTreeObject + PartialEq> Get<K> for RTree<K> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        RTree::contains(self, key).then_some(&())
    }
}

impl<K: RTreeObject> Insert<K> for RTree<K> {
    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) {
        RTree::insert(self, key);
    }
}

impl<K: RTreeObject + PartialEq> Remove<K> for RTree<K> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<()> {
        RTree::remove(self, key).map(|_| ())
    }
}

pub struct MapIntoIter<K: RTreeObject>(rstar::iterators::IntoIter<K>);

impl<K: RTreeObject> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K: RTreeObject> IntoIter<K> for RTree<K> {
    type IntoIter = MapIntoIter<K>;

    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
