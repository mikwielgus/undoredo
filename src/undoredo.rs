// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec::Vec;
use maplike::{Container, Get};

use crate::{Delta, Recorder};

/// Revert an edit and return an edit that reverts the revert.
pub trait Revert<T> {
    /// Revert an edit and return an edit that reverts the revert.
    fn revert(self, target: &mut T) -> Self;
}

impl<T> Revert<T> for () {
    fn revert(self, _target: &mut T) -> Self {
        ()
    }
}

/// Extract an edit corresponding to the current state.
///
/// In delta-based undo-redo, this flushes the delta. In snapshot-based
/// undo-redo, this clones the current state into a snapshot.
pub trait Extract<T> {
    /// Extract an edit corresponding to the current state.
    ///
    /// In delta-based undo-redo, this flushes the delta. In snapshot-based
    /// undo-redo, this clones the current state into a snapshot.
    fn extract(target: &mut T) -> Self;
}

impl<T> Extract<T> for () {
    fn extract(_target: &mut T) -> Self {
        ()
    }
}

/// An edit along with metadata.
///
/// The metadata usually somehow represents the command that originated the
/// edit, but it really can be anything, as it is only for the convenience of
/// the programmer using the library, without any effect on logic.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CmdEdit<Cmd, E> {
    /// Command or other metadata associated with this edit.
    pub cmd: Cmd,
    /// The recorded change (for example a flushed diff from a [`Recorder`](crate::Recorder)).
    pub edit: E,
}

/// An undo-redo bistack.
///
/// `E` is the type of each committed edit (for example, the concrete diff type
/// your recorder or domain uses).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UndoRedo<E, Cmd = ()> {
    done: Vec<CmdEdit<Cmd, E>>,
    undone: Vec<CmdEdit<Cmd, E>>,
}

impl<Cmd, E> UndoRedo<E, Cmd> {
    /// Create a new empty undo-redo bistack.
    pub fn new() -> Self {
        Self {
            done: Vec::new(),
            undone: Vec::new(),
        }
    }

    /// Returns a slice of the *done* stack, which contains all the done (or
    /// redone) edits.
    pub fn done(&self) -> &[CmdEdit<Cmd, E>] {
        &self.done
    }

    /// Returns a slice of the *undone* stack, which contains all the undone
    /// edits.
    pub fn undone(&self) -> &[CmdEdit<Cmd, E>] {
        &self.undone
    }
}

impl<Cmd: Default, E> UndoRedo<E, Cmd> {
    /// Flush and push changes onto the *done* stack.
    ///
    /// Clears the undone stack.
    pub fn commit<T>(&mut self, target: &mut T)
    where
        E: Extract<T>,
    {
        self.cmd_commit(Default::default(), E::extract(target));
    }
}

impl<Cmd, E> UndoRedo<E, Cmd> {
    /// Flush and push changes onto the *done* stack along with additional
    /// metadata ("cmd").
    ///
    /// Clears the undone stack.
    pub fn cmd_commit(&mut self, cmd: Cmd, edit: E) {
        self.done.push(CmdEdit { cmd, edit });
        self.undone.clear();
    }
}

impl<Cmd> UndoRedo<(), Cmd> {
    /// Push command onto the *done* stack without any edit (delta or snapshot).
    ///
    /// This is convenience interface for [Command
    /// pattern](https://en.wikipedia.org/wiki/Command_pattern), equivalent to
    /// calling [`cmd_commit()`] with the `command` as `cmd` and `()` as `edit`.
    pub fn command(&mut self, command: Cmd) {
        self.cmd_commit(command, ());
    }
}

impl<Cmd: Clone, E: Clone> UndoRedo<E, Cmd> {
    /// Undo the last done edit.
    ///
    /// The undone edit is popped from the *done* stack, reversed, reverted,
    /// and pushed onto the *undone* stack.
    pub fn undo<T>(&mut self, target: &mut T) -> Option<Cmd>
    where
        E: Revert<T>,
    {
        let CmdEdit { cmd, edit } = self.done.pop()?;
        self.undone.push(CmdEdit {
            cmd: cmd.clone(),
            edit: edit.revert(target),
        });

        Some(cmd)
    }

    /// Redo the last undone edit.
    ///
    /// The redone edit is popped from the *undone* stack, reversed, reverted,
    /// and pushed back onto the *done* stack.
    pub fn redo<T>(&mut self, target: &mut T) -> Option<Cmd>
    where
        E: Revert<T>,
    {
        let CmdEdit { cmd, edit } = self.undone.pop()?;
        self.done.push(CmdEdit {
            cmd: cmd.clone(),
            edit: edit.revert(target),
        });

        Some(cmd)
    }
}

impl<Cmd, DC: Container + Default> UndoRedo<Delta<DC>, Cmd> {
    /// Make and record changes to the recorded container from within a closure,
    /// automatically committing them once the closure finishes.
    pub fn edit<
        K,
        V,
        C: Container<Key = K, Value = V> + Get<K>,
        F: FnOnce(&mut Recorder<C, DC>) -> Cmd,
    >(
        &mut self,
        container: C,
        f: F,
    ) -> C
    where
        DC: Container<Key = K, Value = V>,
        K: Clone,
        V: Clone,
    {
        let mut recorder = Recorder::<C, DC>::new(container);
        let cmd = f(&mut recorder);
        let (container, delta) = recorder.dissolve();

        self.cmd_commit(cmd, delta);

        container
    }
}
