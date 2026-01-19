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

mod edit;
mod recorder;
mod undoredo;

pub use crate::edit::ApplyEdit;
pub use crate::edit::Edit;
pub use crate::recorder::Recorder;
pub use crate::undoredo::UndoRedo;
pub use maplike::{Get, Insert, IntoIter, Keyed, Map, Push, Remove, StableRemove};
