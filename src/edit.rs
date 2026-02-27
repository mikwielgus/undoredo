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
pub struct Edit<EC> {
    pub(super) removed: EC,
    pub(super) inserted: EC,
}

impl<EC: Default> Edit<EC> {
    /// Create a new empty edit with no recorded changes.
    pub fn new() -> Self {
        Self {
            removed: Default::default(),
            inserted: Default::default(),
        }
    }
}

impl<EC> Edit<EC> {
    /// Create an new edit from collections of removals and insertions.
    pub fn with_removed_inserted(removed: EC, inserted: EC) -> Self {
        Self { removed, inserted }
    }

    /// Consume the edit and return removed and inserted collections.
    pub fn dissolve(self) -> (EC, EC) {
        (self.removed, self.inserted)
    }

    /// Reverse the edit.
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

/// Apply the changes in an edit to a collection.
///
/// This can be used to revert a previously recorded edit. The edit has to
/// be reversed first with [`Edit::reverse()`].
pub trait ApplyEdit<EC> {
    /// Apply the changes in an edit to a collection.
    ///
    /// This can be used to revert a previously recorded edit. The edit has to
    /// be reversed first with [`Edit::reverse()`].
    fn apply_edit(&mut self, edit: &Edit<EC>);
}

fn apply_edit_on_map<K, C, EC>(collection: &mut C, edit: &Edit<EC>)
where
    C: KeyedCollection<Key = K> + Insert<K> + Remove<K>,
    EC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = C::Value>,
{
    for (removed_key, _removed_value) in edit.removed.clone().into_iter() {
        collection.remove(&removed_key);
    }

    for (inserted_key, inserted_value) in edit.inserted.clone().into_iter() {
        collection.insert(inserted_key, inserted_value);
    }
}

impl<V: Clone, EC: Clone + IntoIter<usize, Value = V>> ApplyEdit<EC> for Vec<V> {
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        // This implementation is different than the others because stable
        // removal is impossible on `Vec`.

        // We reverse the order of removeds to have removals become
        // corresponding pops of all pushes in their reverse order. Otherwise,
        // the no-op branch would get triggered.
        for (removed_index, _removed_value) in edit.removed.clone().into_iter() {
            if removed_index == self.len() - 1 {
                self.pop();
            } else {
                // No-op. The value will just get overridden by the subsequent
                // insertion.
            }
        }

        for (inserted_index, inserted_value) in edit.inserted.clone().into_iter() {
            if inserted_index == self.len() {
                self.push(inserted_value);
            } else {
                Insert::insert(self, inserted_index, inserted_value);
            }
        }
    }
}

impl<K: Ord, V, EC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = V>> ApplyEdit<EC>
    for BTreeMap<K, V>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit_on_map(self, edit);
    }
}

impl<K: Ord, EC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = ()>> ApplyEdit<EC>
    for BTreeSet<K>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit_on_map(self, edit);
    }
}

impl<V, EC> ApplyEdit<EC> for PhantomData<V> {
    fn apply_edit(&mut self, _edit: &Edit<EC>) {
        // Nothing happens, obviously.
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V, EC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = V>> ApplyEdit<EC>
    for HashMap<K, V>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit_on_map(self, edit);
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, EC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = ()>> ApplyEdit<EC>
    for HashSet<K>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit_on_map(self, edit);
    }
}

#[cfg(feature = "stable-vec")]
impl<
    V,
    C: stable_vec::core::Core<V>,
    EC: Clone + IntoIter<usize> + KeyedCollection<Key = usize, Value = V>,
> ApplyEdit<EC> for StableVecFacade<V, C>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit_on_map(self, edit);
    }
}

#[cfg(feature = "thunderdome")]
impl<V, EC: Clone + IntoIter<Index> + KeyedCollection<Key = Index, Value = V>> ApplyEdit<EC>
    for Arena<V>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit_on_map(self, edit);
    }
}

#[cfg(feature = "rstar")]
impl<K: RTreeObject + PartialEq, EC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = ()>>
    ApplyEdit<EC> for RTree<K>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit_on_map(self, edit);
    }
}

#[cfg(feature = "rstared")]
impl<
    K: Clone + PartialEq,
    V: Clone + PartialEq + RTreeObject,
    C: Get<K> + KeyedCollection<Key = K, Value = V> + Insert<K> + Remove<K>,
    EC: Clone + IntoIter<K> + KeyedCollection<Key = K, Value = V>,
> ApplyEdit<EC> for RTreed<C>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit_on_map(self, edit);
    }
}
