// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Apply an edit.
pub trait ApplyEdit<T> {
    /// Apply an edit.
    fn apply_edit(self, target: &mut T);
}

impl<T> ApplyEdit<T> for () {
    fn apply_edit(self, _target: &mut T) {
        // Nothing happens, obviously.
    }
}

/// Revert an edit and return an edit that reverts the revert.
///
/// If an edit has a reverse, this should be equivalent to applying it.
pub trait RevertEdit<T> {
    /// Revert an edit and return an edit that reverts the revert.
    ///
    /// If an edit has a reverse, this should be equivalent to applying it.
    fn revert_edit(self, target: &mut T) -> Self;
}

impl<T> RevertEdit<T> for () {
    fn revert_edit(self, _target: &mut T) -> Self {
        // Nothing happens, obviously.
    }
}

/// Extract an edit corresponding to the current state.
///
/// In delta-based undo-redo, this flushes the delta. In snapshot-based
/// undo-redo, this clones the current state into a snapshot.
pub trait ExtractEdit<T> {
    /// Extract an edit corresponding to the current state.
    ///
    /// In delta-based undo-redo, this flushes the delta. In snapshot-based
    /// undo-redo, this clones the current state into a snapshot.
    fn extract_edit(target: &mut T) -> Self;
}

impl<T> ExtractEdit<T> for () {
    fn extract_edit(_target: &mut T) -> Self {
        ()
    }
}

/// An edit along with metadata.
///
/// The metadata usually somehow represents the command that originated the
/// edit, but it really can be anything, as it is only for the convenience of
/// the programmer using the library, without any effect on logic.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct CmdEdit<Cmd, E> {
    /// Command or other metadata associated with this edit.
    pub cmd: Cmd,
    /// The recorded change (for example a flushed diff from a [`Recorder`](crate::Recorder)).
    pub edit: E,
}
