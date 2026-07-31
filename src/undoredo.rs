// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec::Vec;
use maplike::containers::Container;
use maplike::ops::Get;

use crate::{CmdEdit, Delta, ExtractEdit, Recorder, RevertEdit};

/// An history bistack for linear undo-redo action.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct UndoRedo<E, Cmd = ()> {
    done: Vec<CmdEdit<Cmd, E>>,
    undone: Vec<CmdEdit<Cmd, E>>,
}

impl<Cmd, E> UndoRedo<E, Cmd> {
    /// Create a new empty history bistack.
    #[inline]
    pub fn new() -> Self {
        Self {
            done: Vec::new(),
            undone: Vec::new(),
        }
    }

    /// Returns a slice of the *done* stack, which contains all the done (or
    /// redone) edits.
    #[inline]
    pub fn done(&self) -> &[CmdEdit<Cmd, E>] {
        &self.done
    }

    /// Returns a slice of the *undone* stack, which contains all the undone
    /// edits.
    #[inline]
    pub fn undone(&self) -> &[CmdEdit<Cmd, E>] {
        &self.undone
    }
}

impl<Cmd: Default, E> UndoRedo<E, Cmd> {
    /// Flush the target container and push its changes as an edit onto the
    /// *done* stack.
    ///
    /// This clears the undone stack.
    #[inline]
    pub fn commit<T>(&mut self, target: &mut T)
    where
        E: ExtractEdit<T>,
    {
        self.cmd_commit(Default::default(), target);
    }
}

impl<Cmd, E> UndoRedo<E, Cmd> {
    /// Flush the target container and push its changes as an edit onto the
    /// *done* stack as an edit along with additional metadata ("cmd").
    ///
    /// This clears the undone stack.
    #[inline]
    pub fn cmd_commit<T>(&mut self, cmd: Cmd, target: &mut T)
    where
        E: ExtractEdit<T>,
    {
        self.done.push(CmdEdit {
            cmd,
            edit: E::extract_edit(target),
        });
        self.undone.clear();
    }
}

impl<Cmd> UndoRedo<(), Cmd> {
    /// Push command onto the *done* stack without any edit (delta or snapshot).
    ///
    /// This is a convenience interface for [Command
    /// pattern](https://en.wikipedia.org/wiki/Command_pattern), equivalent to
    /// calling [`cmd_commit()`] with the `command` as `cmd` and `()` as `edit`.
    #[inline]
    pub fn command(&mut self, command: Cmd) {
        self.cmd_commit(command, &mut ());
    }
}

impl<Cmd: Clone, E: Clone> UndoRedo<E, Cmd> {
    /// Undo the last done edit.
    ///
    /// The undone edit is popped from the *done* stack, reversed, reverted,
    /// and pushed onto the *undone* stack.
    #[inline]
    pub fn undo<T>(&mut self, target: &mut T) -> Option<Cmd>
    where
        E: RevertEdit<T>,
    {
        let CmdEdit { cmd, edit } = self.done.pop()?;
        self.undone.push(CmdEdit {
            cmd: cmd.clone(),
            edit: edit.revert_edit(target),
        });

        Some(cmd)
    }

    /// Redo the last undone edit.
    ///
    /// The redone edit is popped from the *undone* stack, reversed, reverted,
    /// and pushed back onto the *done* stack.
    #[inline]
    pub fn redo<T>(&mut self, target: &mut T) -> Option<Cmd>
    where
        E: RevertEdit<T>,
    {
        let CmdEdit { cmd, edit } = self.undone.pop()?;
        self.done.push(CmdEdit {
            cmd: cmd.clone(),
            edit: edit.revert_edit(target),
        });

        Some(cmd)
    }
}

impl<Cmd: Clone> UndoRedo<(), Cmd> {
    /// Pop the last done command from the *done* stack and push it onto the
    /// *undone* stack, returning its clone. The underlying data is not anyhow
    /// mutated otherwise.
    ///
    /// This is a convenience interface for [Command
    /// pattern](https://en.wikipedia.org/wiki/Command_pattern). Implementing
    /// the returned command's behavior is the responsibility of the caller.
    #[inline]
    pub fn undo_command(&mut self) -> Option<Cmd> {
        let CmdEdit { cmd, .. } = self.done.pop()?;
        self.undone.push(CmdEdit {
            cmd: cmd.clone(),
            edit: (),
        });

        Some(cmd)
    }

    /// Pop the last undone command from the *undone* stack and push it onto the
    /// *done* stack, returning its clone. The underlying data is not anyhow
    /// mutated otherwise.
    ///
    /// This is a convenience interface for [Command
    /// pattern](https://en.wikipedia.org/wiki/Command_pattern). Implementing
    /// the returned command's action is the responsibility of the caller.
    #[inline]
    pub fn redo_command(&mut self) -> Option<Cmd> {
        let CmdEdit { cmd, .. } = self.undone.pop()?;
        self.done.push(CmdEdit {
            cmd: cmd.clone(),
            edit: (),
        });

        Some(cmd)
    }
}

impl<Cmd, DC: Container + Default> UndoRedo<Delta<DC>, Cmd> {
    /// Make and record changes to the recorded container from within a closure,
    /// automatically committing them once the closure finishes.
    #[inline]
    pub fn edit<K, V, C, F>(&mut self, container: C, f: F) -> C
    where
        C: Container<Key = K, Value = V> + Get<K>,
        DC: Container<Key = K, Value = V>,
        K: Clone,
        V: Clone,
        F: FnOnce(&mut Recorder<C, DC>) -> Cmd,
    {
        let mut recorder = Recorder::<C, DC>::new(container);
        let cmd = f(&mut recorder);
        let (container, delta) = recorder.dissolve();

        self.done.push(CmdEdit { cmd, edit: delta });
        self.undone.clear();

        container
    }
}
