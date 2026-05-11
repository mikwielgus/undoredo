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

use crate::{
    FlushDelta,
    undoredo::{Extract, Revert},
};

/// A reversible set of changes to a container.
///
/// Consists of a container of removed elements and another container of
/// inserted elements.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl<DC: Clone, T: ApplyDelta<DC>> Revert<T> for Delta<DC> {
    fn revert(self, target: &mut T) -> Self {
        let reverse = self.reverse();
        target.apply_delta(reverse.clone());

        reverse
    }
}

impl<DC, T: FlushDelta<DC>> Extract<T> for Delta<DC> {
    fn extract(target: &mut T) -> Self {
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

/// Half-delta for `Vec<V>`. Alias for `BTreeMap<usize, V>`.
pub type VecHalfDelta<V> = BTreeMap<usize, V>;
/// Delta for `Vec<V>`. Alias for `Delta<VecHalfDelta<V>>`.
pub type VecDelta<V> = Delta<VecHalfDelta<V>>;

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

/// Half-delta for `BTreeMap<K, V>`. Alias for `BTreeMap<K, V>`.
pub type BTreeMapHalfDelta<K, V> = BTreeMap<K, V>;
/// Delta for `BTreeMap<K, V>`. Alias for `Delta<BTreeMapHalfDelta<K, V>>`.
pub type BTreeMapDelta<K, V> = Delta<BTreeMapHalfDelta<K, V>>;

impl<K: Ord, V, DC: IntoIter<K> + Container<Key = K, Value = V>> ApplyDelta<DC> for BTreeMap<K, V> {
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

/// Half-delta for `BTreeSet<K>`. Alias for `BTreeMap<K, ()>`.
pub type BTreeSetHalfDelta<K> = BTreeMap<K, ()>;
/// Delta for `BTreeSet<K>`. Alias for `Delta<BTreeSetHalfDelta<K>>`.
pub type BTreeSetDelta<K> = Delta<BTreeSetHalfDelta<K>>;

impl<K: Ord, DC: IntoIter<K> + Container<Key = K, Value = ()>> ApplyDelta<DC> for BTreeSet<K> {
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

macro_rules! impl_delta_for_scalar {
    ($($half_delta:ident, $delta:ident, $t:ty);+ $(;)?) => {
        $(
            #[doc = concat!(
                "Half-delta for `", stringify!($t), "`. ",
                "Alias for `BTreeMap<usize, ", stringify!($t), ">`.",
            )]
            pub type $half_delta = BTreeMap<usize, $t>;
            #[doc = concat!(
                "Delta for `", stringify!($t), "`. ",
                "Alias for `Delta<", stringify!($half_delta), ">`.",
            )]
            pub type $delta = Delta<$half_delta>;

            impl ApplyDelta<$half_delta> for $t {
                fn apply_delta(&mut self, delta: $delta) {
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
        #[doc = concat!(
            "Half-delta for `(",
            $( stringify!($typ), ", ", )+
            ")`. ",
            "Alias for `BTreeMap<usize, (",
            $( stringify!($typ), ", ", )+
            ")>`.",
        )]
        pub type $half_delta<$($typ),+> = BTreeMap<usize, ($($typ,)+)>;
        #[doc = concat!(
            "Delta for `(",
            $( stringify!($typ), ", ", )+
            ")`. ",
            "Alias for `Delta<", stringify!($half_delta), "<",
            $( stringify!($typ), ", ", )+
            ">>`.",
        )]
        pub type $delta<$($typ),+> = Delta<$half_delta<$($typ),+>>;

        impl<$($typ,)+> ApplyDelta<$half_delta<$($typ),+>> for ($($typ,)+) {
            fn apply_delta(&mut self, delta: $delta<$($typ),+>) {
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

/// Half-delta for `PhantomData<V>`. Alias for `BTreeMap<usize, ()>`.
pub type PhantomDataHalfDelta = BTreeMap<usize, ()>;
/// Delta for `PhantomData<V>`. Alias for `Delta<PhantomDataHalfDelta>`.
pub type PhantomDataDelta = Delta<PhantomDataHalfDelta>;

impl<V, DC> ApplyDelta<DC> for PhantomData<V> {
    fn apply_delta(&mut self, _delta: Delta<DC>) {
        // Nothing happens here, obviously.
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// Half-delta for `HashMap<K, V>`. Alias for `HashMap<K, V>`.
pub type HashMapHalfDelta<K, V> = HashMap<K, V>;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// Delta for `HashMap<K, V>`. Alias for `Delta<HashMapHalfDelta<K, V>>`.
pub type HashMapDelta<K, V> = Delta<HashMapHalfDelta<K, V>>;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<K: Eq + Hash, V, DC: IntoIter<K> + Container<Key = K, Value = V>> ApplyDelta<DC>
    for HashMap<K, V>
{
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// Half-delta for `HashSet<K>`. Alias for `HashMap<K, ()>`.
pub type HashSetHalfDelta<K> = HashMap<K, ()>;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// Delta for `HashSet<K>`. Alias for `Delta<HashSetHalfDelta<K>>`.
pub type HashSetDelta<K> = Delta<HashSetHalfDelta<K>>;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<K: Eq + Hash, DC: IntoIter<K> + Container<Key = K, Value = ()>> ApplyDelta<DC> for HashSet<K> {
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "stable-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "stable-vec")))]
/// Half-delta for `StableVecFacade<V, C>`. Alias for `BTreeMap<usize, V>`.
pub type StableVecHalfDelta<V> = BTreeMap<usize, V>;
#[cfg(feature = "stable-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "stable-vec")))]
/// Delta for `StableVecFacade<V, C>`. Alias for `Delta<StableVecHalfDelta<V>>`.
pub type StableVecDelta<V> = Delta<StableVecHalfDelta<V>>;

#[cfg(feature = "stable-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "stable-vec")))]
impl<V, C: stable_vec::core::Core<V>, DC: IntoIter<usize> + Container<Key = usize, Value = V>>
    ApplyDelta<DC> for StableVecFacade<V, C>
{
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "thunderdome")]
#[cfg_attr(docsrs, doc(cfg(feature = "thunderdome")))]
/// Half-delta for `Arena<V>`. Alias for `BTreeMap<Index, V>`.
pub type ThunderdomeHalfDelta<V> = BTreeMap<Index, V>;
#[cfg(feature = "thunderdome")]
#[cfg_attr(docsrs, doc(cfg(feature = "thunderdome")))]
/// Delta for `Arena<V>`. Alias for `Delta<ThunderdomeHalfDelta<V>>`.
pub type ThunderdomeDelta<V> = Delta<ThunderdomeHalfDelta<V>>;

#[cfg(feature = "thunderdome")]
#[cfg_attr(docsrs, doc(cfg(feature = "thunderdome")))]
impl<V, DC: IntoIter<Index> + Container<Key = Index, Value = V>> ApplyDelta<DC> for Arena<V> {
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "rstar")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstar")))]
/// Half-delta for `RTree<K>`. Alias for `BTreeMap<K, ()>`.
pub type RTreeHalfDelta<K> = BTreeMap<K, ()>;

#[cfg(feature = "rstar")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstar")))]
/// Delta for `RTree<K>`. Alias for `Delta<RTreeHalfDelta<K>>`.
pub type RTreeDelta<K> = Delta<RTreeHalfDelta<K>>;

#[cfg(feature = "rstar")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstar")))]
impl<K: RTreeObject + PartialEq, DC: IntoIter<K> + Container<Key = K, Value = ()>> ApplyDelta<DC>
    for RTree<K>
{
    fn apply_delta(&mut self, delta: Delta<DC>) {
        apply_delta_on_map(self, delta);
    }
}

#[cfg(feature = "rstared")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstared")))]
/// Half-delta for `RTreed<C>`. Alias for `BTreeMap<K, V>`.
pub type RTreedHalfDelta<K, V> = BTreeMap<K, V>;

#[cfg(feature = "rstared")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstared")))]
/// Delta for `RTreed<C>`. Alias for `Delta<RTreedHalfDelta<K, V>>`.
pub type RTreedDelta<K, V> = Delta<RTreedHalfDelta<K, V>>;

#[cfg(feature = "rstared")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstared")))]
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
