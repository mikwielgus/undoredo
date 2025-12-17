// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec::Vec;

use crate::{
    Edit, Recorder,
    edit::ApplyEdit,
    maplike::{Get, Insert},
};

pub struct CmdEdit<Cmd, EC> {
    pub cmd: Cmd,
    pub edit: Edit<EC>,
}

pub struct UndoRedo<EC, Cmd = ()> {
    done: Vec<CmdEdit<Cmd, EC>>,
    undone: Vec<CmdEdit<Cmd, EC>>,
}

impl<Cmd, EC> UndoRedo<EC, Cmd> {
    pub fn new() -> Self {
        Self {
            done: Vec::new(),
            undone: Vec::new(),
        }
    }

    pub fn done(&self) -> &[CmdEdit<Cmd, EC>] {
        &self.done
    }

    pub fn undone(&self) -> &[CmdEdit<Cmd, EC>] {
        &self.undone
    }
}

impl<Cmd: Default, EC> UndoRedo<EC, Cmd> {
    pub fn commit(&mut self, edit: Edit<EC>) {
        self.done.push(CmdEdit {
            cmd: Default::default(),
            edit,
        });
        self.undone.clear();
    }
}

impl<Cmd, EC> UndoRedo<EC, Cmd> {
    pub fn cmd_commit(&mut self, cmd: Cmd, edit: Edit<EC>) {
        self.done.push(CmdEdit { cmd, edit });
        self.undone.clear();
    }
}

impl<Cmd, EC: Default> UndoRedo<EC, Cmd> {
    pub fn edit<
        K: Clone,
        V: Clone,
        C: Get<K, Item = V> + Insert<K>,
        F: FnOnce(&mut Recorder<K, V, C, EC>) -> Cmd,
    >(
        &mut self,
        collection: C,
        f: F,
    ) -> C {
        let mut recorder = Recorder::<K, V, C, EC>::new(collection);
        let cmd = f(&mut recorder);
        let (container, edit) = recorder.dissolve();

        self.cmd_commit(cmd, edit);

        container
    }
}

impl<Cmd: Clone, EC: Clone> UndoRedo<EC, Cmd> {
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
