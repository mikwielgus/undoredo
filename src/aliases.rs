// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(feature = "indexmap")]
use indexmap::IndexMap;

#[cfg(feature = "thunderdome")]
use thunderdome::Index;

use crate::Delta;

/// Half-delta for `Vec<V>`. Alias for `BTreeMap<usize, V>`.
pub type VecHalfDelta<V> = BTreeMap<usize, V>;
/// Delta for `Vec<V>`. Alias for `Delta<VecHalfDelta<V>>`.
pub type VecDelta<V> = Delta<VecHalfDelta<V>>;

/// Half-delta for `VecDeque<V>`. Alias for `BTreeMap<usize, V>`.
pub type VecDequeHalfDelta<V> = BTreeMap<usize, V>;
/// Delta for `VecDeque<V>`. Alias for `Delta<VecDequeHalfDelta<V>>`.
pub type VecDequeDelta<V> = Delta<VecDequeHalfDelta<V>>;

/// Half-delta for `BTreeMap<K, V>`. Alias for `BTreeMap<K, V>`.
pub type BTreeMapHalfDelta<K, V> = BTreeMap<K, V>;
/// Delta for `BTreeMap<K, V>`. Alias for `Delta<BTreeMapHalfDelta<K, V>>`.
pub type BTreeMapDelta<K, V> = Delta<BTreeMapHalfDelta<K, V>>;

/// Half-delta for `BTreeSet<K>`. Alias for `BTreeMap<K, ()>`.
pub type BTreeSetHalfDelta<K> = BTreeMap<K, ()>;
/// Delta for `BTreeSet<K>`. Alias for `Delta<BTreeSetHalfDelta<K>>`.
pub type BTreeSetDelta<K> = Delta<BTreeSetHalfDelta<K>>;

macro_rules! declare_scalar_aliases {
    ($(($half_delta:ident, $delta:ident, $t:ty, $t_name:expr));+ $(;)?) => {
        $(
            #[doc = concat!(
                "Half-delta for `",
                $t_name,
                "`. Alias for `BTreeMap<usize, ",
                $t_name,
                ">`.",
            )]
            pub type $half_delta = BTreeMap<usize, $t>;
            #[doc = concat!(
                "Delta for `",
                $t_name,
                "`. Alias for `Delta<",
                stringify!($half_delta),
                ">`.",
            )]
            pub type $delta = Delta<$half_delta>;
        )+
    };
}

declare_scalar_aliases! {
    (I8HalfDelta, I8Delta, i8, "i8");
    (I16HalfDelta, I16Delta, i16, "i16");
    (I32HalfDelta, I32Delta, i32, "i32");
    (I64HalfDelta, I64Delta, i64, "i64");
    (I128HalfDelta, I128Delta, i128, "i128");
    (IsizeHalfDelta, IsizeDelta, isize, "isize");
    (U8HalfDelta, U8Delta, u8, "u8");
    (U16HalfDelta, U16Delta, u16, "u16");
    (U32HalfDelta, U32Delta, u32, "u32");
    (U64HalfDelta, U64Delta, u64, "u64");
    (U128HalfDelta, U128Delta, u128, "u128");
    (UsizeHalfDelta, UsizeDelta, usize, "usize");
    (F32HalfDelta, F32Delta, f32, "f32");
    (F64HalfDelta, F64Delta, f64, "f64");
    (CharHalfDelta, CharDelta, char, "char");
    (BoolHalfDelta, BoolDelta, bool, "bool");
    (UnitHalfDelta, UnitDelta, (), "()");
}

macro_rules! declare_tuple_aliases {
    ($half_delta:ident, $delta:ident, $($typ:ident),+; $tuple:expr) => {
        #[doc = concat!(
            "Half-delta for `",
            $tuple,
            "`. Alias for `BTreeMap<usize, ",
            $tuple,
            ">`.",
        )]
        pub type $half_delta<$($typ),+> = BTreeMap<usize, ($($typ,)+)>;
        #[doc = concat!(
            "Delta for `",
            $tuple,
            "`. Alias for `Delta<",
            stringify!($half_delta),
            "<",
            $tuple,
            ">>`.",
        )]
        pub type $delta<$($typ),+> = Delta<$half_delta<$($typ),+>>;
    };
}

declare_tuple_aliases!(Tuple1HalfDelta, Tuple1Delta, T0; "(T0,)");
declare_tuple_aliases!(Tuple2HalfDelta, Tuple2Delta, T0, T1; "(T0, T1)");
declare_tuple_aliases!(Tuple3HalfDelta, Tuple3Delta, T0, T1, T2; "(T0, T1, T2)");
declare_tuple_aliases!(Tuple4HalfDelta, Tuple4Delta, T0, T1, T2, T3; "(T0, T1, T2, T3)");
declare_tuple_aliases!(
    Tuple5HalfDelta,
    Tuple5Delta,
    T0,
    T1,
    T2,
    T3,
    T4;
    "(T0, T1, T2, T3, T4)"
);
declare_tuple_aliases!(
    Tuple6HalfDelta,
    Tuple6Delta,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5;
    "(T0, T1, T2, T3, T4, T5)"
);
declare_tuple_aliases!(
    Tuple7HalfDelta,
    Tuple7Delta,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6;
    "(T0, T1, T2, T3, T4, T5, T6)"
);
declare_tuple_aliases!(
    Tuple8HalfDelta,
    Tuple8Delta,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7;
    "(T0, T1, T2, T3, T4, T5, T6, T7)"
);
declare_tuple_aliases!(
    Tuple9HalfDelta,
    Tuple9Delta,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8;
    "(T0, T1, T2, T3, T4, T5, T6, T7, T8)"
);
declare_tuple_aliases!(
    Tuple10HalfDelta,
    Tuple10Delta,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9;
    "(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)"
);
declare_tuple_aliases!(
    Tuple11HalfDelta,
    Tuple11Delta,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10;
    "(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)"
);
declare_tuple_aliases!(
    Tuple12HalfDelta,
    Tuple12Delta,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11;
    "(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)"
);

/// Half-delta for `Option<V>`. Alias for `BTreeMap<usize, V>`.
pub type OptionHalfDelta<V> = BTreeMap<usize, V>;
/// Delta for `Option<V>`. Alias for `Delta<OptionHalfDelta<V>>`.
pub type OptionDelta<V> = Delta<OptionHalfDelta<V>>;

/// Half-delta for `Box<V>`. Alias for `BTreeMap<usize, V>`.
pub type BoxHalfDelta<V> = BTreeMap<usize, V>;
/// Delta for `Box<V>`. Alias for `Delta<BoxHalfDelta<V>>`.
pub type BoxDelta<V> = Delta<BoxHalfDelta<V>>;

/// Half-delta for `Rc<V>`. Alias for `BTreeMap<usize, V>`.
pub type RcHalfDelta<V> = BTreeMap<usize, V>;
/// Delta for `Rc<V>`. Alias for `Delta<RcHalfDelta<V>>`.
pub type RcDelta<V> = Delta<RcHalfDelta<V>>;

#[cfg(feature = "std")]
/// Half-delta for `Arc<V>`. Alias for `BTreeMap<usize, V>`.
pub type ArcHalfDelta<V> = BTreeMap<usize, V>;
#[cfg(feature = "std")]
/// Delta for `Arc<V>`. Alias for `Delta<ArcHalfDelta<V>>`.
pub type ArcDelta<V> = Delta<ArcHalfDelta<V>>;

/// Half-delta for `PhantomData<V>`. Alias for `BTreeMap<usize, ()>`.
pub type PhantomDataHalfDelta = BTreeMap<usize, ()>;
/// Delta for `PhantomData<V>`. Alias for `Delta<PhantomDataHalfDelta>`.
pub type PhantomDataDelta = Delta<PhantomDataHalfDelta>;

#[cfg(feature = "thunderdome")]
/// Half-delta for `Arena<V>`. Alias for `BTreeMap<Index, V>`.
pub type ArenaHalfDelta<V> = BTreeMap<Index, V>;
#[cfg(feature = "thunderdome")]
/// Delta for `Arena<V>`. Alias for `Delta<ArenaHalfDelta<V>>`.
pub type ArenaDelta<V> = Delta<ArenaHalfDelta<V>>;

#[cfg(feature = "bidimap")]
/// Half-delta for `BiBTreeMap<L, R>`. Alias for `BTreeMap<L, R>`.
pub type BiBTreeMapHalfDelta<L, R> = BTreeMap<L, R>;
#[cfg(feature = "bidimap")]
/// Delta for `BiBTreeMap<L, R>`. Alias for `Delta<BiBTreeMapHalfDelta<L, R>>`.
pub type BiBTreeMapDelta<L, R> = Delta<BiBTreeMapHalfDelta<L, R>>;

#[cfg(all(feature = "bidimap", feature = "std"))]
/// Half-delta for `BiHashMap<L, R>`. Alias for `HashMap<L, R>`.
pub type BiHashMapHalfDelta<L, R> = HashMap<L, R>;
#[cfg(all(feature = "bidimap", feature = "std"))]
/// Delta for `BiHashMap<L, R>`. Alias for `Delta<BiHashMapHalfDelta<L, R>>`.
pub type BiHashMapDelta<L, R> = Delta<BiHashMapHalfDelta<L, R>>;

#[cfg(feature = "std")]
/// Half-delta for `HashMap<K, V>`. Alias for `HashMap<K, V>`.
pub type HashMapHalfDelta<K, V> = HashMap<K, V>;
#[cfg(feature = "std")]
/// Delta for `HashMap<K, V>`. Alias for `Delta<HashMapHalfDelta<K, V>>`.
pub type HashMapDelta<K, V> = Delta<HashMapHalfDelta<K, V>>;

#[cfg(feature = "std")]
/// Half-delta for `HashSet<K>`. Alias for `HashMap<K, ()>`.
pub type HashSetHalfDelta<K> = HashMap<K, ()>;
#[cfg(feature = "std")]
/// Delta for `HashSet<K>`. Alias for `Delta<HashSetHalfDelta<K>>`.
pub type HashSetDelta<K> = Delta<HashSetHalfDelta<K>>;

#[cfg(feature = "indexmap")]
/// Half-delta for `IndexMap<K, V>`. Alias for `IndexMap<K, V>`.
pub type IndexMapHalfDelta<K, V> = IndexMap<K, V>;
#[cfg(feature = "indexmap")]
/// Delta for `IndexMap<K, V>`. Alias for `Delta<IndexMapHalfDelta<K, V>>`.
pub type IndexMapDelta<K, V> = Delta<IndexMapHalfDelta<K, V>>;

#[cfg(feature = "indexmap")]
/// Half-delta for `IndexSet<K>`. Alias for `IndexMap<K, ()>`.
pub type IndexSetHalfDelta<K> = IndexMap<K, ()>;
#[cfg(feature = "indexmap")]
/// Delta for `IndexSet<K>`. Alias for `Delta<IndexSetHalfDelta<K>>`.
pub type IndexSetDelta<K> = Delta<IndexSetHalfDelta<K>>;

#[cfg(feature = "rstar")]
/// Half-delta for `RTree<K>`. Alias for `BTreeMap<K, ()>`.
pub type RTreeHalfDelta<K> = BTreeMap<K, ()>;
#[cfg(feature = "rstar")]
/// Delta for `RTree<K>`. Alias for `Delta<RTreeHalfDelta<K>>`.
pub type RTreeDelta<K> = Delta<RTreeHalfDelta<K>>;

#[cfg(feature = "stable-vec")]
/// Half-delta for `StableVec<V>`. Alias for `BTreeMap<usize, V>`.
pub type StableVecHalfDelta<V> = BTreeMap<usize, V>;
#[cfg(feature = "stable-vec")]
/// Delta for `StableVec<V>`. Alias for `Delta<StableVecHalfDelta<V>>`.
pub type StableVecDelta<V> = Delta<StableVecHalfDelta<V>>;

#[cfg(feature = "tinyvec")]
/// Half-delta for `ArrayVec<A>`. Alias for `BTreeMap<usize, A::Item>`.
pub type ArrayVecHalfDelta<A> = BTreeMap<usize, <A as tinyvec::Array>::Item>;
#[cfg(feature = "tinyvec")]
/// Delta for `ArrayVec<A>`. Alias for `Delta<ArrayVecHalfDelta<A>>`.
pub type ArrayVecDelta<A> = Delta<ArrayVecHalfDelta<A>>;

#[cfg(feature = "tinyvec")]
/// Half-delta for `TinyVec<A>`. Alias for `BTreeMap<usize, A::Item>`.
pub type TinyVecHalfDelta<A> = BTreeMap<usize, <A as tinyvec::Array>::Item>;
#[cfg(feature = "tinyvec")]
/// Delta for `TinyVec<A>`. Alias for `Delta<TinyVecHalfDelta<A>>`.
pub type TinyVecDelta<A> = Delta<TinyVecHalfDelta<A>>;
