// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thunderdome::{Arena, Index};

use crate::map::{Get, Insert, IntoIter, Keyed, Map, Push, Remove};

impl<V> Map for Arena<V> {
    type Item = V;
}

impl<V> Keyed for Arena<V> {
    type Key = Index;
}

impl<V> Get<Index> for Arena<V> {
    #[inline(always)]
    fn get(&self, key: &Index) -> Option<&V> {
        Arena::get(self, *key)
    }
}

impl<V> Insert<Index> for Arena<V> {
    #[inline(always)]
    fn insert(&mut self, key: Index, value: V) {
        Arena::insert_at(self, key, value);
    }
}

impl<V> Remove<Index> for Arena<V> {
    #[inline(always)]
    fn remove(&mut self, key: &Index) -> Option<V> {
        Arena::remove(self, *key)
    }
}

impl<V> Push<Index> for Arena<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> Index {
        Arena::insert(self, value)
    }
}

impl<V> IntoIter<Index> for Arena<V> {
    type IntoIter = thunderdome::iter::IntoIter<V>;

    #[inline(always)]
    fn into_iter(self) -> thunderdome::iter::IntoIter<V> {
        IntoIterator::into_iter(self)
    }
}
