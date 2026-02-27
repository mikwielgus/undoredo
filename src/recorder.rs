// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::marker::PhantomData;

use maplike::{Get, Insert, IntoIter, KeyedCollection, Len, Pop, Push, Remove, Set, StableRemove};

use crate::{ApplyEdit, edit::Edit};

/// Records edits applied to a collection so they can be replayed or reverted.
#[derive(Clone, Debug, Default)]
pub struct Recorder<C: KeyedCollection, EC: KeyedCollection = C> {
    collection: C,
    edit: Edit<EC>,
}

impl<C: KeyedCollection, EC: KeyedCollection> AsRef<C> for Recorder<C, EC> {
    #[inline]
    fn as_ref(&self) -> &C {
        &self.collection
    }
}

impl<C: KeyedCollection, EC: KeyedCollection + Default> Recorder<C, EC> {
    /// Create a new recorder recording changes to an owned collection.
    #[inline]
    pub fn new(collection: C) -> Self {
        Self::with_edit(collection, Default::default())
    }
}

impl<C: KeyedCollection, EC: KeyedCollection> Recorder<C, EC> {
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

impl<K, V, C, EC> Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K> + Remove<K>,
    EC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Remove an element, pass it through a closure, then insert it back.
    #[inline]
    pub fn update<F: FnOnce(Option<V>) -> Option<V>>(&mut self, key: K, f: F) {
        if let Some(value) = f(self.remove(&key)) {
            self.insert(key, value);
        }
    }
}

impl<C: KeyedCollection, EC: KeyedCollection> KeyedCollection for Recorder<C, EC> {
    type Key = C::Key;
    type Value = C::Value;
}

impl<K, C, EC> Get<K> for Recorder<C, EC>
where
    C: Get<K, Key = K>,
    EC: KeyedCollection,
{
    #[inline]
    fn get(&self, key: &K) -> Option<&C::Value> {
        self.get(key)
    }
}

impl<K, C, EC> Recorder<C, EC>
where
    C: Get<K, Key = K>,
    EC: KeyedCollection,
{
    /// Returns a reference to the value corresponding to the key.
    #[inline]
    pub fn get(&self, key: &K) -> Option<&C::Value> {
        self.collection.get(key)
    }
}

impl<K, V, C, EC> Set<K> for Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Set<K>,
    EC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn set(&mut self, key: K, value: Self::Value) {
        self.set(key, value);
    }
}

impl<K, V, C, EC> Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Set<K>,
    EC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Set the value of an already existing element under a key.
    #[inline]
    pub fn set(&mut self, key: K, value: V) {
        if self.edit.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.collection.get(&key) {
                self.edit
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.edit.inserted.insert(key.clone(), value.clone());
        self.collection.set(key, value);
    }
}

impl<K, V, C, EC> Insert<K> for Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    EC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn insert(&mut self, key: K, value: V) {
        self.insert(key, value)
    }
}

impl<K, V, C, EC> Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    EC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a key-value pair into the collection.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
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

impl<K, V, C, EC> Remove<K> for Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Remove<K>,
    EC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

impl<K, V, C, EC> StableRemove<K> for Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + StableRemove<K>,
    EC: KeyedCollection<Key = K, Value = V> + Insert<K> + StableRemove<K>,
    K: Clone,
    V: Clone,
{
}

impl<K, V, C, EC> Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Remove<K>,
    EC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Remove an element under a key from the collection, returning the value at
    /// the key if the key was previously in the map.
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.collection.remove(key)?;

        if self.edit.inserted.remove(key).is_none() {
            self.edit.removed.insert(key.clone(), value.clone());
        }

        Some(value)
    }
}

impl<K, V, C, EC> Push<K> for Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Push<K>,
    EC: KeyedCollection<Key = K, Value = V> + Insert<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn push(&mut self, value: V) -> K {
        self.push(value)
    }
}

impl<K, V, C, EC> Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Push<K>,
    EC: KeyedCollection<Key = K, Value = V> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    #[inline]
    pub fn push(&mut self, value: V) -> K {
        let key = self.collection.push(value.clone());
        self.edit.inserted.insert(key.clone(), value);

        key
    }
}

impl<K, V, C, EC> Pop for Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Len + Pop,
    EC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn pop(&mut self) -> Option<V> {
        self.pop()
    }
}

impl<K, V, C, EC> Recorder<C, EC>
where
    C: KeyedCollection<Key = K, Value = V> + Len + Pop,
    EC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    #[inline]
    pub fn pop(&mut self) -> Option<V> {
        let value = self.collection.pop()?;

        if self.edit.inserted.remove(&self.collection.len()).is_none() {
            self.edit
                .removed
                .insert(self.collection.len(), value.clone());
        }

        Some(value)
    }
}

impl<C, EC> Len for Recorder<C, EC>
where
    C: Len,
    EC: KeyedCollection,
{
    #[inline]
    fn len(&self) -> C::Key {
        self.len()
    }
}

impl<C, EC> Recorder<C, EC>
where
    C: Len,
    EC: KeyedCollection,
{
    /// Returns the length of the collection.
    #[inline]
    fn len(&self) -> C::Key {
        self.collection.len()
    }
}

impl<K, C, EC> IntoIter<K> for Recorder<C, EC>
where
    C: IntoIter<K>,
    EC: KeyedCollection,
{
    type IntoIter = C::IntoIter;

    #[inline]
    fn into_iter(self) -> C::IntoIter {
        self.collection.into_iter()
    }
}

// This won't compile because K is unconstrained type parameter.
//
// XXX: Remove `K` parameter from `IntoIter<K>`?
/*impl<K, C, EC> Recorder<C, EC>
where
    C: IntoIter<K>,
    EC: KeyedCollection,
{
    #[inline]
    pub fn into_iter(self) -> C::IntoIter {
        self.into_iter()
    }
}*/

impl<
    C: KeyedCollection + ApplyEdit<EC>,
    REC: KeyedCollection,
    EC: Clone + IntoIter<C::Key> + KeyedCollection<Key = C::Key, Value = C::Value>,
> ApplyEdit<EC> for Recorder<C, REC>
where
    Self: KeyedCollection<Key = C::Key, Value = C::Value> + Insert<C::Key> + Remove<C::Key>,
{
    #[inline]
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        self.collection.apply_edit(edit)
    }
}

// This won't compile because K is unconstrained type parameter.
//
/*impl<
    C: KeyedCollection + ApplyEdit<EC>,
    REC: KeyedCollection,
    EC: Clone + IntoIter<C::Key> + KeyedCollection<Key = C::Key, Value = C::Value>,
> Recorder<C, REC>
where
    Self: KeyedCollection<Key = C::Key, Value = C::Value> + Insert<C::Key> + Remove<C::Key>,
{
    #[inline]
    pub fn apply_edit(&mut self, edit: &Edit<EC>) {
        self.collection.apply_edit(edit);
    }
}*/

/// Flush the recorder, returning the recorded edit and replacing it with a
/// new empty one.
pub trait FlushEdit<EC> {
    /// Flush the recorder, returning the recorded edit and replacing it with a
    /// new empty one.
    fn flush_edit(&mut self) -> Edit<EC>;
}

impl<C: KeyedCollection, EC: KeyedCollection + Default> FlushEdit<EC> for Recorder<C, EC> {
    #[inline]
    fn flush_edit(&mut self) -> Edit<EC> {
        self.flush_edit()
    }
}

impl<C, EC> FlushEdit<Recorder<C, EC>> for Recorder<C, EC>
where
    C: KeyedCollection + Default + ApplyEdit<EC>,
    EC: KeyedCollection + Default,
{
    #[inline]
    fn flush_edit(&mut self) -> Edit<Recorder<C, EC>> {
        let (removed, inserted) = <Recorder<C, EC> as FlushEdit<EC>>::flush_edit(self).dissolve();

        // HACK: This is currently the only way to turn EC to C. This may be
        // improved later.
        let mut removed_collection = C::default();
        removed_collection.apply_edit(&Edit::with_removed_inserted(EC::default(), removed));

        // HACK: This is currently the only way to turn EC to C. This may be
        // improved later.
        let mut inserted_collection = C::default();
        inserted_collection.apply_edit(&Edit::with_removed_inserted(EC::default(), inserted));

        Edit::with_removed_inserted(
            Recorder::new(removed_collection),
            Recorder::new(inserted_collection),
        )
    }
}

impl<C: KeyedCollection, EC: KeyedCollection + Default> Recorder<C, EC> {
    /// Flush the recorder, returning the recorded edit and replacing it with a
    /// new empty one.
    #[inline]
    pub fn flush_edit(&mut self) -> Edit<EC> {
        core::mem::replace(&mut self.edit, Edit::new())
    }
}

impl<V, EC: Default> FlushEdit<EC> for PhantomData<V> {
    #[inline]
    fn flush_edit(&mut self) -> Edit<EC> {
        // Nothing happens, obviously.
        Edit::default()
    }
}
