// SPDX-FileCopyrightText: 2025 undoredo Developers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    Edit, Recorder,
    collection::{Get, Insert},
    edit::ApplyEdit,
};

pub struct UndoRedo<EC, Cmd = ()> {
    done: Vec<(Cmd, Edit<EC>)>,
    undone: Vec<(Cmd, Edit<EC>)>,
}

impl<Cmd, EC> UndoRedo<EC, Cmd> {
    pub fn new() -> Self {
        Self {
            done: Vec::new(),
            undone: Vec::new(),
        }
    }
}

impl<Cmd: Default, EC> UndoRedo<EC, Cmd> {
    pub fn commit(&mut self, edit: Edit<EC>) {
        self.done.push((Default::default(), edit));
        self.undone.clear();
    }
}

impl<Cmd, EC> UndoRedo<EC, Cmd> {
    pub fn cmd_commit(&mut self, cmd: Cmd, edit: Edit<EC>) {
        self.done.push((cmd, edit));
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
        container: C,
        f: F,
    ) -> C {
        let mut recorder = Recorder::<K, V, C, EC>::new(container);
        let cmd = f(&mut recorder);
        let (container, edit) = recorder.dissolve();

        self.cmd_commit(cmd, edit);

        container
    }
}

impl<Cmd: Clone, EC: Clone> UndoRedo<EC, Cmd> {
    pub fn undo(&mut self, target: &mut impl ApplyEdit<EC>) -> Option<Cmd> {
        let (cmd, edit) = self.done.pop()?;
        let reverse_edit = edit.reverse();

        target.apply_edit(&reverse_edit);
        self.undone.push((cmd.clone(), reverse_edit));

        Some(cmd)
    }

    pub fn redo(&mut self, target: &mut impl ApplyEdit<EC>) -> Option<Cmd> {
        let (cmd, edit) = self.undone.pop()?;
        let reverse_edit = edit.reverse();

        target.apply_edit(&reverse_edit);
        self.done.push((cmd.clone(), reverse_edit));

        Some(cmd)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        UndoRedo,
        collection::{Get, Insert, IntoIter, Push, Remove},
    };

    pub(crate) trait FromU32 {
        fn from_u32(u: usize) -> Self;
    }

    impl FromU32 for usize {
        fn from_u32(u: usize) -> usize {
            u
        }
    }

    pub(crate) fn test_edit_undo_redo_at_generated_indexes<
        K: Clone,
        C: Get<K, Item = i32> + Insert<K> + Remove<K> + Push<K> + IntoIter<K>,
        EC: Clone + Default + Insert<K, Item = i32> + IntoIter<K, Key = K> + Remove<K>,
    >(
        mut container: C,
    ) {
        let mut undoredo: UndoRedo<EC> = UndoRedo::new();
        assert_eq!(undoredo.undo(&mut container), None);
        assert_eq!(undoredo.redo(&mut container), None);

        let mut indexes = vec![];

        let mut container = undoredo.edit(container, |recorder| {
            indexes.push(recorder.push(10));
            // Repeat the same index to start indexing from 1 like in the test with specified indexes.
            indexes.push(indexes[0].clone());

            indexes.push(recorder.push(20));
            indexes.push(recorder.push(30));
            indexes.push(recorder.push(40));
            indexes.push(recorder.push(50));

            indexes.push(recorder.push(60));
            recorder.remove(&indexes[6]);
        });

        assert_eq!(undoredo.redo(&mut container), None);

        assert_eq!(container.get(&indexes[1]), Some(&10));
        assert_eq!(container.get(&indexes[2]), Some(&20));
        assert_eq!(container.get(&indexes[3]), Some(&30));
        assert_eq!(container.get(&indexes[4]), Some(&40));
        assert_eq!(container.get(&indexes[5]), Some(&50));

        let mut container = undoredo.edit(container, |recorder| {
            recorder.remove(&indexes[2]);
            recorder.insert(indexes[1].clone(), 11);
            recorder.insert(indexes[3].clone(), 33);
        });

        assert_eq!(container.get(&indexes[1]), Some(&11));
        assert_eq!(container.get(&indexes[2]), None);
        assert_eq!(container.get(&indexes[3]), Some(&33));
        assert_eq!(container.get(&indexes[4]), Some(&40));
        assert_eq!(container.get(&indexes[5]), Some(&50));

        assert!(undoredo.undo(&mut container).is_some());

        assert_eq!(container.get(&indexes[1]), Some(&10));
        assert_eq!(container.get(&indexes[2]), Some(&20));
        assert_eq!(container.get(&indexes[3]), Some(&30));
        assert_eq!(container.get(&indexes[4]), Some(&40));
        assert_eq!(container.get(&indexes[5]), Some(&50));

        assert!(undoredo.redo(&mut container).is_some());

        assert_eq!(container.get(&indexes[1]), Some(&11));
        assert_eq!(container.get(&indexes[2]), None);
        assert_eq!(container.get(&indexes[3]), Some(&33));
        assert_eq!(container.get(&indexes[4]), Some(&40));
        assert_eq!(container.get(&indexes[5]), Some(&50));

        let mut container = undoredo.edit(container, |recorder| {
            recorder.remove(&indexes[3]);
            recorder.insert(indexes[6].clone(), 60);
        });

        assert_eq!(container.get(&indexes[1]), Some(&11));
        assert_eq!(container.get(&indexes[2]), None);
        assert_eq!(container.get(&indexes[3]), None);
        assert_eq!(container.get(&indexes[4]), Some(&40));
        assert_eq!(container.get(&indexes[5]), Some(&50));
        assert_eq!(container.get(&indexes[6]), Some(&60));

        assert_eq!(undoredo.redo(&mut container), None);

        assert!(undoredo.undo(&mut container).is_some());
        assert!(undoredo.undo(&mut container).is_some());
        assert!(undoredo.undo(&mut container).is_some());
        assert_eq!(undoredo.undo(&mut container), None);

        assert!(undoredo.redo(&mut container).is_some());

        assert_eq!(container.get(&indexes[1]), Some(&10));
        assert_eq!(container.get(&indexes[2]), Some(&20));
        assert_eq!(container.get(&indexes[3]), Some(&30));
        assert_eq!(container.get(&indexes[4]), Some(&40));
        assert_eq!(container.get(&indexes[5]), Some(&50));

        assert!(undoredo.redo(&mut container).is_some());

        assert_eq!(container.get(&indexes[1]), Some(&11));
        assert_eq!(container.get(&indexes[2]), None);
        assert_eq!(container.get(&indexes[3]), Some(&33));
        assert_eq!(container.get(&indexes[4]), Some(&40));
        assert_eq!(container.get(&indexes[5]), Some(&50));
    }

    pub(crate) fn test_edit_undo_redo_at_specified_indexes<
        K: Clone + FromU32,
        C: Get<K, Item = i32> + Insert<K> + IntoIter<K, Key = K> + Remove<K>,
        EC: Clone + Default + Insert<K, Item = i32> + IntoIter<K, Key = K> + Remove<K>,
    >(
        mut container: C,
    ) {
        let mut undoredo: UndoRedo<EC> = UndoRedo::new();
        assert_eq!(undoredo.undo(&mut container), None);
        assert_eq!(undoredo.redo(&mut container), None);

        let mut container = undoredo.edit(container, |recorder| {
            recorder.insert(K::from_u32(1), 10);
            recorder.insert(K::from_u32(2), 20);
            recorder.insert(K::from_u32(3), 30);
            recorder.insert(K::from_u32(4), 40);
            recorder.insert(K::from_u32(5), 50);
        });

        assert_eq!(undoredo.redo(&mut container), None);

        assert_eq!(container.get(&K::from_u32(1)), Some(&10));
        assert_eq!(container.get(&K::from_u32(2)), Some(&20));
        assert_eq!(container.get(&K::from_u32(3)), Some(&30));
        assert_eq!(container.get(&K::from_u32(4)), Some(&40));
        assert_eq!(container.get(&K::from_u32(5)), Some(&50));

        let mut container = undoredo.edit(container, |recorder| {
            recorder.remove(&K::from_u32(2));
            recorder.insert(K::from_u32(1), 11);
            recorder.insert(K::from_u32(3), 33);
        });

        assert_eq!(container.get(&K::from_u32(1)), Some(&11));
        assert_eq!(container.get(&K::from_u32(2)), None);
        assert_eq!(container.get(&K::from_u32(3)), Some(&33));
        assert_eq!(container.get(&K::from_u32(4)), Some(&40));
        assert_eq!(container.get(&K::from_u32(5)), Some(&50));

        assert!(undoredo.undo(&mut container).is_some());

        assert_eq!(container.get(&K::from_u32(1)), Some(&10));
        assert_eq!(container.get(&K::from_u32(2)), Some(&20));
        assert_eq!(container.get(&K::from_u32(3)), Some(&30));
        assert_eq!(container.get(&K::from_u32(4)), Some(&40));
        assert_eq!(container.get(&K::from_u32(5)), Some(&50));

        assert!(undoredo.redo(&mut container).is_some());

        assert_eq!(container.get(&K::from_u32(1)), Some(&11));
        assert_eq!(container.get(&K::from_u32(2)), None);
        assert_eq!(container.get(&K::from_u32(3)), Some(&33));
        assert_eq!(container.get(&K::from_u32(4)), Some(&40));
        assert_eq!(container.get(&K::from_u32(5)), Some(&50));

        let mut container = undoredo.edit(container, |recorder| {
            recorder.remove(&K::from_u32(3));
            recorder.insert(K::from_u32(6), 60);
        });

        assert_eq!(container.get(&K::from_u32(1)), Some(&11));
        assert_eq!(container.get(&K::from_u32(2)), None);
        assert_eq!(container.get(&K::from_u32(3)), None);
        assert_eq!(container.get(&K::from_u32(4)), Some(&40));
        assert_eq!(container.get(&K::from_u32(5)), Some(&50));
        assert_eq!(container.get(&K::from_u32(6)), Some(&60));

        assert_eq!(undoredo.redo(&mut container), None);

        assert!(undoredo.undo(&mut container).is_some());
        assert!(undoredo.undo(&mut container).is_some());
        assert!(undoredo.undo(&mut container).is_some());
        assert_eq!(undoredo.undo(&mut container), None);

        assert!(undoredo.redo(&mut container).is_some());

        assert_eq!(container.get(&K::from_u32(1)), Some(&10));
        assert_eq!(container.get(&K::from_u32(2)), Some(&20));
        assert_eq!(container.get(&K::from_u32(3)), Some(&30));
        assert_eq!(container.get(&K::from_u32(4)), Some(&40));
        assert_eq!(container.get(&K::from_u32(5)), Some(&50));

        assert!(undoredo.redo(&mut container).is_some());

        assert_eq!(container.get(&K::from_u32(1)), Some(&11));
        assert_eq!(container.get(&K::from_u32(2)), None);
        assert_eq!(container.get(&K::from_u32(3)), Some(&33));
        assert_eq!(container.get(&K::from_u32(4)), Some(&40));
        assert_eq!(container.get(&K::from_u32(5)), Some(&50));
    }
}
