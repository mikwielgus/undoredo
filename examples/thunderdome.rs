// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thunderdome::Arena;
use undoredo::aliases::{ArenaDelta, ArenaHalfDelta};
use undoredo::{Recorder, UndoRedo};

fn main() {
    let mut recorder: Recorder<Arena<char>, ArenaHalfDelta<char>> = Recorder::new(Arena::new());
    let mut undoredo: UndoRedo<ArenaDelta<char>> = UndoRedo::new();

    recorder.push('A');
    undoredo.commit(&mut recorder);

    recorder.push('B');
    recorder.push('B');
    undoredo.commit(&mut recorder);

    let key = recorder.push('X');
    recorder.remove(&key);
    recorder.push('C');
    undoredo.commit(&mut recorder);

    let (_, values): (Vec<_>, Vec<char>) = recorder.container().clone().into_iter().unzip();
    assert!(values == vec!['A', 'B', 'B', 'C']);

    undoredo.undo(&mut recorder);
    let (_, values): (Vec<_>, Vec<char>) = recorder.container().clone().into_iter().unzip();
    assert!(values == vec!['A', 'B', 'B']);

    undoredo.undo(&mut recorder);
    let (_, values): (Vec<_>, Vec<char>) = recorder.container().clone().into_iter().unzip();
    assert!(values == vec!['A']);

    undoredo.redo(&mut recorder);
    let (_, values): (Vec<_>, Vec<char>) = recorder.container().clone().into_iter().unzip();
    assert!(values == vec!['A', 'B', 'B']);

    undoredo.redo(&mut recorder);
    let (_, values): (Vec<_>, Vec<char>) = recorder.container().clone().into_iter().unzip();
    assert!(values == vec!['A', 'B', 'B', 'C']);
}

#[test]
fn test() {
    main();
}
