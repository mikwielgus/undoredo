// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use maplike::{Insert, IntoIter, Keyed, StableRemove};

/// A reversible set of changes to a collection.
///
/// Consists of a collection of removed elements and another collection of
/// inserted elements.
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

impl<EC: Default> Default for Edit<EC> {
    fn default() -> Self {
        Self {
            removed: Default::default(),
            inserted: Default::default(),
        }
    }
}

/// Apply the changes in an edit to a collection.
pub trait ApplyEdit<EC> {
    /// Apply the changes in an edit to a collection.
    ///
    /// This can be used to revert a previously recorded edit. The edit has to
    /// be reversed first with [`Edit::reverse()`].
    fn apply_edit(&mut self, edit: &Edit<EC>);
}

impl<
    K: Clone,
    V: Clone,
    C: Insert<K, Item = V> + StableRemove<K>,
    EC: Clone + IntoIter<K, Item = V> + Keyed<Key = K>,
> ApplyEdit<EC> for C
{
    fn apply_edit(&mut self, edit: &Edit<EC>) {
        for (removed_key, _removed_value) in edit.removed.clone().into_iter() {
            self.remove(&removed_key);
        }

        for (inserted_key, inserted_value) in edit.inserted.clone().into_iter() {
            self.insert(inserted_key.clone(), inserted_value.clone());
        }
    }
}
