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
        Self::with_edit(container, Default::default())
    }

    #[inline(always)]
    pub fn flush(&mut self) -> Edit<EC> {
        core::mem::replace(&mut self.edit, Edit::new())
    }
}

impl<K, V, C, EC> Recorder<K, V, C, EC> {
    #[inline(always)]
    pub fn with_edit(container: C, edit: Edit<EC>) -> Self {
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

impl<
    K: Clone,
    V: Clone,
    C: Get<K, Item = V> + Insert<K> + Remove<K, Item = V>,
    EC: Get<K, Item = V> + Insert<K, Item = V> + Remove<K>,
> Recorder<K, V, C, EC>
{
    #[inline(always)]
    pub fn update<F: FnOnce(Option<V>) -> Option<V>>(&mut self, key: K, f: F) {
        if let Some(value) = f(self.remove(&key)) {
            self.insert(key, value);
        }
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
