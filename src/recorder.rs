// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;
use core::borrow::Borrow;
use core::marker::PhantomData;
use core::ops::Index;

use maplike::abc::{Container, Keyed};
use maplike::iter::{IntoIter, IntoValues, Iter, Values};
use maplike::ops::{
    Assign, Clear, Get, GetByLeft, GetByRight, Insert, Len, Modify, Pop, Push, Remove,
    RemoveByLeft, RemoveByRight, Set, SwapRemove,
};

use crate::{ApplyDelta, MergeDeltas, delta::Delta};

/// Records deltas applied to a container so that they can be replayed or
/// reverted.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Recorder<C: Keyed, DC: Keyed = BTreeMap<<C as Keyed>::Key, <C as Container>::Value>> {
    container: C,
    delta: Delta<DC>,
}

impl<C: Keyed, DC: Keyed> AsRef<C> for Recorder<C, DC> {
    #[inline]
    fn as_ref(&self) -> &C {
        &self.container
    }
}

impl<C: Keyed, DC: Keyed + Default> Recorder<C, DC> {
    /// Create a new recorder recording changes to an owned container.
    #[inline]
    pub fn new(container: C) -> Self {
        Self::with_delta(container, Default::default())
    }
}

impl<C: Keyed, DC: Keyed> Recorder<C, DC> {
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

impl<C: Keyed, DC: Keyed> Container for Recorder<C, DC> {
    type Value = C::Value;
}

impl<C: Keyed, DC: Keyed> Keyed for Recorder<C, DC> {
    type Key = C::Key;
}

impl<C, DC> Assign<C> for Recorder<C, DC>
where
    C: Assign + Clone + Keyed,
    DC: Get<usize, Value = C> + Set<usize> + Keyed,
{
    #[inline]
    fn assign(&mut self, value: C) {
        self.assign(value);
    }
}

impl<C, DC> Recorder<C, DC>
where
    C: Assign + Clone + Keyed,
    DC: Get<usize, Value = C> + Set<usize> + Keyed,
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

impl<Q: ?Sized, C, DC> Get<Q> for Recorder<C, DC>
where
    C: Get<Q> + Keyed,
    DC: Keyed,
{
    #[inline]
    fn get(&self, key: &Q) -> Option<&C::Value> {
        self.container.get(key)
    }
}

impl<K, C, DC> Recorder<C, DC>
where
    C: Keyed<Key = K> + Get<K>,
    DC: Keyed,
{
    /// Returns a reference to the value corresponding to the key.
    #[inline]
    pub fn get(&self, key: &K) -> Option<&C::Value> {
        self.container.get(key)
    }
}

impl<Q: ?Sized, K, V, C, DC> GetByLeft<Q> for Recorder<C, DC>
where
    K: Borrow<Q>,
    C: Keyed<Value = V, Key = K> + GetByLeft<Q>,
    DC: Keyed,
{
    #[inline]
    fn get_by_left(&self, key: &Q) -> Option<&C::Value> {
        self.container.get_by_left(key)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + GetByLeft<K>,
    DC: Keyed,
{
    /// Returns a reference to the right value corresponding to the given left
    /// value in a bidirectional map.
    #[inline]
    pub fn get_by_left(&self, key: &K) -> Option<&V> {
        self.container.get_by_left(key)
    }
}

impl<K, V, Q: ?Sized, C, DC> GetByRight<K, Q> for Recorder<C, DC>
where
    V: Borrow<Q>,
    C: Keyed<Value = V, Key = K> + GetByRight<K, Q>,
    DC: Keyed,
{
    #[inline]
    fn get_by_right(&self, key: &Q) -> Option<&K> {
        self.container.get_by_right(key)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + GetByRight<K>,
    DC: Keyed,
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
    C: Keyed + Index<I, Output = C::Value>,
    DC: Keyed,
{
    type Output = C::Value;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.container[index]
    }
}

impl<K, V, C, DC> Set<K> for Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + Get<K> + Set<K>,
    DC: Keyed<Value = V, Key = K> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    type Output = <C as Set<K>>::Output;

    #[inline]
    fn set(&mut self, key: K, value: Self::Value) -> Self::Output {
        self.set(key, value)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + Get<K> + Set<K>,
    DC: Keyed<Value = V, Key = K> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Set the value of an already existing element under a key.
    #[inline]
    pub fn set(&mut self, key: K, value: V) -> <C as Set<K>>::Output {
        if self.delta.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.container.get(&key) {
                self.delta
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.delta.inserted.insert(key.clone(), value.clone());
        self.container.set(key, value)
    }
}

impl<K, V, C, DC> Modify<K> for Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + Get<K> + Modify<K>,
    DC: Keyed<Value = V, Key = K> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    #[inline]
    fn modify<F>(&mut self, key: &K, f: F)
    where
        F: FnMut(&mut Self::Value),
    {
        self.modify(key.clone(), f);
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + Get<K> + Modify<K>,
    DC: Keyed<Value = V, Key = K> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Modify the value under key with a closure.
    #[inline]
    pub fn modify<F>(&mut self, key: K, f: F)
    where
        F: FnMut(&mut V),
    {
        if self.delta.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.container.get(&key) {
                self.delta
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.container.modify(&key, f);

        if let Some(value) = self.container.get(&key) {
            self.delta.inserted.insert(key, value.clone());
        }
    }
}

impl<K, V, C, DC> Insert<K> for Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + Get<K> + Insert<K>,
    DC: Keyed<Value = V, Key = K> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    type Output = <C as Insert<K>>::Output;

    #[inline]
    fn insert(&mut self, key: K, value: V) -> Self::Output {
        self.insert(key, value)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + Get<K> + Insert<K>,
    DC: Keyed<Value = V, Key = K> + Get<K> + Insert<K>,
    K: Clone,
    V: Clone,
{
    /// Insert a key-value pair into the container.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> <C as Insert<K>>::Output {
        if self.delta.inserted.get(&key).is_none() {
            if let Some(value_to_remove) = self.container.get(&key) {
                self.delta
                    .removed
                    .insert(key.clone(), value_to_remove.clone());
            }
        }

        self.delta.inserted.insert(key.clone(), value.clone());
        self.container.insert(key, value)
    }
}

impl<K, V, C, DC> Remove<K> for Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + Remove<K, Output = Option<V>>,
    DC: Keyed<Value = V, Key = K> + Insert<K> + Remove<K, Output = Option<V>>,
    K: Clone,
    V: Clone,
{
    type Output = Option<V>;

    #[inline]
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Keyed<Value = V, Key = K> + Remove<K, Output = Option<V>>,
    DC: Keyed<Value = V, Key = K> + Insert<K> + Remove<K, Output = Option<V>>,
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
    C: Keyed<Value = V, Key = K> + RemoveByLeft<K>,
    DC: Keyed<Value = V, Key = K> + Insert<K> + Remove<K, Output = Option<V>>,
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
    C: Keyed<Value = V, Key = K> + RemoveByLeft<K>,
    DC: Keyed<Value = V, Key = K> + Insert<K> + Remove<K, Output = Option<V>>,
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
    C: Keyed<Value = V, Key = K> + RemoveByRight<K>,
    DC: Keyed<Value = V, Key = K> + Insert<K> + Remove<K, Output = Option<V>>,
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
    C: Keyed<Value = V, Key = K> + RemoveByRight<K>,
    DC: Keyed<Value = V, Key = K> + Insert<K> + Remove<K, Output = Option<V>>,
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
    C: Keyed<Value = V, Key = K> + Push<K>,
    DC: Keyed<Value = V, Key = K> + Insert<K>,
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
    C: Keyed<Value = V, Key = K> + Push<K>,
    DC: Keyed<Value = V, Key = K> + Insert<K>,
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

impl<V, C, DC> Pop for Recorder<C, DC>
where
    C: Keyed<Value = V, Key = usize> + Len + Pop,
    DC: Keyed<Value = V, Key = usize> + Insert<usize> + Remove<usize, Output = Option<V>>,
    V: Clone,
{
    #[inline]
    fn pop(&mut self) -> Option<V> {
        self.pop()
    }
}

impl<V, C, DC> Recorder<C, DC>
where
    C: Keyed<Value = V, Key = usize> + Len + Pop,
    DC: Keyed<Value = V, Key = usize> + Insert<usize> + Remove<usize, Output = Option<V>>,
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

impl<V, C, DC> SwapRemove<usize> for Recorder<C, DC>
where
    C: Keyed<Value = V, Key = usize> + Get<usize> + Len + SwapRemove<usize, Output = V>,
    DC: Keyed<Value = V, Key = usize>
        + Get<usize>
        + Insert<usize>
        + Remove<usize, Output = Option<V>>,
    V: Clone,
{
    type Output = V;

    #[inline]
    fn swap_remove(&mut self, key: &usize) -> V {
        self.swap_remove(key)
    }
}

impl<V, C, DC> Recorder<C, DC>
where
    C: Keyed<Value = V, Key = usize> + Get<usize> + Len + SwapRemove<usize, Output = V>,
    DC: Keyed<Value = V, Key = usize>
        + Get<usize>
        + Insert<usize>
        + Remove<usize, Output = Option<V>>,
    V: Clone,
{
    /// Remove an element under a key by swapping it with the last element,
    /// returning the removed value.
    #[inline]
    pub fn swap_remove(&mut self, key: &usize) -> V {
        let last = self
            .container
            .len()
            .checked_sub(1)
            .expect("swap_remove index is out of bounds");

        if *key == last {
            // Since the key is the last element, there is no need to perform a
            // swap, we only need to perform a plain remove.

            let value = self.container.swap_remove(key);

            // Same delta update logic as if it was plain `.remove(key)`.
            if self.delta.inserted.remove(key).is_none() {
                self.delta.removed.insert(*key, value.clone());
            }

            value
        } else {
            let last_value = self.container.get(&last).unwrap().clone();
            let value = self.container.swap_remove(key);

            // This delta update is like that of `.insert(key, ...)` or
            // `.modify(key, ...)`, but we use `.swap_remove()`'s return value
            // instead of a `.get()` call.

            if self.delta.inserted.get(key).is_none() {
                self.delta.removed.insert(*key, value.clone());
            }

            self.delta.inserted.insert(*key, last_value.clone());

            // Now same delta update as if it was plain `.remove(last)`.
            if self.delta.inserted.remove(&last).is_none() {
                self.delta.removed.insert(last, last_value);
            }

            value
        }
    }
}

impl<K, V, C, DC> Clear for Recorder<C, DC>
where
    C: Clear + Clone + IntoIter<K, Value = V> + Keyed,
    DC: Keyed<Value = V, Key = K> + Insert<K> + Remove<K, Output = Option<V>>,
{
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<K, V, C, DC> Recorder<C, DC>
where
    C: Clear + Clone + IntoIter<K, Value = V> + Keyed,
    DC: Keyed<Value = V, Key = K> + Insert<K> + Remove<K, Output = Option<V>>,
{
    /// Remove all elements from the container.
    pub fn clear(&mut self) {
        for (key, value) in self.container.clone().into_iter() {
            // The same delta update as if it was plain `.remove(key)`.
            if self.delta.inserted.remove(&key).is_none() {
                self.delta.removed.insert(key, value);
            }
        }

        self.container.clear();
    }
}

impl<C, DC> Len for Recorder<C, DC>
where
    C: Len + Keyed,
    DC: Keyed,
{
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<C, DC> Recorder<C, DC>
where
    C: Len + Keyed,
    DC: Keyed,
{
    /// Returns the length of the container.
    #[inline]
    fn len(&self) -> usize {
        self.container.len()
    }
}

impl<'a, C, DC> Values<'a> for Recorder<C, DC>
where
    C: Values<'a> + Keyed,
    DC: Keyed,
    Self: 'a,
{
    type Values = C::Values;

    #[inline]
    fn values(&'a self) -> C::Values {
        self.container.values()
    }
}

impl<C, DC> IntoValues for Recorder<C, DC>
where
    C: IntoValues + Keyed,
    DC: Keyed,
{
    type IntoValues = C::IntoValues;

    #[inline]
    fn into_values(self) -> C::IntoValues {
        self.container.into_values()
    }
}

impl<'a, K, C, DC> Iter<'a, K> for Recorder<C, DC>
where
    C: Iter<'a, K> + Keyed,
    DC: Keyed,
    Self: 'a,
{
    type Iter = C::Iter;

    #[inline]
    fn iter(&'a self) -> C::Iter {
        self.container.iter()
    }
}

impl<K, C, DC> IntoIter<K> for Recorder<C, DC>
where
    C: IntoIter<K> + Keyed,
    DC: Keyed,
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
    DC: Keyed,
{
    #[inline]
    pub fn into_iter(self) -> C::IntoIter {
        self.into_iter()
    }
}*/

impl<
    C: Keyed + ApplyDelta<DC>,
    RDC: Keyed,
    DC: IntoIter<C::Key> + Keyed<Value = C::Value, Key = C::Key>,
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
    C: Keyed + ApplyDelta<DC>,
    RDC: Keyed,
    DC: Clone + IntoIter<C::Key> + Keyed<Value = C::Value, Key = C::Key>,
> Recorder<C, RDC>
where
    Self: Keyed<Value = C::Value, Key = C::Key> + Insert<C::Key> + Remove<C::Key>,
{
    #[inline]
    pub fn apply_delta(&mut self, delta: &Delta<DC>) {
        self.container.apply_delta(delta);
    }
}*/

/// Extend (merge) the recorder's recorded delta with the delta passed as an
/// argument.
pub trait ExtendDelta<DC> {
    /// Extend (merge) the recorder's recorded delta with the delta passed as an
    /// argument.
    fn extend_delta(&mut self, delta: Delta<DC>);
}

impl<C: Keyed + ApplyDelta<DC>, DC: IntoIter<C::Key> + Keyed<Value = C::Value, Key = C::Key>>
    ExtendDelta<DC> for Recorder<C, DC>
where
    Delta<DC>: Clone + Default + MergeDeltas<DC>,
{
    #[inline]
    fn extend_delta(&mut self, delta: Delta<DC>) {
        self.extend_delta(delta);
    }
}

impl<C: Keyed + ApplyDelta<DC>, DC: IntoIter<C::Key> + Keyed<Value = C::Value, Key = C::Key>>
    Recorder<C, DC>
where
    Delta<DC>: Clone + Default + MergeDeltas<DC>,
{
    /// Extend (merge) the recorder's recorded delta with the delta passed as an
    /// argument.
    #[inline]
    pub fn extend_delta(&mut self, delta: Delta<DC>) {
        self.apply_delta(delta.clone());

        let delta = core::mem::replace(&mut self.delta, Default::default()).merge_deltas(delta);
        self.delta = delta;
    }
}

impl<V, DC> ExtendDelta<DC> for PhantomData<V> {
    #[inline]
    fn extend_delta(&mut self, _delta: Delta<DC>) {
        // Nothing happens, obviously.
        ()
    }
}

/// Flush the recorder, returning the recorded delta and replacing it with a
/// new empty one.
pub trait FlushDelta<DC> {
    /// Flush the recorder, returning the recorded delta and replacing it with a
    /// new empty one.
    fn flush_delta(&mut self) -> Delta<DC>;
}

impl<C: Keyed, DC: Keyed + Default> FlushDelta<DC> for Recorder<C, DC> {
    #[inline]
    fn flush_delta(&mut self) -> Delta<DC> {
        self.flush_delta()
    }
}

impl<C: Keyed + Default + ApplyDelta<DC>, DC: Keyed + Default> FlushDelta<Recorder<C, DC>>
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

impl<C: Keyed, DC: Keyed + Default> Recorder<C, DC> {
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

impl<C: Keyed + ApplyDelta<DC>, DC: Keyed + Default> ResetDelta<DC> for Recorder<C, DC> {
    #[inline]
    fn reset_delta(&mut self) {
        self.reset_delta()
    }
}

impl<C: Keyed + ApplyDelta<DC>, DC: Keyed + Default> Recorder<C, DC> {
    /// Reset the currently recorded delta by flushing it out of the recorder and
    /// then applying its reverse.
    #[inline]
    pub fn reset_delta(&mut self) {
        let delta = self.flush_delta();
        self.container.apply_delta(delta.reverse());
    }
}

impl<
    C: Keyed + Default + ApplyDelta<DC>,
    DC: IntoIter<C::Key> + Keyed<Value = C::Value, Key = C::Key> + Default,
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
