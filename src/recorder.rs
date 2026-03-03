// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use maplike::{
    Clear, Get, Insert, IntoIter, KeyedCollection, Len, Pop, Push, Remove, Set, StableRemove,
};

use crate::{ApplyDelta, delta::Delta};

/// Records deltas applied to a collection so that they can be replayed or
/// reverted.
#[derive(Clone, Debug, Default)]
pub struct Recorder<
    C: KeyedCollection,
    DC: KeyedCollection = BTreeMap<<C as KeyedCollection>::Key, <C as KeyedCollection>::Value>,
> {
    collection: C,
    delta: Delta<DC>,
}

/// Access the type of the delta collection carried by a recorder.
pub trait RecorderDeltaCollection {
    /// The collection type used to store recorder deltas.
    type DeltaCollection: KeyedCollection;
}

impl<C: KeyedCollection, DC: KeyedCollection> RecorderDeltaCollection for Recorder<C, DC> {
    type DeltaCollection = DC;
}

impl<C: KeyedCollection, DC: KeyedCollection> AsRef<C> for Recorder<C, DC> {
    #[inline]
    fn as_ref(&self) -> &C {
        &self.collection
    }
}

impl<C: KeyedCollection, DC: KeyedCollection + Default> Recorder<C, DC> {
    /// Create a new recorder recording changes to an owned collection.
    #[inline]
    pub fn new(collection: C) -> Self {
        Self::with_delta(collection, Default::default())
    }
}

impl<C: KeyedCollection, DC: KeyedCollection> Recorder<C, DC> {
    /// Create a new recorder recording changes to an owned collection, storing
    /// them in an already existing delta.
    #[inline]
    pub fn with_delta(collection: C, delta: Delta<DC>) -> Self {
        Self { collection, delta }
    }

    /// Returns a reference to the recorded collection.
    #[inline]
    pub fn collection(&self) -> &C {
        &self.collection
    }

    /// Dissolve the recorder, returning and ceding ownership of its recorded
    /// collection and delta.
    #[inline]
    pub fn dissolve(self) -> (C, Delta<DC>) {
        (self.collection, self.delta)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K> + Remove<K>,
    DC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K> + Remove<K>,
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

impl<C: KeyedCollection, DC: KeyedCollection> KeyedCollection for Recorder<C, DC> {
    type Key = C::Key;
    type Value = C::Value;
}

impl<K, C, DC> Get<K> for Recorder<C, DC>
where
    C: Get<K, Key = K>,
    DC: KeyedCollection,
{
    #[inline]
    fn get(&self, key: &K) -> Option<&C::Value> {
        self.get(key)
    }
}

impl<K, C, DC> Recorder<C, DC>
where
    C: Get<K, Key = K>,
    DC: KeyedCollection,
{
    /// Returns a reference to the value corresponding to the key.
    #[inline]
    pub fn get(&self, key: &K) -> Option<&C::Value> {
        self.collection.get(key)
    }
}

impl<K, V, C, DC> Set<K> for Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Set<K>,
    DC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn set(&mut self, key: K, value: Self::Value) {
        self.set(key, value);
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Set<K>,
    DC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Set the value of an already existing element under a key.
    #[inline]
    pub fn set(&mut self, key: K, value: V) {
        if self.delta.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.collection.get(&key) {
                self.delta
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.delta.inserted.insert(key.clone(), value.clone());
        self.collection.set(key, value);
    }
}

impl<K, V, C, DC> Insert<K> for Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    DC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn insert(&mut self, key: K, value: V) {
        self.insert(key, value)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    DC: KeyedCollection<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a key-value pair into the collection.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        if self.delta.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.collection.get(&key) {
                self.delta
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.delta.inserted.insert(key.clone(), value.clone());
        self.collection.insert(key, value.clone());
    }
}

impl<K, V, C, DC> Remove<K> for Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Remove<K>,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

impl<K, V, C, DC> StableRemove<K> for Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + StableRemove<K>,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K> + StableRemove<K>,
    K: Clone,
    V: Clone,
{
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Remove<K>,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Remove an element under a key from the collection, returning the value at
    /// the key if the key was previously in the map.
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.collection.remove(key)?;

        if self.delta.inserted.remove(key).is_none() {
            self.delta.removed.insert(key.clone(), value.clone());
        }

        Some(value)
    }
}

impl<K, V, C, DC> Push<K> for Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Push<K>,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn push(&mut self, value: V) -> K {
        self.push(value)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Push<K>,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    #[inline]
    pub fn push(&mut self, value: V) -> K {
        let key = self.collection.push(value.clone());
        self.delta.inserted.insert(key.clone(), value);

        key
    }
}

impl<K, V, C, DC> Pop for Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Len + Pop,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn pop(&mut self) -> Option<V> {
        self.pop()
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: KeyedCollection<Key = K, Value = V> + Len + Pop,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    #[inline]
    pub fn pop(&mut self) -> Option<V> {
        let value = self.collection.pop()?;

        if self.delta.inserted.remove(&self.collection.len()).is_none() {
            self.delta
                .removed
                .insert(self.collection.len(), value.clone());
        }

        Some(value)
    }
}

impl<K, V, C, DC> Clear for Recorder<C, DC>
where
    C: Clear + Clone + IntoIter<K, Value = V>,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
{
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Clear + Clone + IntoIter<K, Value = V>,
    DC: KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
{
    /// Remove all elements from the collection.
    pub fn clear(&mut self) {
        for (key, value) in self.collection.clone().into_iter() {
            self.delta.removed.insert(key, value);
        }

        self.collection.clear();
    }
}

impl<C, DC> Len for Recorder<C, DC>
where
    C: Len,
    DC: KeyedCollection,
{
    #[inline]
    fn len(&self) -> C::Key {
        self.len()
    }
}

impl<C, DC> Recorder<C, DC>
where
    C: Len,
    DC: KeyedCollection,
{
    /// Returns the length of the collection.
    #[inline]
    fn len(&self) -> C::Key {
        self.collection.len()
    }
}

impl<K, C, DC> IntoIter<K> for Recorder<C, DC>
where
    C: IntoIter<K>,
    DC: KeyedCollection,
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
/*impl<K, C, DC> Recorder<C, DC>
where
    C: IntoIter<K>,
    DC: KeyedCollection,
{
    #[inline]
    pub fn into_iter(self) -> C::IntoIter {
        self.into_iter()
    }
}*/

impl<
    C: KeyedCollection + ApplyDelta<DC>,
    RDC: KeyedCollection,
    DC: Clone + IntoIter<C::Key> + KeyedCollection<Key = C::Key, Value = C::Value>,
> ApplyDelta<DC> for Recorder<C, RDC>
where
    Self: KeyedCollection<Key = C::Key, Value = C::Value> + Insert<C::Key> + Remove<C::Key>,
{
    #[inline]
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        self.collection.apply_delta(delta)
    }
}

// This won't compile because K is unconstrained type parameter.
//
/*impl<
    C: KeyedCollection + ApplyDelta<DC>,
    RDC: KeyedCollection,
    DC: Clone + IntoIter<C::Key> + KeyedCollection<Key = C::Key, Value = C::Value>,
> Recorder<C, RDC>
where
    Self: KeyedCollection<Key = C::Key, Value = C::Value> + Insert<C::Key> + Remove<C::Key>,
{
    #[inline]
    pub fn apply_delta(&mut self, delta: &Delta<DC>) {
        self.collection.apply_delta(delta);
    }
}*/

/// Flush the recorder, returning the recorded delta and replacing it with a
/// new empty one.
pub trait FlushDelta<DC> {
    /// Flush the recorder, returning the recorded delta and replacing it with a
    /// new empty one.
    fn flush_delta(&mut self) -> Delta<DC>;
}

impl<C: KeyedCollection, DC: KeyedCollection + Default> FlushDelta<DC> for Recorder<C, DC> {
    #[inline]
    fn flush_delta(&mut self) -> Delta<DC> {
        self.flush_delta()
    }
}

impl<C, DC> FlushDelta<Recorder<C, DC>> for Recorder<C, DC>
where
    C: KeyedCollection + Default + ApplyDelta<DC>,
    DC: KeyedCollection + Default,
{
    #[inline]
    fn flush_delta(&mut self) -> Delta<Recorder<C, DC>> {
        let (removed, inserted) = <Recorder<C, DC> as FlushDelta<DC>>::flush_delta(self).dissolve();

        // HACK: This is currently the only way to turn DC to C. This may be
        // improved later.
        let mut removed_collection = C::default();
        removed_collection.apply_delta(&Delta::with_removed_inserted(DC::default(), removed));

        // HACK: This is currently the only way to turn DC to C. This may be
        // improved later.
        let mut inserted_collection = C::default();
        inserted_collection.apply_delta(&Delta::with_removed_inserted(DC::default(), inserted));

        Delta::with_removed_inserted(
            Recorder::new(removed_collection),
            Recorder::new(inserted_collection),
        )
    }
}

impl<C: KeyedCollection, DC: KeyedCollection + Default> Recorder<C, DC> {
    /// Flush the recorder, returning the recorded delta and replacing it with a
    /// new empty one.
    #[inline]
    pub fn flush_delta(&mut self) -> Delta<DC> {
        core::mem::replace(&mut self.delta, Delta::new())
    }
}

impl<V, DC: Default> FlushDelta<DC> for PhantomData<V> {
    #[inline]
    fn flush_delta(&mut self) -> Delta<DC> {
        // Nothing happens, obviously.
        Delta::default()
    }
}
