// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(feature = "thunderdome")]
use thunderdome::Index;

use crate::Delta;

/// Half-delta for `Vec<V>`. Alias for `BTreeMap<usize, V>`.
pub type VecHalfDelta<V> = BTreeMap<usize, V>;
/// Delta for `Vec<V>`. Alias for `Delta<VecHalfDelta<V>>`.
pub type VecDelta<V> = Delta<VecHalfDelta<V>>;

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

/// Half-delta for `PhantomData<V>`. Alias for `BTreeMap<usize, ()>`.
pub type PhantomDataHalfDelta = BTreeMap<usize, ()>;
/// Delta for `PhantomData<V>`. Alias for `Delta<PhantomDataHalfDelta>`.
pub type PhantomDataDelta = Delta<PhantomDataHalfDelta>;

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
/// Half-delta for `HashSet<K>`. Alias for `HashMap<K, ()>`.
pub type HashSetHalfDelta<K> = HashMap<K, ()>;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// Delta for `HashSet<K>`. Alias for `Delta<HashSetHalfDelta<K>>`.
pub type HashSetDelta<K> = Delta<HashSetHalfDelta<K>>;

#[cfg(feature = "stable-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "stable-vec")))]
/// Half-delta for `StableVec<V>`. Alias for `BTreeMap<usize, V>`.
pub type StableVecHalfDelta<V> = BTreeMap<usize, V>;
#[cfg(feature = "stable-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "stable-vec")))]
/// Delta for `StableVec<V>`. Alias for `Delta<StableVecHalfDelta<V>>`.
pub type StableVecDelta<V> = Delta<StableVecHalfDelta<V>>;

#[cfg(feature = "thunderdome")]
#[cfg_attr(docsrs, doc(cfg(feature = "thunderdome")))]
/// Half-delta for `Arena<V>`. Alias for `BTreeMap<Index, V>`.
pub type ArenaHalfDelta<V> = BTreeMap<Index, V>;
#[cfg(feature = "thunderdome")]
#[cfg_attr(docsrs, doc(cfg(feature = "thunderdome")))]
/// Delta for `Arena<V>`. Alias for `Delta<ArenaHalfDelta<V>>`.
pub type ArenaDelta<V> = Delta<ArenaHalfDelta<V>>;

#[cfg(feature = "rstar")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstar")))]
/// Half-delta for `RTree<K>`. Alias for `BTreeMap<K, ()>`.
pub type RTreeHalfDelta<K> = BTreeMap<K, ()>;
#[cfg(feature = "rstar")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstar")))]
/// Delta for `RTree<K>`. Alias for `Delta<RTreeHalfDelta<K>>`.
pub type RTreeDelta<K> = Delta<RTreeHalfDelta<K>>;

#[cfg(feature = "rstared")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstared")))]
/// Half-delta for `RTreed<C>`. Alias for `BTreeMap<K, V>`.
pub type RTreedHalfDelta<K, V> = BTreeMap<K, V>;
#[cfg(feature = "rstared")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstared")))]
/// Delta for `RTreed<C>`. Alias for `Delta<RTreedHalfDelta<K, V>>`.
pub type RTreedDelta<K, V> = Delta<RTreedHalfDelta<K, V>>;
