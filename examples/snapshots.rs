// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use undoredo::{Snapshot, UndoRedo};

fn main() {
    // A container that will hold some elements.
    // NOTE: Unlike with deltas, there is no need to wrap the container in a
    // `Recorder`. The container is instead cloned upon committing.
    let mut container: HashMap<usize, char> = HashMap::new();

    // The undo-redo struct that holds and maintains the undo-redo bistack.
    let mut undoredo: UndoRedo<Snapshot<HashMap<usize, char>>> = UndoRedo::new();

    // NOTE: Unlike with deltas. here to be able undo back to the initial, empty
    // state, you need to commit it beforehand. Otherwise, there will be no
    // snapshot available to restore it.
    undoredo.commit(&mut container);

    // Push elements.
    container.insert(1, 'A');
    container.insert(2, 'B');
    container.insert(3, 'C');

    // The pushed elements are now present in the container.
    assert!(container == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // NOTE: Here you again need to do differently than with deltas. We do not
    // commit the latest changes before the first undo, because then we would
    // have to undo twice.

    // Now undo the action.
    undoredo.undo(&mut container);

    // The container is now empty; the action of pushing elements has been undone.
    assert!(container == HashMap::from([]));

    // Now redo the action.
    undoredo.redo(&mut container);

    // The elements are back in the container; the action has been redone.
    assert!(container == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // NOTE: Since, unlike with deltas, there is no `Recorder` that owns the
    // container, there is no need for dissolution to reclaim ownership.
}

#[test]
fn test() {
    main();
}
