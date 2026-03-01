// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};
#[cfg(feature = "rstared")]
use maplike::Get;
use maplike::{Insert, IntoIter, KeyedCollection, Remove};

use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[cfg(feature = "stable-vec")]
use stable_vec::StableVecFacade;

#[cfg(feature = "thunderdome")]
use thunderdome::{Arena, Index};

#[cfg(feature = "rstar")]
use rstar::{RTree, RTreeObject};

#[cfg(feature = "rstared")]
use rstared::RTreed;

/// A reversible set of changes to a collection.
///
/// Consists of a collection of removed elements and another collection of
/// inserted elements.
#[derive(Clone, Debug, Default)]
pub struct Delta<DC> {
    pub(super) removed: DC,
    pub(super) inserted: DC,
}

impl<DC: Default> Delta<DC> {
    /// Create a new empty delta with no recorded changes.
    pub fn new() -> Self {
        Self {
            removed: Default::default(),
            inserted: Default::default(),
        }
    }
}

impl<DC> Delta<DC> {
    /// Create an new delta from collections of removals and insertions.
    pub fn with_removed_inserted(removed: DC, inserted: DC) -> Self {
        Self { removed, inserted }
    }

    /// Consume the delta and return removed and inserted collections.
    pub fn dissolve(self) -> (DC, DC) {
        (self.removed, self.inserted)
    }

    /// Reverse the delta.
    ///
    /// This is done by swapping the collections of removed and inserted
    /// elements.
    pub fn reverse(self) -> Self {
        Self {
            removed: self.inserted,
            inserted: self.removed,
        }
    }
}

/// Apply the changes in an delta to a collection.
///
/// This can be used to revert a previously recorded delta. The delta has to
/// be reversed first with [`Delta::reverse()`].
pub trait ApplyDelta<DC> {
    /// Apply the changes in an delta to a collection.
    ///
    /// This can be used to revert a previously recorded delta. The delta has to
    /// be reversed first with [`Delta::reverse()`].
    fn apply_delta(&mut self, delta: &Delta<DC>);
}

fn apply_delta_on_map<K, C, DC>(collection: &mut C, delta: &Delta<DC>)
where
    C: KeyedCollection<Key = K> + Insert<K> + Remove<K>,
    DC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = C::Value>,
{
    for (removed_key, _removed_value) in delta.removed.clone().into_iter() {
        collection.remove(&removed_key);
    }

    for (inserted_key, inserted_value) in delta.inserted.clone().into_iter() {
        collection.insert(inserted_key, inserted_value);
    }
}

impl<V: Clone, DC: Clone + IntoIter<usize, Value = V>> ApplyDelta<DC> for Vec<V>
where
    DC::IntoIter: DoubleEndedIterator,
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        // This implementation is different than the others because stable
        // removal is impossible on `Vec`.

        // We reverse the order of removeds to be descending so that we never
        // miss a pop.
        for (removed_index, _removed_value) in delta.removed.clone().into_iter().rev() {
            if removed_index + 1 == self.len() {
                self.pop();
            } else {
                // No-op. The value will just get overridden by the subsequent
                // insertion.
            }
        }

        for (inserted_index, inserted_value) in delta.inserted.clone().into_iter() {
            if inserted_index == self.len() {
                self.push(inserted_value);
            } else {
                Insert::insert(self, inserted_index, inserted_value);
            }
        }
    }
}

impl<K: Ord, V, DC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = V>> ApplyDelta<DC>
    for BTreeMap<K, V>
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

impl<K: Ord, DC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = ()>> ApplyDelta<DC>
    for BTreeSet<K>
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

impl<V, DC> ApplyDelta<DC> for PhantomData<V> {
    fn apply_delta(&mut self, _delta: &Delta<DC>) {
        // Nothing happens, obviously.
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V, DC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = V>> ApplyDelta<DC>
    for HashMap<K, V>
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, DC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = ()>> ApplyDelta<DC>
    for HashSet<K>
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "stable-vec")]
impl<
    V,
    C: stable_vec::core::Core<V>,
    DC: Clone + IntoIter<usize> + KeyedCollection<Key = usize, Value = V>,
> ApplyDelta<DC> for StableVecFacade<V, C>
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "thunderdome")]
impl<V, DC: Clone + IntoIter<Index> + KeyedCollection<Key = Index, Value = V>> ApplyDelta<DC>
    for Arena<V>
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "rstar")]
impl<K: RTreeObject + PartialEq, DC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = ()>>
    ApplyDelta<DC> for RTree<K>
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "rstared")]
impl<
    K: Clone + PartialEq,
    V: Clone + PartialEq + RTreeObject,
    C: Get<K> + KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    DC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = V>,
> ApplyDelta<DC> for RTreed<C>
{
    fn apply_delta(&mut self, delta: &Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}
