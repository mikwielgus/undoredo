// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rstar::RTree;
use undoredo::aliases::{RTreeDelta, RTreeHalfDelta};
use undoredo::{Recorder, UndoRedo};

fn main() {
    let mut recorder: Recorder<RTree<(i32, i32)>, RTreeHalfDelta<(i32, i32)>> =
        Recorder::new(RTree::new());
    let mut undoredo: UndoRedo<RTreeDelta<(i32, i32)>> = UndoRedo::new();

    recorder.insert((1, 1), ());
    undoredo.commit(&mut recorder);

    recorder.insert((2, 2), ());
    undoredo.commit(&mut recorder);

    recorder.insert((3, 3), ());
    undoredo.commit(&mut recorder);

    assert_eq!(
        recorder.container().iter().collect::<Vec<_>>(),
        RTree::bulk_load(vec![(1, 1), (2, 2), (3, 3)])
            .iter()
            .collect::<Vec<_>>()
    );

    undoredo.undo(&mut recorder);
    assert_eq!(
        *recorder.container().iter().collect::<Vec<_>>(),
        RTree::bulk_load(vec![(1, 1), (2, 2)])
            .iter()
            .collect::<Vec<_>>()
    );

    undoredo.undo(&mut recorder);
    assert_eq!(
        *recorder.container().iter().collect::<Vec<_>>(),
        RTree::bulk_load(vec![(1, 1)]).iter().collect::<Vec<_>>()
    );

    undoredo.redo(&mut recorder);
    assert_eq!(
        *recorder.container().iter().collect::<Vec<_>>(),
        RTree::bulk_load(vec![(1, 1), (2, 2)])
            .iter()
            .collect::<Vec<_>>()
    );

    undoredo.redo(&mut recorder);
    assert_eq!(
        *recorder.container().iter().collect::<Vec<_>>(),
        RTree::bulk_load(vec![(1, 1), (2, 2), (3, 3)])
            .iter()
            .collect::<Vec<_>>()
    );
}

#[test]
fn test() {
    main();
}
