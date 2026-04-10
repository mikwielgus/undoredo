// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use stable_vec::StableVec;
use undoredo::delta::{StableVecDelta, StableVecHalfDelta};
use undoredo::{Recorder, UndoRedo};

fn main() {
    let mut recorder: Recorder<StableVec<char>, StableVecHalfDelta<char>> =
        Recorder::new(StableVec::new());
    let mut undoredo: UndoRedo<StableVecDelta<char>> = UndoRedo::new();

    recorder.push('A');
    undoredo.commit(&mut recorder);

    recorder.push('B');
    recorder.push('B');
    undoredo.commit(&mut recorder);

    let key = recorder.push('X');
    recorder.remove(&key);
    recorder.push('C');
    undoredo.commit(&mut recorder);

    assert!(
        recorder
            .container()
            .values()
            .copied()
            .eq(['A', 'B', 'B', 'C'])
    );

    undoredo.undo(&mut recorder);
    assert!(recorder.container().values().copied().eq(['A', 'B', 'B']));

    undoredo.undo(&mut recorder);
    assert!(recorder.container().values().copied().eq(['A']));

    undoredo.redo(&mut recorder);
    assert!(recorder.container().values().copied().eq(['A', 'B', 'B']));

    undoredo.redo(&mut recorder);
    assert!(
        recorder
            .container()
            .values()
            .copied()
            .eq(['A', 'B', 'B', 'C'])
    );
}

#[test]
fn test() {
    main();
}
