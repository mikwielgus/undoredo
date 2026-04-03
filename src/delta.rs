// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};
#[cfg(feature = "rstared")]
use maplike::Get;
use maplike::{Container, Insert, IntoIter, Remove};

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

/// A reversible set of changes to a container.
///
/// Consists of a container of removed elements and another container of
/// inserted elements.
#[derive(Clone, Debug, Default, PartialEq)]
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
    /// Create an new delta from containers of removals and insertions.
    pub fn with_removed_inserted(removed: DC, inserted: DC) -> Self {
        Self { removed, inserted }
    }

    /// Consume the delta and return removed and inserted containers.
    pub fn dissolve(self) -> (DC, DC) {
        (self.removed, self.inserted)
    }

    /// Reverse the delta.
    ///
    /// This is done by swapping the containers of removed and inserted
    /// elements.
    pub fn reverse(self) -> Self {
        Self {
            removed: self.inserted,
            inserted: self.removed,
        }
    }
}

/// Apply the changes in an delta to a container.
///
/// This can be used to revert a previously recorded delta if you reverse it
/// with [`Delta::reverse()`].
pub trait ApplyDelta<DC> {
    /// Apply the changes in an delta to a container.
    ///
    /// This can be used to revert a previously recorded delta. The delta has to
    /// be reversed first with [`Delta::reverse()`].
    fn apply_delta(&mut self, delta: Delta<DC>);
}

fn apply_delta_on_map<K, C, DC>(container: &mut C, delta: Delta<DC>)
where
    C: Container<Key = K> + Insert<K> + Remove<K>,
    DC: IntoIter<K> + Container<Key = K, Value = C::Value>,
{
    let (removed, inserted) = delta.dissolve();

    for (removed_key, _removed_value) in removed.into_iter() {
        container.remove(&removed_key);
    }

    for (inserted_key, inserted_value) in inserted.into_iter() {
        container.insert(inserted_key, inserted_value);
    }
}

impl<V: Clone, DC: Clone + IntoIter<usize, Value = V>> ApplyDelta<DC> for Vec<V>
where
    DC::IntoIter: DoubleEndedIterator,
{
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (removed, inserted) = delta.dissolve();
        // This implementation is different than the others because stable
        // removal is impossible on `Vec`.

        // We reverse the order of removeds to be descending so that we never
        // miss a pop.
        for (removed_index, _removed_value) in removed.into_iter().rev() {
            if removed_index + 1 == self.len() {
                self.pop();
            } else {
                // No-op. The value will just get overridden by the subsequent
                // insertion.
            }
        }

        for (inserted_index, inserted_value) in inserted.into_iter() {
            if inserted_index == self.len() {
                self.push(inserted_value);
            } else {
                if inserted_index < self.len() {
                    self[inserted_index] = inserted_value;
                } else {
                    self.resize(inserted_index + 1, inserted_value);
                }
            }
        }
    }
}

impl<K: Ord, V, DC: IntoIter<K> + Container<Key = K, Value = V>> ApplyDelta<DC> for BTreeMap<K, V> {
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

impl<K: Ord, DC: IntoIter<K> + Container<Key = K, Value = ()>> ApplyDelta<DC> for BTreeSet<K> {
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

macro_rules! impl_apply_delta_for_scalar {
    ($($t:ty),+ $(,)?) => {
        $(
            impl ApplyDelta<BTreeMap<usize, $t>> for $t {
                fn apply_delta(&mut self, delta: Delta<BTreeMap<usize, $t>>) {
                    let (_removed, mut inserted) = delta.dissolve();
                    if let Some(value) = inserted.remove(&0) {
                        *self = value;
                    }
                }
            }
        )+
    };
}

impl_apply_delta_for_scalar!(i8, i16, i32, i64, i128, isize);
impl_apply_delta_for_scalar!(u8, u16, u32, u64, u128, usize);
impl_apply_delta_for_scalar!(f32, f64);
impl_apply_delta_for_scalar!(char, bool, ());

macro_rules! impl_apply_delta_for_tuple {
    ($($idx:tt $typ:ident),+ $(,)?) => {
        impl<$($typ,)+> ApplyDelta<BTreeMap<usize, ($($typ,)+)>> for ($($typ,)+) {
            fn apply_delta(&mut self, delta: Delta<BTreeMap<usize, ($($typ,)+)>>) {
                let (_removed, mut inserted) = delta.dissolve();
                if let Some(value) = inserted.remove(&0) {
                    *self = value;
                }
            }
        }
    };
}

impl_apply_delta_for_tuple!(0 T0);
impl_apply_delta_for_tuple!(0 T0, 1 T1);
impl_apply_delta_for_tuple!(0 T0, 1 T1, 2 T2);
impl_apply_delta_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3);
impl_apply_delta_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
impl_apply_delta_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
impl_apply_delta_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
impl_apply_delta_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7);
impl_apply_delta_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8);
impl_apply_delta_for_tuple!(
    0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9
);
impl_apply_delta_for_tuple!(
    0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10
);
impl_apply_delta_for_tuple!(
    0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11
);

impl<V, DC> ApplyDelta<DC> for PhantomData<V> {
    fn apply_delta(&mut self, _delta: Delta<DC>) {
        // Nothing happens here, obviously.
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V, DC: IntoIter<K> + Container<Key = K, Value = V>> ApplyDelta<DC>
    for HashMap<K, V>
{
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, DC: IntoIter<K> + Container<Key = K, Value = ()>> ApplyDelta<DC> for HashSet<K> {
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "stable-vec")]
impl<V, C: stable_vec::core::Core<V>, DC: IntoIter<usize> + Container<Key = usize, Value = V>>
    ApplyDelta<DC> for StableVecFacade<V, C>
{
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "thunderdome")]
impl<V, DC: IntoIter<Index> + Container<Key = Index, Value = V>> ApplyDelta<DC> for Arena<V> {
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "rstar")]
impl<K: RTreeObject + PartialEq, DC: IntoIter<K> + Container<Key = K, Value = ()>> ApplyDelta<DC>
    for RTree<K>
{
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "rstared")]
impl<
    K: Clone + PartialEq,
    V: Clone + PartialEq + RTreeObject,
    C: Get<K> + Container<Key = K, Value = V> + Insert<K> + Remove<K>,
    DC: IntoIter<K> + Container<Key = K, Value = V>,
> ApplyDelta<DC> for RTreed<C>
{
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}
