// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/*#![doc(html_root_url = "https://docs.rs/undoredo")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]*/
//#![cfg_attr(not(feature = "std"), no_std)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

// No feature for alloc because it would be always enabled anyway.
extern crate alloc;

mod edit;
mod maplike;
mod recorder;
mod undoredo;

pub use crate::edit::Edit;
pub use crate::edit::ApplyEdit;
pub use crate::maplike::{Get, Insert, IntoIter, Push, Remove};
pub use crate::recorder::Recorder;
pub use crate::undoredo::UndoRedo;
