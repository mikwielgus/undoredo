// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use stable_vec::StableVecFacade;

use crate::map::{Get, Insert, IntoIter, Keyed, Map, Push, Remove};

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
