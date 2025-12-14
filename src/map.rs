// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait Map {
    type Item;
}

pub trait Keyed {
    type Key;
}

pub trait MapGet<K>: Map {
    fn get(&self, key: &K) -> Option<&Self::Item>;
}

pub trait MapInsert<K>: Map {
    fn insert(&mut self, key: K, value: Self::Item);
}

pub trait MapRemove<K>: Map {
    fn remove(&mut self, key: &K) -> Option<Self::Item>;
}

pub trait MapPush<K>: Map {
    fn push(&mut self, value: Self::Item) -> K;
}

/*pub trait Iter<K>: Collection + Keyed {
    type Iter<'a>: Iterator<Item = (&'a K, &'a Self::Item)>
    where
        Self: 'a,
        K: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}*/

pub trait MapIntoIter<K>: Map + Keyed {
    type IntoIter: Iterator<Item = (K, Self::Item)>;

    fn into_iter(self) -> Self::IntoIter;
}
