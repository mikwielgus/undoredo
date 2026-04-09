// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::{BTreeMap, HashMap};
use undoredo::{Delta, Recorder, UndoRedo};

#[allow(unused_mut)]
fn main() {
    // The recorder records the ongoing changes to the recorded container.
    let mut recorder: Recorder<HashMap<usize, char>, BTreeMap<usize, char>> =
        Recorder::new(HashMap::new());

    // The undo-redo struct maintains the undo-redo bistack.
    let mut undoredo: UndoRedo<Delta<BTreeMap<usize, char>>> = UndoRedo::new();

    // Push elements while recording the changes in an delta.
    recorder.insert(1, 'A');
    recorder.insert(2, 'B');
    recorder.insert(3, 'C');

    // Flush the recorder and commit the recorded delta of pushing 'A', 'B', 'C'
    // into the undo-redo bistack.
    undoredo.commit(&mut recorder);

    // The pushed elements are now present in the container.
    assert!(*recorder.container() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Now undo the action.
    undoredo.undo(&mut recorder);

    // The container is now empty; the action of pushing elements has been undone.
    assert!(*recorder.container() == HashMap::from([]));

    // Now redo the action.
    undoredo.redo(&mut recorder);

    // The elements are back in the container; the action has been redone.
    assert!(*recorder.container() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Once you are done recording, you can dissolve the recorder to regain
    // ownership and mutability over the recorded container.
    let (mut hashmap, ..) = recorder.dissolve();
    assert!(hashmap == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));
}

#[test]
fn test() {
    main();
}
