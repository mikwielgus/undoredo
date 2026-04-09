// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec::Vec;
use maplike::{Container, Get};

use crate::{Delta, FlushDelta, Recorder, delta::ApplyDelta};

pub trait Revert<T> {
    fn revert(self, target: &mut T) -> Self;
}

pub trait Extract<T> {
    fn extract(target: &mut T) -> Self;
}

/// An edit along with metadata.
///
/// The metadata usually somehow represents the command that originated the
/// edit, but it really can be anything, as it is only for the convenience of
/// the programmer using the library, without any effect on logic.
pub struct CmdEdit<Cmd, E> {
    /// Command or other metadata associated with this edit.
    pub cmd: Cmd,
    /// The recorded change.
    pub edit: E,
}

/// An undo-redo bistack.
pub struct UndoRedo<DC, Cmd = ()> {
    done: Vec<CmdEdit<Cmd, Delta<DC>>>,
    undone: Vec<CmdEdit<Cmd, Delta<DC>>>,
}

impl<Cmd, DC> UndoRedo<DC, Cmd> {
    /// Create a new empty undo-redo bistack.
    pub fn new() -> Self {
        Self {
            done: Vec::new(),
            undone: Vec::new(),
        }
    }

    /// Returns a slice of the *done* stack, which contains all the done (or
    /// redone) edits.
    pub fn done(&self) -> &[CmdEdit<Cmd, Delta<DC>>] {
        &self.done
    }

    /// Returns a slice of the *undone* stack, which contains all the undone
    /// edits.
    pub fn undone(&self) -> &[CmdEdit<Cmd, Delta<DC>>] {
        &self.undone
    }
}

impl<Cmd: Default, DC> UndoRedo<DC, Cmd> {
    /// Flush and push changes onto the *done* stack.
    ///
    /// Clears the undone stack.
    pub fn commit(&mut self, target: &mut impl FlushDelta<DC>) {
        self.cmd_commit(Default::default(), Extract::extract(target));
    }
}

impl<Cmd, DC> UndoRedo<DC, Cmd> {
    /// Flush and push changes onto the *done* stack.
    ///
    /// Clears the undone stack.
    pub fn cmd_commit(&mut self, cmd: Cmd, delta: Delta<DC>) {
        self.done.push(CmdEdit { cmd, edit: delta });
        self.undone.clear();
    }
}

impl<Cmd, DC: Container + Default> UndoRedo<DC, Cmd> {
    /// Make and record changes to the recorded container from within a
    /// closure, automatically committing them once closure finishes.
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

impl<Cmd: Clone, DC: Clone> UndoRedo<DC, Cmd> {
    /// Undo the last done delta.
    ///
    /// The undone delta is popped from the *done* stack, reversed, reverted,
    /// and pushed onto the *undone* stack.
    pub fn undo(&mut self, target: &mut impl ApplyDelta<DC>) -> Option<Cmd> {
        let CmdEdit { cmd, edit } = self.done.pop()?;
        self.undone.push(CmdEdit {
            cmd: cmd.clone(),
            edit: edit.revert(target),
        });

        Some(cmd)
    }

    /// Redo the last undone delta.
    ///
    /// The redone delta is popped from the *undone* stack, reversed, reverted,
    /// and pushed back onto the *done* stack.
    pub fn redo(&mut self, target: &mut impl ApplyDelta<DC>) -> Option<Cmd> {
        let CmdEdit { cmd, edit } = self.undone.pop()?;
        self.done.push(CmdEdit {
            cmd: cmd.clone(),
            edit: edit.revert(target),
        });

        Some(cmd)
    }
}
