// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/undoredo")]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

// No feature for `alloc` because it would be always enabled anyway.
extern crate alloc;

mod delta;
mod recorder;
mod undoredo;

pub use crate::delta::{ApplyDelta, Delta};
pub use crate::recorder::{FlushDelta, Recorder, RecorderDeltaCollection};
pub use crate::undoredo::UndoRedo;
pub use maplike::{Get, Insert, IntoIter, KeyedCollection, Map, Push, Remove, StableRemove};

#[cfg(feature = "derive")]
pub use undoredo_derive::{ApplyDelta, CompositeDelta, FlushDelta};
