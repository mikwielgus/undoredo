// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;
use core::marker::PhantomData;
use core::ops::Index;

use maplike::{
    Assign, Clear, Container, Get, GetByLeft, GetByRight, Insert, IntoIter, Len, Modify, Pop, Push,
    Remove, RemoveByLeft, RemoveByRight, Set,
};

use crate::{ApplyDelta, delta::Delta};

/// Records deltas applied to a container so that they can be replayed or
/// reverted.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Recorder<
    C: Container,
    DC: Container = BTreeMap<<C as Container>::Key, <C as Container>::Value>,
> {
    container: C,
    delta: Delta<DC>,
}

impl<C: Container, DC: Container> AsRef<C> for Recorder<C, DC> {
    #[inline]
    fn as_ref(&self) -> &C {
        &self.container
    }
}

impl<C: Container, DC: Container + Default> Recorder<C, DC> {
    /// Create a new recorder recording changes to an owned container.
    #[inline]
    pub fn new(container: C) -> Self {
        Self::with_delta(container, Default::default())
    }
}

impl<C: Container, DC: Container> Recorder<C, DC> {
    /// Create a new recorder recording changes to an owned container, storing
    /// them in an already existing delta.
    #[inline]
    pub fn with_delta(container: C, delta: Delta<DC>) -> Self {
        Self { container, delta }
    }

    /// Returns a reference to the recorded container.
    #[inline]
    pub fn container(&self) -> &C {
        &self.container
    }

    /// Dissolve the recorder, returning and ceding ownership of its recorded
    /// container and delta.
    #[inline]
    pub fn dissolve(self) -> (C, Delta<DC>) {
        (self.container, self.delta)
    }
}

impl<C: Container, DC: Container> Container for Recorder<C, DC> {
    type Key = C::Key;
    type Value = C::Value;
}

impl<C, DC> Assign<C> for Recorder<C, DC>
where
    C: Assign + Clone,
    DC: Get<usize, Value = C> + Set<usize>,
{
    #[inline]
    fn assign(&mut self, value: C) {
        self.assign(value);
    }
}

impl<C, DC> Recorder<C, DC>
where
    C: Assign + Clone,
    DC: Get<usize, Value = C> + Set<usize>,
{
    /// Assign a new value to `*self`.
    #[inline]
    pub fn assign(&mut self, value: C) {
        if self.delta.inserted.get(&0).is_none() {
            self.delta.removed.set(0, self.container.clone());
        }

        self.delta.inserted.set(0, value.clone());
        self.container.assign(value);
    }
}

impl<K, C, DC> Get<K> for Recorder<C, DC>
where
    C: Get<K, Key = K>,
    DC: Container,
{
    #[inline]
    fn get(&self, key: &K) -> Option<&C::Value> {
        self.get(key)
    }
}

impl<K, C, DC> Recorder<C, DC>
where
    C: Get<K, Key = K>,
    DC: Container,
{
    /// Returns a reference to the value corresponding to the key.
    #[inline]
    pub fn get(&self, key: &K) -> Option<&C::Value> {
        self.container.get(key)
    }
}

impl<K, V, C, DC> GetByLeft<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + GetByLeft<K>,
    DC: Container,
{
    #[inline]
    fn get_by_left(&self, key: &K) -> Option<&C::Value> {
        self.get_by_left(key)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + GetByLeft<K>,
    DC: Container,
{
    /// Returns a reference to the right value corresponding to the given left
    /// value in a bidirectional map.
    #[inline]
    pub fn get_by_left(&self, key: &K) -> Option<&V> {
        self.container.get_by_left(key)
    }
}

impl<K, V, C, DC> GetByRight<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + GetByRight<K>,
    DC: Container,
{
    #[inline]
    fn get_by_right(&self, key: &Self::Value) -> Option<&K> {
        self.get_by_right(key)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + GetByRight<K>,
    DC: Container,
{
    /// Returns a reference to the left value corresponding to the given right
    /// value in a bidirectional map.
    #[inline]
    pub fn get_by_right(&self, key: &V) -> Option<&K> {
        self.container.get_by_right(key)
    }
}

impl<I, C, DC> Index<I> for Recorder<C, DC>
where
    C: Container + Index<I, Output = C::Value>,
    DC: Container,
{
    type Output = C::Value;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.container[index]
    }
}

impl<K, V, C, DC> Set<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + Get<K> + Set<K>,
    DC: Container<Key = K, Value = V> + Get<K> + Insert<K>,
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
    C: Container<Key = K, Value = V> + Get<K> + Set<K>,
    DC: Container<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Set the value of an already existing element under a key.
    #[inline]
    pub fn set(&mut self, key: K, value: V) {
        if self.delta.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.container.get(&key) {
                self.delta
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.delta.inserted.insert(key.clone(), value.clone());
        self.container.set(key, value);
    }
}

impl<K, V, C, DC> Modify<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + Get<K> + Modify<K>,
    DC: Container<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn modify<F>(&mut self, key: K, f: F)
    where
        F: FnOnce(&mut Self::Value),
    {
        self.modify(key, f);
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + Get<K> + Modify<K>,
    DC: Container<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Modify the value under key with a closure.
    #[inline]
    pub fn modify<F>(&mut self, key: K, f: F)
    where
        F: FnOnce(&mut V),
    {
        if self.delta.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.container.get(&key) {
                self.delta
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.container.modify(key.clone(), f);

        if let Some(value) = self.container.get(&key) {
            self.delta.inserted.insert(key, value.clone());
        }
    }
}

impl<K, V, C, DC> Insert<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + Get<K> + Insert<K>,
    DC: Container<Key = K, Value = V> + Get<K> + Insert<K>,
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
    C: Container<Key = K, Value = V> + Get<K> + Insert<K>,
    DC: Container<Key = K, Value = V> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a key-value pair into the container.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        if self.delta.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.container.get(&key) {
                self.delta
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.delta.inserted.insert(key.clone(), value.clone());
        self.container.insert(key, value.clone());
    }
}

impl<K, V, C, DC> Remove<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + Remove<K>,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + Remove<K>,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Remove an element under a key from the container, returning the value at
    /// the key if the key was previously in the map.
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.container.remove(key)?;

        if self.delta.inserted.remove(key).is_none() {
            self.delta.removed.insert(key.clone(), value.clone());
        }

        Some(value)
    }
}

impl<K, V, C, DC> RemoveByLeft<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + RemoveByLeft<K>,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn remove_by_left(&mut self, key: &K) -> Option<V> {
        self.remove_by_left(key)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + RemoveByLeft<K>,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Remove the left and right values from pair corresponding to the given
    /// left value in a bidirectional map.
    #[inline]
    pub fn remove_by_left(&mut self, key: &K) -> Option<V> {
        let value = self.container.remove_by_left(key)?;

        if self.delta.inserted.remove(key).is_none() {
            self.delta.removed.insert(key.clone(), value.clone());
        }

        Some(value)
    }
}

impl<K, V, C, DC> RemoveByRight<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + RemoveByRight<K>,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn remove_by_right(&mut self, key: &Self::Value) -> Option<K> {
        self.remove_by_right(key)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + RemoveByRight<K>,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Remove the left and right values from pair corresponding to the given
    /// right value in a bidirectional map.
    #[inline]
    pub fn remove_by_right(&mut self, key: &V) -> Option<K> {
        let left_key = self.container.remove_by_right(key)?;

        if self.delta.inserted.remove(&left_key).is_none() {
            self.delta.removed.insert(left_key.clone(), key.clone());
        }

        Some(left_key)
    }
}

impl<K, V, C, DC> Push<K> for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + Push<K>,
    DC: Container<Key = K, Value = V> + Insert<K>,
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
    C: Container<Key = K, Value = V> + Push<K>,
    DC: Container<Key = K, Value = V> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a value into the container without specifying a key, returning
    /// the key that was automatically generated.
    #[inline]
    pub fn push(&mut self, value: V) -> K {
        let key = self.container.push(value.clone());
        self.delta.inserted.insert(key.clone(), value);

        key
    }
}

impl<K, V, C, DC> Pop for Recorder<C, DC>
where
    C: Container<Key = K, Value = V> + Len + Pop,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
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
    C: Container<Key = K, Value = V> + Len + Pop,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a value into the container without specifying a key, returning
    /// the key that was automatically generated.
    #[inline]
    pub fn pop(&mut self) -> Option<V> {
        let value = self.container.pop()?;

        if self.delta.inserted.remove(&self.container.len()).is_none() {
            self.delta
                .removed
                .insert(self.container.len(), value.clone());
        }

        Some(value)
    }
}

impl<K, V, C, DC> Clear for Recorder<C, DC>
where
    C: Clear + Clone + IntoIter<K, Value = V>,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
{
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Clear + Clone + IntoIter<K, Value = V>,
    DC: Container<Key = K, Value = V> + Insert<K> + Remove<K>,
{
    /// Remove all elements from the container.
    pub fn clear(&mut self) {
        for (key, value) in self.container.clone().into_iter() {
            self.delta.removed.insert(key, value);
        }

        self.container.clear();
    }
}

impl<C, DC> Len for Recorder<C, DC>
where
    C: Len,
    DC: Container,
{
    #[inline]
    fn len(&self) -> C::Key {
        self.len()
    }
}

impl<C, DC> Recorder<C, DC>
where
    C: Len,
    DC: Container,
{
    /// Returns the length of the container.
    #[inline]
    fn len(&self) -> C::Key {
        self.container.len()
    }
}

impl<K, C, DC> IntoIter<K> for Recorder<C, DC>
where
    C: IntoIter<K>,
    DC: Container,
{
    type IntoIter = C::IntoIter;

    #[inline]
    fn into_iter(self) -> C::IntoIter {
        self.container.into_iter()
    }
}

// This won't compile because K is unconstrained type parameter.
//
// XXX: Remove `K` parameter from `IntoIter<K>`?
/*impl<K, C, DC> Recorder<C, DC>
where
    C: IntoIter<K>,
    DC: Container,
{
    #[inline]
    pub fn into_iter(self) -> C::IntoIter {
        self.into_iter()
    }
}*/

impl<
    C: Container + ApplyDelta<DC>,
    RDC: Container,
    DC: IntoIter<C::Key> + Container<Key = C::Key, Value = C::Value>,
> ApplyDelta<DC> for Recorder<C, RDC>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        self.container.apply_delta(delta)
    }
}

// This won't compile because K is unconstrained type parameter.
//
/*impl<
    C: Container + ApplyDelta<DC>,
    RDC: Container,
    DC: Clone + IntoIter<C::Key> + Container<Key = C::Key, Value = C::Value>,
> Recorder<C, RDC>
where
    Self: Container<Key = C::Key, Value = C::Value> + Insert<C::Key> + Remove<C::Key>,
{
    #[inline]
    pub fn apply_delta(&mut self, delta: &Delta<DC>) {
        self.container.apply_delta(delta);
    }
}*/

/// Flush the recorder, returning the recorded delta and replacing it with a
/// new empty one.
pub trait FlushDelta<DC> {
    /// Flush the recorder, returning the recorded delta and replacing it with a
    /// new empty one.
    fn flush_delta(&mut self) -> Delta<DC>;
}

impl<C: Container, DC: Container + Default> FlushDelta<DC> for Recorder<C, DC> {
    #[inline]
    fn flush_delta(&mut self) -> Delta<DC> {
        self.flush_delta()
    }
}

impl<C: Container + Default + ApplyDelta<DC>, DC: Container + Default> FlushDelta<Recorder<C, DC>>
    for Recorder<C, DC>
{
    #[inline]
    fn flush_delta(&mut self) -> Delta<Recorder<C, DC>> {
        let (removed, inserted) = <Recorder<C, DC> as FlushDelta<DC>>::flush_delta(self).dissolve();

        // HACK: This is currently the only way to turn DC to C. This may be
        // improved later.
        let mut removed_container = C::default();
        removed_container.apply_delta(Delta::with_removed_inserted(DC::default(), removed));

        // HACK: This is currently the only way to turn DC to C. This may be
        // improved later.
        let mut inserted_container = C::default();
        inserted_container.apply_delta(Delta::with_removed_inserted(DC::default(), inserted));

        Delta::with_removed_inserted(
            Recorder::new(removed_container),
            Recorder::new(inserted_container),
        )
    }
}

impl<C: Container, DC: Container + Default> Recorder<C, DC> {
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

/// Reset the currently recorded delta by flushing it out of the recorder and
/// then applying its reverse.
pub trait ResetDelta<DC> {
    /// Reset the currently recorded delta by flushing it out of the recorder and
    /// then applying its reverse.
    fn reset_delta(&mut self);
}

impl<C: Container + ApplyDelta<DC>, DC: Container + Default> ResetDelta<DC> for Recorder<C, DC> {
    #[inline]
    fn reset_delta(&mut self) {
        self.reset_delta()
    }
}

impl<C: Container + ApplyDelta<DC>, DC: Container + Default> Recorder<C, DC> {
    /// Reset the currently recorded delta by flushing it out of the recorder and
    /// then applying its reverse.
    #[inline]
    pub fn reset_delta(&mut self) {
        let delta = self.flush_delta();
        self.container.apply_delta(delta.reverse());
    }
}

impl<
    C: Container + Default + ApplyDelta<DC>,
    DC: IntoIter<C::Key> + Container<Key = C::Key, Value = C::Value> + Default,
> ResetDelta<Recorder<C, DC>> for Recorder<C, DC>
{
    #[inline]
    fn reset_delta(&mut self) {
        let delta = self.flush_delta();
        self.apply_delta(delta.reverse());
    }
}

impl<V, DC: Default> ResetDelta<DC> for PhantomData<V> {
    #[inline]
    fn reset_delta(&mut self) {
        // Nothing happens, obviously.
    }
}
