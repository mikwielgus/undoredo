// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec::Vec;
use maplike::{Container, Get};

use crate::{Delta, Recorder, delta::ApplyDelta};

/// A delta along with metadata.
///
/// The metadata usually somehow represents the command that originated the
/// delta, but it really can be anything, as it is only for the convenience of
/// the programmer using the library, without any effect on logic.
pub struct CmdDelta<Cmd, DC> {
    pub cmd: Cmd,
    pub delta: Delta<DC>,
}

/// An undo-redo bistack.
pub struct UndoRedo<DC, Cmd = ()> {
    done: Vec<CmdDelta<Cmd, DC>>,
    undone: Vec<CmdDelta<Cmd, DC>>,
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
    /// redone) deltas.
    pub fn done(&self) -> &[CmdDelta<Cmd, DC>] {
        &self.done
    }

    /// Returns a slice of the *undone* stack, which contains all the undone
    /// deltas.
    pub fn undone(&self) -> &[CmdDelta<Cmd, DC>] {
        &self.undone
    }
}

impl<Cmd: Default, DC> UndoRedo<DC, Cmd> {
    /// Push the delta onto the done stack.
    ///
    /// Clears the undone stack.
    pub fn commit(&mut self, delta: Delta<DC>) {
        self.cmd_commit(Default::default(), delta);
    }
}

impl<Cmd, DC> UndoRedo<DC, Cmd> {
    /// Push the delta onto the done stack along with metadata.
    pub fn cmd_commit(&mut self, cmd: Cmd, delta: Delta<DC>) {
        self.done.push(CmdDelta { cmd, delta });
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
    /// The undone delta is popped from the done stack, reversed, reverted, and
    /// pushed onto the undone stack.
    pub fn undo(&mut self, target: &mut impl ApplyDelta<DC>) -> Option<Cmd> {
        let CmdDelta { cmd, delta } = self.done.pop()?;
        let reverse_delta = delta.reverse();

        target.apply_delta(&reverse_delta);
        self.undone.push(CmdDelta {
            cmd: cmd.clone(),
            delta: reverse_delta,
        });

        Some(cmd)
    }

    /// Redo the last undone delta.
    ///
    /// The redone delta is popped from the undone stack, reversed, reverted, and
    /// pushed back onto the done stack.
    pub fn redo(&mut self, target: &mut impl ApplyDelta<DC>) -> Option<Cmd> {
        let CmdDelta { cmd, delta } = self.undone.pop()?;
        let reverse_delta = delta.reverse();

        target.apply_delta(&reverse_delta);
        self.done.push(CmdDelta {
            cmd: cmd.clone(),
            delta: reverse_delta,
        });

        Some(cmd)
    }
}
