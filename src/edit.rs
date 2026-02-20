// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::{BTreeMap, BTreeSet};
use maplike::{Insert, IntoIter, Keyed, Map, StableRemove};

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

fn apply_edit<
    K,
    V,
    C: Insert<K, Item = V> + StableRemove<K>,
    EC: Clone + IntoIter<K, Item = V> + Keyed<Key = K>,
>(
    collection: &mut C,
    edit: &Edit<EC>,
) {
    for (removed_key, _removed_value) in edit.removed.clone().into_iter() {
        collection.remove(&removed_key);
    }

    for (inserted_key, inserted_value) in edit.inserted.clone().into_iter() {
        collection.insert(inserted_key, inserted_value);
    }
}

impl<K: Ord, V, EC: Clone + IntoIter<K, Item = V> + Keyed<Key = K>> ApplyEdit<EC>
    for BTreeMap<K, V>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit(self, edit);
    }
}

impl<K: Ord, EC: Clone + IntoIter<K, Item = ()> + Keyed<Key = K>> ApplyEdit<EC> for BTreeSet<K> {
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit(self, edit);
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V, EC: Clone + IntoIter<K, Item = V> + Keyed<Key = K>> ApplyEdit<EC>
    for HashMap<K, V>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit(self, edit);
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, EC: Clone + IntoIter<K, Item = ()> + Keyed<Key = K>> ApplyEdit<EC>
    for HashSet<K>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit(self, edit);
    }
}

#[cfg(feature = "stable-vec")]
impl<V, C: stable_vec::core::Core<V>, EC: Clone + IntoIter<usize, Item = V> + Keyed<Key = usize>>
    ApplyEdit<EC> for StableVecFacade<V, C>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit(self, edit);
    }
}

#[cfg(feature = "thunderdome")]
impl<V, EC: Clone + IntoIter<Index, Item = V> + Keyed<Key = Index>> ApplyEdit<EC> for Arena<V> {
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit(self, edit);
    }
}

#[cfg(feature = "rstar")]
impl<K: RTreeObject + PartialEq, EC: Clone + IntoIter<K, Item = ()> + Keyed<Key = K>> ApplyEdit<EC>
    for RTree<K>
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit(self, edit);
    }
}

impl<
    C: Keyed + Map,
    REC: Keyed + Map,
    EC: Clone + IntoIter<C::Key, Item = C::Item> + Keyed<Key = C::Key>,
> ApplyEdit<EC> for crate::recorder::Recorder<C, REC>
where
    Self: Insert<C::Key, Item = C::Item> + StableRemove<C::Key>,
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        apply_edit(self, edit);
    }
}
