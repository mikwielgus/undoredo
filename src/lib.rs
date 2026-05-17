// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/undoredo")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

// No feature for `alloc` because it would be always enabled anyway.
pub extern crate alloc;

mod delta;
mod recorder;
mod snapshot;
mod undoredo;

pub use crate::delta::*;
pub use crate::recorder::{FlushDelta, Recorder};
pub use crate::snapshot::Snapshot;
pub use crate::undoredo::{CmdEdit, Extract, Revert, UndoRedo};
pub use maplike::{
    Container, Get, Insert, IntoIter, Maplike, Push, Remove, Scalarlike, Setlike, Veclike,
};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::ApplyDelta;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::Delta;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::FlushDelta;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::HalfDelta;
