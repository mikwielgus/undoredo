// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec::Vec;
use maplike::abc::Keyed;
use maplike::ops::Get;

use crate::{CmdEdit, Delta, ExtractEdit, Recorder, RevertEdit};

/// An undo stack, without redo functionality.
///
/// In stack-based backtracking search algorithm such as depth-first search and
/// IDA*, there's no need for redo action, so such a single undo stack, not a
/// bistack, is sufficient for backtracking.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct HistoryStack<E, Cmd = ()> {
    done: Vec<CmdEdit<Cmd, E>>,
}

impl<Cmd, E> HistoryStack<E, Cmd> {
    /// Create a new empty undo stack.
    #[inline]
    pub fn new() -> Self {
        Self { done: Vec::new() }
    }

    /// Returns a slice of the underlying stack of done edits.
    #[inline]
    pub fn done(&self) -> &[CmdEdit<Cmd, E>] {
        &self.done
    }
}

impl<Cmd: Default, E> HistoryStack<E, Cmd> {
    /// Flush the target and push its recent changes onto the undo stack as
    /// an edit.
    #[inline]
    pub fn commit<T>(&mut self, target: &mut T)
    where
        E: ExtractEdit<T>,
    {
        self.cmd_commit(Default::default(), target);
    }
}

impl<Cmd, E> HistoryStack<E, Cmd> {
    /// Flush the target and push its recent changes onto the stack as an edit
    /// along with additional metadata ("cmd").
    #[inline]
    pub fn cmd_commit<T>(&mut self, cmd: Cmd, target: &mut T)
    where
        E: ExtractEdit<T>,
    {
        self.done.push(CmdEdit {
            cmd,
            edit: E::extract_edit(target),
        });
    }
}

impl<Cmd> HistoryStack<(), Cmd> {
    /// Push command without any edit.
    #[inline]
    pub fn command(&mut self, command: Cmd) {
        self.cmd_commit(command, &mut ());
    }
}

impl<Cmd: Clone, E: Clone> HistoryStack<E, Cmd> {
    /// Undo the last done edit.
    ///
    /// The undone edit is popped from the *done* stack, reversed, reverted, and
    /// then returned.
    #[inline]
    pub fn undo<T>(&mut self, target: &mut T) -> Option<CmdEdit<Cmd, E>>
    where
        E: RevertEdit<T>,
    {
        let CmdEdit { cmd, edit } = self.done.pop()?;

        Some(CmdEdit {
            cmd: cmd.clone(),
            edit: edit.revert_edit(target),
        })
    }
}

impl<Cmd: Clone> HistoryStack<(), Cmd> {
    /// Pop the last done command and return it.
    ///
    /// This is a convenience interface for [Command
    /// pattern](https://en.wikipedia.org/wiki/Command_pattern). Implementing
    /// the returned command's behavior is the responsibility of the caller.
    #[inline]
    pub fn undo_command(&mut self) -> Option<Cmd> {
        let CmdEdit { cmd, .. } = self.done.pop()?;

        Some(cmd)
    }
}

impl<Cmd, DC: Keyed + Default> HistoryStack<Delta<DC>, Cmd> {
    /// Make and record changes to the recorded container from within a closure,
    /// automatically committing them once the closure finishes.
    #[inline]
    pub fn edit<K, V, C, F>(&mut self, container: C, f: F) -> C
    where
        C: Keyed<Value = V, Key = K> + Get<K>,
        DC: Keyed<Value = V, Key = K>,
        K: Clone,
        V: Clone,
        F: FnOnce(&mut Recorder<C, DC>) -> Cmd,
    {
        let mut recorder = Recorder::<C, DC>::new(container);
        let cmd = f(&mut recorder);
        let (container, delta) = recorder.dissolve();

        self.done.push(CmdEdit { cmd, edit: delta });

        container
    }
}

// TODO? tests.
