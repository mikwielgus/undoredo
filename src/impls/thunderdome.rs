// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thunderdome::{Arena, Index};

use crate::map::{Keyed, Map, MapGet, MapInsert, MapIntoIter, MapPush, MapRemove};

impl<V> Map for Arena<V> {
    type Item = V;
}

impl<V> Keyed for Arena<V> {
    type Key = Index;
}

impl<V> MapGet<Index> for Arena<V> {
    #[inline(always)]
    fn get(&self, key: &Index) -> Option<&V> {
        Arena::get(self, *key)
    }
}

impl<V> MapInsert<Index> for Arena<V> {
    #[inline(always)]
    fn insert(&mut self, key: Index, value: V) {
        Arena::insert_at(self, key, value);
    }
}

impl<V> MapRemove<Index> for Arena<V> {
    #[inline(always)]
    fn remove(&mut self, key: &Index) -> Option<V> {
        Arena::remove(self, *key)
    }
}

impl<V> MapPush<Index> for Arena<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> Index {
        Arena::insert(self, value)
    }
}

impl<V> MapIntoIter<Index> for Arena<V> {
    type IntoIter = thunderdome::iter::IntoIter<V>;

    #[inline(always)]
    fn into_iter(self) -> thunderdome::iter::IntoIter<V> {
        IntoIterator::into_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use thunderdome::{Arena, Index};

    use crate::Recorder;

    #[test]
    fn test_apply_edit_at_generated_indexes() {
        let recorder = Recorder::<Index, i32, Arena<i32>, Arena<i32>>::new(Arena::new());
        crate::recorder::tests::test_apply_edit_at_generated_indexes(recorder);
    }

    #[test]
    fn test_insert_and_remove_at_generated_indexes() {
        let recorder = Recorder::<Index, i32, Arena<i32>, Arena<i32>>::new(Arena::new());
        crate::recorder::tests::test_insert_and_remove_at_generated_indexes(recorder);
    }

    #[test]
    fn test_edit_undo_redo_at_generated_indexes() {
        crate::undoredo::tests::test_edit_undo_redo_at_generated_indexes::<
            Index,
            Arena<i32>,
            Arena<i32>,
        >(Arena::new());
    }
}
