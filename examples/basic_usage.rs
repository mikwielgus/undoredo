// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use undoredo::{Insert, Recorder, UndoRedo};

#[allow(unused_mut)]
fn main() {
    // The recorder records the ongoing changes to the recorded collection.
    let mut recorder: Recorder<usize, char, HashMap<usize, char>> = Recorder::new(HashMap::new());

    // The undo-redo struct maintains the undo-redo bistack.
    let mut undoredo: UndoRedo<HashMap<usize, char>> = UndoRedo::new();

    // Push elements while recording the changes in an edit.
    recorder.insert(1, 'A');
    recorder.insert(2, 'B');
    recorder.insert(3, 'C');

    // Flush the recorder and commit the recorded edit of pushing 'A', 'B', 'C'
    // into the undo-redo bistack.
    undoredo.commit(recorder.flush());

    // The pushed elements are now present in the collection.
    assert!(*recorder.collection() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Now undo the action.
    undoredo.undo(&mut recorder);

    // The collection is now empty; the action of pushing elements has been undone.
    assert!(*recorder.collection() == HashMap::from([]));

    // Now redo the action.
    undoredo.redo(&mut recorder);

    // The elements are back in the collection; the action has been redone.
    assert!(*recorder.collection() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Once you are done recording, you can dissolve the recorder to regain
    // ownership and mutability over the recorded collection.
    let (mut hashmap, ..) = recorder.dissolve();
    assert!(hashmap == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));
}

#[test]
fn test() {
    main();
}
