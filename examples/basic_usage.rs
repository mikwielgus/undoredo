// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use undoredo::aliases::{HashMapDelta, HashMapHalfDelta};
use undoredo::{Recorder, UndoRedo};

fn main() {
    // The recorder records the ongoing changes to the recorded container.
    //
    // For some containers such as `HashMap` and `HashSet`, you need
    // to explicitly pass a half-delta type to the `Recorder` (here
    // `HashMapHalfDelta`). For most containers, however, you don't need to do
    // that because their half-delta is already the default, `BTreeMap`.
    let mut recorder: Recorder<HashMap<usize, char>, HashMapHalfDelta<usize, char>> =
        Recorder::new(HashMap::new());

    // The undo-redo struct that holds and maintains the undo-redo bistack.
    let mut undoredo: UndoRedo<HashMapDelta<usize, char>> = UndoRedo::new();

    // Push elements while recording the changes in a delta.
    recorder.insert(1, 'A');
    recorder.insert(2, 'B');
    recorder.insert(3, 'C');

    // The pushed elements are now present in the container.
    assert!(*recorder.container() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Flush the recorder and commit the recorded delta of pushing 'A', 'B', 'C'
    // into the undo-redo bistack.
    undoredo.commit(&mut recorder);

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
    let (hashmap, ..) = recorder.dissolve();
    assert!(hashmap == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));
}

#[test]
fn test() {
    main();
}
