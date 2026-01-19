// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use maplike::{Get, Insert, Keyed, Map, Push, Remove, StableRemove};

use crate::edit::Edit;

/// Records edits applied to a collection so they can be replayed or reverted.
pub struct Recorder<C: Keyed + Map, EC: Keyed + Map = C> {
    collection: C,
    edit: Edit<EC>,
}

impl<C: Keyed + Map, EC: Keyed + Map + Default> Recorder<C, EC> {
    /// Create a new recorder recording changes to an owned collection.
    #[inline]
    pub fn new(collection: C) -> Self {
        Self::with_edit(collection, Default::default())
    }

    /// Flush the recorder, returning the recorded edit and replacing it with a
    /// new empty one.
    #[inline]
    pub fn flush(&mut self) -> Edit<EC> {
        core::mem::replace(&mut self.edit, Edit::new())
    }
}

impl<C: Keyed + Map, EC: Keyed + Map> Recorder<C, EC> {
    /// Create a new recorder recording changes to an owned collection, storing
    /// them in an already existing edit.
    #[inline]
    pub fn with_edit(collection: C, edit: Edit<EC>) -> Self {
        Self { collection, edit }
    }

    /// Returns a reference to the recorded collection.
    #[inline]
    pub fn collection(&self) -> &C {
        &self.collection
    }

    /// Dissolve the recorder, returning and ceding ownership of its recorded
    /// collection and edit.
    #[inline]
    pub fn dissolve(self) -> (C, Edit<EC>) {
        (self.collection, self.edit)
    }
}

impl<
    C: Keyed + Map + Get<C::Key> + Insert<C::Key> + StableRemove<C::Key>,
    EC: Keyed<Key = C::Key> + Map<Item = C::Item> + Get<C::Key> + Insert<C::Key> + StableRemove<C::Key>,
> Recorder<C, EC>
where
    C::Key: Clone,
    C::Item: Clone,
{
    /// Remove an element, pass it through a closure, then insert it back.
    #[inline]
    pub fn update<F: FnOnce(Option<C::Item>) -> Option<C::Item>>(&mut self, key: C::Key, f: F) {
        if let Some(value) = f(self.remove(&key)) {
            self.insert(key, value);
        }
    }
}

impl<C: Keyed + Map + Default, EC: Keyed + Map + Default> Default for Recorder<C, EC> {
    #[inline]
    fn default() -> Self {
        Self {
            collection: Default::default(),
            edit: Default::default(),
        }
    }
}

impl<C: Keyed + Map, EC: Keyed + Map> Map for Recorder<C, EC> {
    type Item = C::Item;
}

impl<C: Keyed + Map, EC: Keyed + Map> Keyed for Recorder<C, EC> {
    type Key = C::Key;
}

impl<C: Keyed + Map + Get<C::Key>, EC: Keyed + Map> Get<C::Key> for Recorder<C, EC> {
    #[inline]
    fn get(&self, key: &C::Key) -> Option<&C::Item> {
        self.get(key)
    }
}

impl<C: Keyed + Map + Get<C::Key>, EC: Keyed + Map> Recorder<C, EC> {
    /// Returns a reference to the value corresponding to the key.
    #[inline]
    pub fn get(&self, key: &C::Key) -> Option<&C::Item> {
        self.collection.get(key)
    }
}

impl<
    C: Keyed + Map + Get<C::Key> + Insert<C::Key>,
    EC: Keyed<Key = C::Key> + Map<Item = C::Item> + Get<C::Key> + Insert<C::Key>,
> Insert<C::Key> for Recorder<C, EC>
where
    C::Key: Clone,
    C::Item: Clone,
{
    #[inline]
    fn insert(&mut self, key: C::Key, value: C::Item) {
        self.insert(key, value)
    }
}

impl<
    C: Keyed + Map + Get<C::Key> + Insert<C::Key>,
    EC: Keyed<Key = C::Key> + Map<Item = C::Item> + Get<C::Key> + Insert<C::Key>,
> Recorder<C, EC>
where
    C::Key: Clone,
    C::Item: Clone,
{
    /// Insert a key-value pair into the collection.
    #[inline]
    pub fn insert(&mut self, key: C::Key, value: C::Item) {
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

impl<
    C: Keyed + Map + StableRemove<C::Key>,
    EC: Keyed<Key = C::Key> + Map<Item = C::Item> + Insert<C::Key> + StableRemove<C::Key>,
> Remove<C::Key> for Recorder<C, EC>
where
    C::Key: Clone,
    C::Item: Clone,
{
    #[inline]
    fn remove(&mut self, key: &C::Key) -> Option<C::Item> {
        self.remove(key)
    }
}

impl<
    C: Keyed + Map + StableRemove<C::Key>,
    EC: Keyed<Key = C::Key> + Map<Item = C::Item> + Insert<C::Key> + StableRemove<C::Key>,
> StableRemove<C::Key> for Recorder<C, EC>
where
    C::Key: Clone,
    C::Item: Clone,
{
}

impl<
    C: Keyed + Map + StableRemove<C::Key>,
    EC: Keyed<Key = C::Key> + Map<Item = C::Item> + Insert<C::Key> + StableRemove<C::Key>,
> Recorder<C, EC>
where
    C::Key: Clone,
    C::Item: Clone,
{
    /// Remove an element under a key from the collection, returning the value at
    /// the key if the key was previously in the map.
    #[inline]
    pub fn remove(&mut self, key: &C::Key) -> Option<C::Item> {
        let value = self.collection.remove(key)?;

        if self.edit.inserted.remove(key).is_none() {
            self.edit.removed.insert(key.clone(), value.clone());
        }

        Some(value)
    }
}

impl<C: Keyed + Map + Push<C::Key>, EC: Keyed<Key = C::Key> + Map<Item = C::Item> + Insert<C::Key>>
    Push<C::Key> for Recorder<C, EC>
where
    C::Key: Clone,
    C::Item: Clone,
{
    #[inline]
    fn push(&mut self, value: C::Item) -> C::Key {
        self.push(value)
    }
}

impl<C: Keyed + Map + Push<C::Key>, EC: Keyed<Key = C::Key> + Map<Item = C::Item> + Insert<C::Key>>
    Recorder<C, EC>
where
    C::Key: Clone,
    C::Item: Clone,
{
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    #[inline]
    pub fn push(&mut self, value: C::Item) -> C::Key {
        let key = self.collection.push(value.clone());
        self.edit.inserted.insert(key.clone(), value);

        key
    }
}
