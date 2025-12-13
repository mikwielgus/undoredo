// SPDX-FileCopyrightText: 2025 undoredo Developers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thunderdome::{Arena, Index};
use undoredo::{Push, Recorder, Remove, UndoRedo};

#[test]
fn main() {
    let mut recorder: Recorder<Index, char, Arena<char>> = Recorder::new(Arena::new());
    let mut undoredo: UndoRedo<Arena<char>> = UndoRedo::new();

    recorder.push('A');
    undoredo.commit(recorder.flush());

    recorder.push('B');
    recorder.push('B');
    undoredo.commit(recorder.flush());

    let key = recorder.push('X');
    recorder.remove(&key);
    recorder.push('C');
    undoredo.commit(recorder.flush());

    let (_, values): (Vec<_>, Vec<char>) = recorder.collection().clone().into_iter().unzip();
    assert!(values == vec!['A', 'B', 'B', 'C']);

    undoredo.undo(&mut recorder);
    let (_, values): (Vec<_>, Vec<char>) = recorder.collection().clone().into_iter().unzip();
    assert!(values == vec!['A', 'B', 'B']);

    undoredo.undo(&mut recorder);
    let (_, values): (Vec<_>, Vec<char>) = recorder.collection().clone().into_iter().unzip();
    assert!(values == vec!['A']);

    undoredo.redo(&mut recorder);
    let (_, values): (Vec<_>, Vec<char>) = recorder.collection().clone().into_iter().unzip();
    assert!(values == vec!['A', 'B', 'B']);

    undoredo.redo(&mut recorder);
    let (_, values): (Vec<_>, Vec<char>) = recorder.collection().clone().into_iter().unzip();
    assert!(values == vec!['A', 'B', 'B', 'C']);
}
