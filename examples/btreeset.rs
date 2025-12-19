// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeSet;

use undoredo::{Insert, Recorder, UndoRedo};

fn main() {
    let mut recorder: Recorder<char, (), BTreeSet<char>> = Recorder::new(BTreeSet::new());
    let mut undoredo: UndoRedo<BTreeSet<char>> = UndoRedo::new();

    recorder.insert('A', ());
    undoredo.commit(recorder.flush());

    recorder.insert('B', ());
    // Inserting to a set is idempotent: repeating the same insert does nothing.
    // It is, however, a logic error if the recorded collection is a multiset,
    // e.g. `rstar::RTree`.
    recorder.insert('B', ());
    undoredo.commit(recorder.flush());

    recorder.insert('C', ());
    undoredo.commit(recorder.flush());

    assert_eq!(*recorder.collection(), BTreeSet::from(['A', 'B', 'C']));

    undoredo.undo(&mut recorder);
    assert_eq!(*recorder.collection(), BTreeSet::from(['A', 'B']));

    undoredo.undo(&mut recorder);
    assert_eq!(*recorder.collection(), BTreeSet::from(['A']));

    undoredo.redo(&mut recorder);
    assert_eq!(*recorder.collection(), BTreeSet::from(['A', 'B']));

    undoredo.redo(&mut recorder);
    assert_eq!(*recorder.collection(), BTreeSet::from(['A', 'B', 'C']));
}

#[test]
fn test() {
    main();
}
