// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec::Vec;
use maplike::{Get, Insert, Keyed, Map};

use crate::{Edit, Recorder, edit::ApplyEdit};

/// An edit along with metadata.
///
/// The metadata usually somehow represents the command that originated the
/// edit, but it really can be anything, as it is only for the convenience of
/// the programmer using the library and has no effect on logic.
pub struct CmdEdit<Cmd, EC> {
    pub cmd: Cmd,
    pub edit: Edit<EC>,
}

/// An undo-redo bistack.
pub struct UndoRedo<EC, Cmd = ()> {
    done: Vec<CmdEdit<Cmd, EC>>,
    undone: Vec<CmdEdit<Cmd, EC>>,
}

impl<Cmd, EC> UndoRedo<EC, Cmd> {
    /// Create a new empty undo-redo bistack.
    pub fn new() -> Self {
        Self {
            done: Vec::new(),
            undone: Vec::new(),
        }
    }

    /// Returns a slice of the done stack, which contains all the done (or
    /// redone) edits.
    pub fn done(&self) -> &[CmdEdit<Cmd, EC>] {
        &self.done
    }

    /// Returns a slice of the undone stack, which contains all the undone
    /// edits.
    pub fn undone(&self) -> &[CmdEdit<Cmd, EC>] {
        &self.undone
    }
}

impl<Cmd: Default, EC> UndoRedo<EC, Cmd> {
    /// Push the edit onto the done stack.
    ///
    /// Clears the undone stack.
    pub fn commit(&mut self, edit: Edit<EC>) {
        self.cmd_commit(Default::default(), edit);
    }
}

impl<Cmd, EC> UndoRedo<EC, Cmd> {
    /// Push the edit onto the done stack along with metadata.
    pub fn cmd_commit(&mut self, cmd: Cmd, edit: Edit<EC>) {
        self.done.push(CmdEdit { cmd, edit });
        self.undone.clear();
    }
}

impl<Cmd, EC: Keyed + Map + Default> UndoRedo<EC, Cmd> {
    /// Make and record changes to the recorded collection from within a
    /// closure, automatically committing them once closure finishes.
    pub fn edit<
        C: Keyed<Key = EC::Key> + Map<Item = EC::Item> + Get<C::Key> + Insert<C::Key>,
        F: FnOnce(&mut Recorder<C, EC>) -> Cmd,
    >(
        &mut self,
        collection: C,
        f: F,
    ) -> C
    where
        C::Key: Clone,
        C::Item: Clone,
    {
        let mut recorder = Recorder::<C, EC>::new(collection);
        let cmd = f(&mut recorder);
        let (container, edit) = recorder.dissolve();

        self.cmd_commit(cmd, edit);

        container
    }
}

impl<Cmd: Clone, EC: Clone> UndoRedo<EC, Cmd> {
    /// Undo the last done edit.
    ///
    /// The undone edit is popped from the done stack, reversed, reverted, and
    /// pushed onto the undone stack.
    pub fn undo(&mut self, target: &mut impl ApplyEdit<EC>) -> Option<Cmd> {
        let CmdEdit { cmd, edit } = self.done.pop()?;
        let reverse_edit = edit.reverse();

        target.apply_edit(&reverse_edit);
        self.undone.push(CmdEdit {
            cmd: cmd.clone(),
            edit: reverse_edit,
        });

        Some(cmd)
    }

    /// Redo the last undone edit.
    ///
    /// The redone edit is popped from the undone stack, reversed, reverted, and
    /// pushed back onto the done stack.
    pub fn redo(&mut self, target: &mut impl ApplyEdit<EC>) -> Option<Cmd> {
        let CmdEdit { cmd, edit } = self.undone.pop()?;
        let reverse_edit = edit.reverse();

        target.apply_edit(&reverse_edit);
        self.done.push(CmdEdit {
            cmd: cmd.clone(),
            edit: reverse_edit,
        });

        Some(cmd)
    }
}
