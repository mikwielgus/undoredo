// SPDX-FileCopyrightText: 2025 undoredo Developers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/*#![doc(html_root_url = "https://docs.rs/undoredo")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]*/
//#![cfg_attr(not(feature = "std"), no_std)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

mod edit;
mod impls;
mod map;
mod recorder;
mod undoredo;

pub use crate::edit::Edit;
pub use crate::map::{MapGet, MapInsert, MapPush, MapRemove};
pub use crate::recorder::Recorder;
pub use crate::undoredo::UndoRedo;
