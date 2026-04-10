// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeSet;

use undoredo::delta::{BTreeSetDelta, BTreeSetHalfDelta};
use undoredo::{Recorder, UndoRedo};

fn main() {
    let mut recorder: Recorder<BTreeSet<char>, BTreeSetHalfDelta<char>> =
        Recorder::new(BTreeSet::new());
    let mut undoredo: UndoRedo<BTreeSetDelta<char>> = UndoRedo::new();

    recorder.insert('A', ());
    undoredo.commit(&mut recorder);

    recorder.insert('B', ());
    // Inserting to a set is idempotent: repeating the same insert does nothing.
    // It is, however, a logic error if the recorded container is a multiset,
    // e.g. `rstar::RTree`.
    recorder.insert('B', ());
    undoredo.commit(&mut recorder);

    recorder.insert('C', ());
    undoredo.commit(&mut recorder);

    assert_eq!(*recorder.container(), BTreeSet::from(['A', 'B', 'C']));

    undoredo.undo(&mut recorder);
    assert_eq!(*recorder.container(), BTreeSet::from(['A', 'B']));

    undoredo.undo(&mut recorder);
    assert_eq!(*recorder.container(), BTreeSet::from(['A']));

    undoredo.redo(&mut recorder);
    assert_eq!(*recorder.container(), BTreeSet::from(['A', 'B']));

    undoredo.redo(&mut recorder);
    assert_eq!(*recorder.container(), BTreeSet::from(['A', 'B', 'C']));
}

#[test]
fn test() {
    main();
}
