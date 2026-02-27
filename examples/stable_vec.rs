// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use stable_vec::StableVec;
use undoredo::{Recorder, UndoRedo};

fn main() {
    let mut recorder: Recorder<StableVec<char>> = Recorder::new(StableVec::new());
    let mut undoredo: UndoRedo<StableVec<char>> = UndoRedo::new();

    recorder.push('A');
    undoredo.commit(recorder.flush_edit());

    recorder.push('B');
    recorder.push('B');
    undoredo.commit(recorder.flush_edit());

    let key = recorder.push('X');
    recorder.remove(&key);
    recorder.push('C');
    undoredo.commit(recorder.flush_edit());

    assert!(
        recorder
            .collection()
            .values()
            .copied()
            .eq(['A', 'B', 'B', 'C'])
    );

    undoredo.undo(&mut recorder);
    assert!(recorder.collection().values().copied().eq(['A', 'B', 'B']));

    undoredo.undo(&mut recorder);
    assert!(recorder.collection().values().copied().eq(['A']));

    undoredo.redo(&mut recorder);
    assert!(recorder.collection().values().copied().eq(['A', 'B', 'B']));

    undoredo.redo(&mut recorder);
    assert!(
        recorder
            .collection()
            .values()
            .copied()
            .eq(['A', 'B', 'B', 'C'])
    );
}

#[test]
fn test() {
    main();
}
