/*#![doc(html_root_url = "https://docs.rs/undoredo")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]*/
#![cfg_attr(not(feature = "std"), no_std)]

mod collection;
mod edit;
mod impls;
mod recorder;
mod undoredo;

pub use crate::collection::{Get, Insert, Push, Remove};
pub use crate::edit::Edit;
pub use crate::recorder::Recorder;
pub use crate::undoredo::UndoRedo;
