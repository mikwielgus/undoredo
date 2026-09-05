// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use undoredo::aliases::VecDelta;
use undoredo::{Recorder, UndoRedo};

fn main() {
    let mut recorder: Recorder<Vec<char>> = Recorder::new(Vec::new());
    let mut undoredo: UndoRedo<VecDelta<char>> = UndoRedo::new();

    recorder.push('A');
    recorder.push('B');
    recorder.push('C');
    recorder.push('D');
    undoredo.commit(&mut recorder);

    assert_eq!(*recorder.container(), vec!['A', 'B', 'C', 'D']);

    // `Vec` has no stable remove, i.e. remove that does not invalidate indices.
    // So we use the `.swap_remove()` method instead, which swaps the removee
    // with the last element and pops.
    recorder.swap_remove(&1);
    undoredo.commit(&mut recorder);

    // 'B' at index 1 was removed; previously last 'D' took its place.
    assert_eq!(*recorder.container(), vec!['A', 'D', 'C']);

    undoredo.undo(&mut recorder);
    assert_eq!(*recorder.container(), vec!['A', 'B', 'C', 'D']);

    undoredo.redo(&mut recorder);
    assert_eq!(*recorder.container(), vec!['A', 'D', 'C']);

    // Removing the last element is just a pop; no swap takes place.
    recorder.swap_remove(&2);
    undoredo.commit(&mut recorder);
    assert_eq!(*recorder.container(), vec!['A', 'D']);

    undoredo.undo(&mut recorder);
    assert_eq!(*recorder.container(), vec!['A', 'D', 'C']);

    undoredo.redo(&mut recorder);
    assert_eq!(*recorder.container(), vec!['A', 'D']);
}

#[test]
fn test() {
    main();
}
