// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use stable_vec::StableVecFacade;

use crate::map::{Keyed, Map, Get, Insert, IntoIter, Push, Remove};

impl<V, C: stable_vec::core::Core<V>> Map for StableVecFacade<V, C> {
    type Item = V;
}

impl<V, C: stable_vec::core::Core<V>> Keyed for StableVecFacade<V, C> {
    type Key = usize;
}

impl<V, C: stable_vec::core::Core<V>> Get<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        StableVecFacade::get(self, *index)
    }
}

impl<V, C: stable_vec::core::Core<V>> Insert<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn insert(&mut self, index: usize, value: V) {
        StableVecFacade::reserve_for(self, index);
        StableVecFacade::insert(self, index, value);
    }
}

impl<V, C: stable_vec::core::Core<V>> Remove<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        self.get(*index)?;
        StableVecFacade::remove(self, *index)
    }
}

impl<V, C: stable_vec::core::Core<V>> Push<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        StableVecFacade::push(self, value)
    }
}

impl<V, C: stable_vec::core::Core<V>> IntoIter<usize> for StableVecFacade<V, C> {
    type IntoIter = stable_vec::iter::IntoIter<V, C>;

    #[inline(always)]
    fn into_iter(self) -> stable_vec::iter::IntoIter<V, C> {
        IntoIterator::into_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use stable_vec::StableVec;

    use crate::Recorder;

    #[test]
    fn test_apply_edit_at_generated_indexes() {
        let recorder =
            Recorder::<usize, i32, StableVec<i32>, StableVec<i32>>::new(StableVec::new());
        crate::recorder::tests::test_apply_edit_at_generated_indexes(recorder);
    }

    #[test]
    fn test_apply_edit_at_specified_indexes() {
        let recorder =
            Recorder::<usize, i32, StableVec<i32>, StableVec<i32>>::new(StableVec::new());
        crate::recorder::tests::test_apply_edit_at_specified_indexes(recorder);
    }

    #[test]
    fn test_insert_and_remove_at_generated_indexes() {
        let recorder =
            Recorder::<usize, i32, StableVec<i32>, StableVec<i32>>::new(StableVec::new());
        crate::recorder::tests::test_insert_and_remove_at_generated_indexes(recorder);
    }

    #[test]
    fn test_insert_and_remove_at_specified_indexes() {
        let recorder =
            Recorder::<usize, i32, StableVec<i32>, StableVec<i32>>::new(StableVec::new());
        crate::recorder::tests::test_insert_and_remove_at_specified_indexes(recorder);
    }

    #[test]
    fn test_edit_undo_redo_at_generated_indexes() {
        crate::undoredo::tests::test_edit_undo_redo_at_generated_indexes::<
            usize,
            StableVec<i32>,
            StableVec<i32>,
        >(StableVec::new());
    }

    #[test]
    fn test_edit_undo_redo_at_specified_indexes() {
        crate::undoredo::tests::test_edit_undo_redo_at_specified_indexes::<
            usize,
            StableVec<i32>,
            StableVec<i32>,
        >(StableVec::new());
    }
}
