// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
    vec::Vec,
};
use maplike::abc::Keyed;
use maplike::iter::IntoIter;
use maplike::ops::{Get, Insert, Len, Pop, Push, Remove, Resize, Set};

#[cfg(any(feature = "std", feature = "indexmap"))]
use core::hash::Hash;
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "std")]
use std::sync::Arc;

#[cfg(feature = "indexmap")]
use indexmap::{IndexMap, IndexSet};

#[cfg(feature = "bidimap")]
use bidimap::BiBTreeMap;
#[cfg(all(feature = "bidimap", feature = "std"))]
use bidimap::BiHashMap;

#[cfg(feature = "stable-vec")]
use stable_vec::StableVecFacade;

#[cfg(feature = "thunderdome")]
use thunderdome::{Arena, Index};

#[cfg(feature = "rstar")]
use rstar::{RTree, RTreeObject};

#[cfg(feature = "tinyvec")]
use tinyvec::{Array, ArrayVec, TinyVec};

use crate::{ApplyEdit, ExtractEdit, FlushDelta, RevertEdit};

/// A reversible set of changes to a container.
///
/// Consists of a container of removed elements and another container of
/// inserted elements.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Delta<DC> {
    pub(super) removed: DC,
    pub(super) inserted: DC,
}

impl<DC: Default> Delta<DC> {
    /// Create a new empty delta with no recorded changes.
    #[inline]
    pub fn new() -> Self {
        Self {
            removed: Default::default(),
            inserted: Default::default(),
        }
    }
}

impl<DC> Delta<DC> {
    /// Create an new delta from containers of removals and insertions.
    #[inline]
    pub fn with_removed_inserted(removed: DC, inserted: DC) -> Self {
        Self { removed, inserted }
    }

    /// Consume the delta and return removed and inserted containers.
    #[inline]
    pub fn dissolve(self) -> (DC, DC) {
        (self.removed, self.inserted)
    }

    /// Reverse the delta.
    ///
    /// This is done by swapping the containers of removed and inserted
    /// elements.
    #[inline]
    pub fn reverse(self) -> Self {
        Self {
            removed: self.inserted,
            inserted: self.removed,
        }
    }
}

/// Merge two deltas into one.
///
/// Applying the result is equivalent to applying `self` and then `other`.
pub trait MergeDeltas<DC> {
    /// Merge `other` into `self`.
    fn merge_deltas(self, other: Delta<DC>) -> Self;
}

#[inline]
fn merge_map_delta<K, V, M>(self_delta: Delta<M>, other: Delta<M>) -> Delta<M>
where
    M: Keyed<Value = V, Key = K> + IntoIter<K> + Get<K> + Insert<K> + Remove<K, Output = Option<V>>,
{
    let (mut self_removed, mut self_inserted) = self_delta.dissolve();
    let (other_removed, other_inserted) = other.dissolve();

    for (key, value) in IntoIter::into_iter(other_removed) {
        if Remove::remove(&mut self_inserted, &key).is_none()
            && Get::get(&self_removed, &key).is_none()
        {
            Insert::insert(&mut self_removed, key, value);
        }
    }

    for (key, value) in IntoIter::into_iter(other_inserted) {
        Insert::insert(&mut self_inserted, key, value);
    }

    Delta::with_removed_inserted(self_removed, self_inserted)
}

impl<K, V> MergeDeltas<BTreeMap<K, V>> for Delta<BTreeMap<K, V>>
where
    K: Ord,
{
    #[inline]
    fn merge_deltas(self, other: Self) -> Self {
        merge_map_delta(self, other)
    }
}

#[cfg(feature = "std")]
impl<K, V> MergeDeltas<HashMap<K, V>> for Delta<HashMap<K, V>>
where
    K: Eq + Hash,
{
    #[inline]
    fn merge_deltas(self, other: Self) -> Self {
        merge_map_delta(self, other)
    }
}

#[cfg(feature = "indexmap")]
impl<K, V> MergeDeltas<IndexMap<K, V>> for Delta<IndexMap<K, V>>
where
    K: Eq + Hash,
{
    #[inline]
    fn merge_deltas(self, other: Self) -> Self {
        merge_map_delta(self, other)
    }
}

impl<DC> Delta<DC>
where
    Self: MergeDeltas<DC>,
{
    /// Merge two deltas into one.
    ///
    /// Applying the result is equivalent to applying `self` and then `other`.
    #[inline]
    pub fn merge_deltas(self, other: Self) -> Self {
        MergeDeltas::merge_deltas(self, other)
    }
}

impl<DC: Clone, T: ApplyDelta<DC>> ApplyEdit<T> for Delta<DC> {
    #[inline]
    fn apply_edit(self, target: &mut T) {
        target.apply_delta(self);
    }
}

impl<DC: Clone, T: ApplyDelta<DC>> RevertEdit<T> for Delta<DC> {
    #[inline]
    fn revert_edit(self, target: &mut T) -> Self {
        let reverse = self.reverse();
        target.apply_delta(reverse.clone());

        reverse
    }
}

impl<DC, T: FlushDelta<DC>> ExtractEdit<T> for Delta<DC> {
    #[inline]
    fn extract_edit(target: &mut T) -> Self {
        target.flush_delta()
    }
}

/// Apply the changes in a delta to a container.
///
/// This can be used to revert a previously recorded delta if you reverse it
/// with [`Delta::reverse()`].
pub trait ApplyDelta<DC> {
    /// Apply the changes in a delta to a container.
    ///
    /// This can be used to revert a previously recorded delta. The delta has to
    /// be reversed first with [`Delta::reverse()`].
    fn apply_delta(&mut self, delta: Delta<DC>);
}

#[inline]
fn apply_delta_on_map<K, C, DC>(container: &mut C, delta: Delta<DC>)
where
    C: Keyed<Key = K> + Insert<K> + Remove<K>,
    DC: IntoIter<K> + Keyed<Value = C::Value, Key = K>,
{
    let (removed, inserted) = delta.dissolve();

    for (removed_key, _removed_value) in removed.into_iter() {
        container.remove(&removed_key);
    }

    for (inserted_key, inserted_value) in inserted.into_iter() {
        container.insert(inserted_key, inserted_value);
    }
}

#[inline]
fn apply_delta_on_vec<V, C, DC>(container: &mut C, delta: Delta<DC>)
where
    V: Clone,
    C: Pop<Value = V> + Push<usize, Value = V> + Set<usize, Value = V> + Len + Resize,
    DC: Clone + IntoIter<usize, Value = V>,
    DC::IntoIter: DoubleEndedIterator,
{
    let (removed, inserted) = delta.dissolve();

    // This implementation is different than the others because stable element
    // removal is impossible in `Vec` and some other types similar to `Vec`.

    // We reverse the order of removeds to be descending so that we never
    // miss a pop.
    for (removed_index, _removed_value) in removed.into_iter().rev() {
        if removed_index + 1 == Len::len(container) {
            Pop::pop(container);
        } else {
            // No-op. The value will just get overridden by the subsequent
            // insertion.
        }
    }

    for (inserted_index, inserted_value) in inserted.into_iter() {
        if inserted_index == Len::len(container) {
            Push::push(container, inserted_value);
        } else if inserted_index < Len::len(container) {
            Set::set(container, inserted_index, inserted_value);
        } else {
            Resize::resize(container, inserted_index + 1, inserted_value);
        }
    }
}

impl<V: Clone, DC: Clone + IntoIter<usize, Value = V>> ApplyDelta<DC> for Vec<V>
where
    DC::IntoIter: DoubleEndedIterator,
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_vec(self, delta);
    }
}

impl<V: Clone, DC: Clone + IntoIter<usize, Value = V>> ApplyDelta<DC> for VecDeque<V>
where
    DC::IntoIter: DoubleEndedIterator,
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_vec(self, delta);
    }
}

impl<K: Ord, V, DC: IntoIter<K> + Keyed<Value = V, Key = K>> ApplyDelta<DC> for BTreeMap<K, V> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "bidimap")]
impl<L: Ord, R: Ord, DC: IntoIter<L> + Keyed<Value = R, Key = L>> ApplyDelta<DC>
    for BiBTreeMap<L, R>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (removed, inserted) = delta.dissolve();

        for (removed_key, _removed_value) in removed.into_iter() {
            self.remove_by_left(&removed_key);
        }

        for (inserted_key, inserted_value) in inserted.into_iter() {
            self.insert(inserted_key, inserted_value);
        }
    }
}

impl<K: Ord, DC: IntoIter<K> + Keyed<Value = (), Key = K>> ApplyDelta<DC> for BTreeSet<K> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

macro_rules! impl_delta_for_scalar {
    ($($half_delta:ident, $delta:ident, $t:ty);+ $(;)?) => {
        $(
            impl ApplyDelta<BTreeMap<usize, $t>> for $t {
                #[inline]
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

impl_delta_for_scalar! {
    I8HalfDelta, I8Delta, i8;
    I16HalfDelta, I16Delta, i16;
    I32HalfDelta, I32Delta, i32;
    I64HalfDelta, I64Delta, i64;
    I128HalfDelta, I128Delta, i128;
    IsizeHalfDelta, IsizeDelta, isize;
    U8HalfDelta, U8Delta, u8;
    U16HalfDelta, U16Delta, u16;
    U32HalfDelta, U32Delta, u32;
    U64HalfDelta, U64Delta, u64;
    U128HalfDelta, U128Delta, u128;
    UsizeHalfDelta, UsizeDelta, usize;
    F32HalfDelta, F32Delta, f32;
    F64HalfDelta, F64Delta, f64;
    CharHalfDelta, CharDelta, char;
    BoolHalfDelta, BoolDelta, bool;
    UnitHalfDelta, UnitDelta, ();
}

macro_rules! impl_delta_for_tuple {
    ($half_delta:ident, $delta:ident, $($idx:tt $typ:ident),+ $(,)?) => {
        impl<$($typ,)+> ApplyDelta<BTreeMap<usize, ($($typ,)+)>> for ($($typ,)+) {
            #[inline]
            fn apply_delta(&mut self, delta: Delta<BTreeMap<usize, ($($typ,)+)>>) {
                let (_removed, mut inserted) = delta.dissolve();
                if let Some(value) = inserted.remove(&0) {
                    *self = value;
                }
            }
        }
    };
}

impl_delta_for_tuple!(Tuple1HalfDelta, Tuple1Delta, 0 T0);
impl_delta_for_tuple!(Tuple2HalfDelta, Tuple2Delta, 0 T0, 1 T1);
impl_delta_for_tuple!(Tuple3HalfDelta, Tuple3Delta, 0 T0, 1 T1, 2 T2);
impl_delta_for_tuple!(Tuple4HalfDelta, Tuple4Delta, 0 T0, 1 T1, 2 T2, 3 T3);
impl_delta_for_tuple!(Tuple5HalfDelta, Tuple5Delta, 0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
impl_delta_for_tuple!(Tuple6HalfDelta, Tuple6Delta, 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
impl_delta_for_tuple!(Tuple7HalfDelta, Tuple7Delta, 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
impl_delta_for_tuple!(
    Tuple8HalfDelta,
    Tuple8Delta,
    0 T0,
    1 T1,
    2 T2,
    3 T3,
    4 T4,
    5 T5,
    6 T6,
    7 T7
);
impl_delta_for_tuple!(
    Tuple9HalfDelta,
    Tuple9Delta,
    0 T0,
    1 T1,
    2 T2,
    3 T3,
    4 T4,
    5 T5,
    6 T6,
    7 T7,
    8 T8
);
impl_delta_for_tuple!(
    Tuple10HalfDelta,
    Tuple10Delta,
    0 T0,
    1 T1,
    2 T2,
    3 T3,
    4 T4,
    5 T5,
    6 T6,
    7 T7,
    8 T8,
    9 T9
);
impl_delta_for_tuple!(
    Tuple11HalfDelta,
    Tuple11Delta,
    0 T0,
    1 T1,
    2 T2,
    3 T3,
    4 T4,
    5 T5,
    6 T6,
    7 T7,
    8 T8,
    9 T9,
    10 T10
);
impl_delta_for_tuple!(
    Tuple12HalfDelta,
    Tuple12Delta,
    0 T0,
    1 T1,
    2 T2,
    3 T3,
    4 T4,
    5 T5,
    6 T6,
    7 T7,
    8 T8,
    9 T9,
    10 T10,
    11 T11
);

impl<V, DC: IntoIter<usize, Value = V>> ApplyDelta<DC> for Option<V> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (removed, inserted) = delta.dissolve();

        for (removed_key, _removed_value) in removed.into_iter() {
            self.remove(&removed_key);
        }

        for (inserted_key, inserted_value) in inserted.into_iter() {
            self.set(inserted_key, inserted_value);
        }
    }
}

impl<V, DC: IntoIter<usize, Value = V>> ApplyDelta<DC> for Box<V> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (_removed, inserted) = delta.dissolve();

        for (inserted_key, inserted_value) in inserted.into_iter() {
            Set::set(self, inserted_key, inserted_value);
        }
    }
}

impl<V, DC: IntoIter<usize, Value = V>> ApplyDelta<DC> for Rc<V> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (_removed, inserted) = delta.dissolve();

        for (inserted_key, inserted_value) in inserted.into_iter() {
            Set::set(self, inserted_key, inserted_value);
        }
    }
}

#[cfg(feature = "std")]
impl<V, DC: IntoIter<usize, Value = V>> ApplyDelta<DC> for Arc<V> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (_removed, inserted) = delta.dissolve();

        for (inserted_key, inserted_value) in inserted.into_iter() {
            Set::set(self, inserted_key, inserted_value);
        }
    }
}

impl<V, DC> ApplyDelta<DC> for PhantomData<V> {
    #[inline]
    fn apply_delta(&mut self, _delta: Delta<DC>) {
        // Nothing happens here, obviously.
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V, DC: IntoIter<K> + Keyed<Value = V, Key = K>> ApplyDelta<DC>
    for HashMap<K, V>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(all(feature = "bidimap", feature = "std"))]
impl<L: Eq + Hash, R: Eq + Hash, DC: IntoIter<L> + Keyed<Value = R, Key = L>> ApplyDelta<DC>
    for BiHashMap<L, R>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (removed, inserted) = delta.dissolve();

        for (removed_key, _removed_value) in removed.into_iter() {
            self.remove_by_left(&removed_key);
        }

        for (inserted_key, inserted_value) in inserted.into_iter() {
            self.insert(inserted_key, inserted_value);
        }
    }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, DC: IntoIter<K> + Keyed<Value = (), Key = K>> ApplyDelta<DC> for HashSet<K> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "indexmap")]
impl<K: Eq + Hash, V, DC: IntoIter<K> + Keyed<Value = V, Key = K>> ApplyDelta<DC>
    for IndexMap<K, V>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "indexmap")]
impl<K: Eq + Hash, DC: IntoIter<K> + Keyed<Value = (), Key = K>> ApplyDelta<DC> for IndexSet<K> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "rstar")]
impl<K: RTreeObject + PartialEq, DC: IntoIter<K> + Keyed<Value = (), Key = K>> ApplyDelta<DC>
    for RTree<K>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "stable-vec")]
impl<V, C: stable_vec::core::Core<V>, DC: IntoIter<usize> + Keyed<Value = V, Key = usize>>
    ApplyDelta<DC> for StableVecFacade<V, C>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "thunderdome")]
impl<V, DC: IntoIter<Index> + Keyed<Value = V, Key = Index>> ApplyDelta<DC> for Arena<V> {
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "tinyvec")]
impl<A: Array, DC: Clone + IntoIter<usize, Value = A::Item>> ApplyDelta<DC> for ArrayVec<A>
where
    A::Item: Clone,
    DC::IntoIter: DoubleEndedIterator,
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_vec(self, delta);
    }
}

#[cfg(feature = "tinyvec")]
impl<A: Array, DC: Clone + IntoIter<usize, Value = A::Item>> ApplyDelta<DC> for TinyVec<A>
where
    A::Item: Clone,
    DC::IntoIter: DoubleEndedIterator,
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_vec(self, delta);
    }
}
