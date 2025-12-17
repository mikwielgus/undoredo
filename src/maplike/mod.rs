// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait Map {
    type Item;
}

pub trait Keyed {
    type Key;
}

pub trait Get<K>: Map {
    fn get(&self, key: &K) -> Option<&Self::Item>;
}

pub trait Insert<K>: Map {
    fn insert(&mut self, key: K, value: Self::Item);
}

pub trait Remove<K>: Map {
    fn remove(&mut self, key: &K) -> Option<Self::Item>;
}

pub trait Push<K>: Map {
    fn push(&mut self, value: Self::Item) -> K;
}

pub trait IntoIter<K>: Map + Keyed {
    type IntoIter: Iterator<Item = (K, Self::Item)>;

    fn into_iter(self) -> Self::IntoIter;
}

#[cfg(feature = "std")]
mod std;

// No feature for alloc because it would be always enabled anyway.
mod alloc;

#[cfg(feature = "stable-vec")]
mod stable_vec;

#[cfg(feature = "thunderdome")]
mod thunderdome;

#[cfg(feature = "rstar")]
mod rstar;
