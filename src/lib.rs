// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/undoredo")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, doc = "\n## Feature flags\n")]
#![cfg_attr(docsrs, doc = document_features::document_features!())]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

// No feature for `alloc` because it would be always enabled anyway.
pub extern crate alloc;

/// Type aliases for full and half deltas.
pub mod aliases;

mod delta;
mod recorder;
mod snapshot;
mod undoredo;

pub use crate::delta::*;
pub use crate::recorder::{ExtendDelta, FlushDelta, Recorder, ResetDelta};
pub use crate::snapshot::Snapshot;
pub use crate::undoredo::{CmdEdit, ExtractEdit, RevertEdit, UndoRedo};
pub use maplike;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::ApplyDelta;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::Delta;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::ExtendDelta;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::FlushDelta;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::HalfDelta;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::MergeDeltas;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use undoredo_derive::ResetDelta;
