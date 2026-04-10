// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;

use rstar::{AABB, primitives::Rectangle};
use rstared::RTreed;
use undoredo::delta::{RTreedDelta, RTreedHalfDelta};
use undoredo::{Recorder, UndoRedo};

fn main() {
    // A hashmap of 2D rectangles will be the underlying container.
    let rect_hashmap: HashMap<i32, Rectangle<(i32, i32)>> = HashMap::new();
    // Wrap `RTreed` around the hashmap and then `Recorder` around it.
    let mut recorder = Recorder::<
        RTreed<HashMap<i32, Rectangle<(i32, i32)>>>,
        RTreedHalfDelta<i32, Rectangle<(i32, i32)>>,
    >::new(RTreed::new(rect_hashmap));
    let mut undoredo: UndoRedo<RTreedDelta<i32, Rectangle<(i32, i32)>>> = UndoRedo::new();

    // Insert two rectangles, recording them in the R-tree.
    recorder.insert(1, Rectangle::from_corners((0, 0), (1, 1)));
    undoredo.commit(&mut recorder);

    recorder.insert(2, Rectangle::from_corners((1, 1), (2, 2)));
    undoredo.commit(&mut recorder);

    // Locate the two rectangles in the R-tree.
    assert_eq!(
        recorder
            .container()
            .rtree()
            .locate_in_envelope(&AABB::from_corners((0, 0), (2, 2)))
            .count(),
        2
    );

    undoredo.undo(&mut recorder);

    // After undo, there is now only one rectangle.
    assert_eq!(
        recorder
            .container()
            .rtree()
            .locate_in_envelope(&AABB::from_corners((0, 0), (2, 2)))
            .count(),
        1
    );

    undoredo.undo(&mut recorder);

    // After another undo, there is no rectangles anymore.
    assert_eq!(
        recorder
            .container()
            .rtree()
            .locate_in_envelope(&AABB::from_corners((0, 0), (2, 2)))
            .count(),
        0
    );

    undoredo.redo(&mut recorder);

    // After redo, we are back to one rectangle.
    assert_eq!(
        recorder
            .container()
            .rtree()
            .locate_in_envelope(&AABB::from_corners((0, 0), (2, 2)))
            .count(),
        1
    );

    undoredo.redo(&mut recorder);

    // After another redo, we are back to two rectangles.
    assert_eq!(
        recorder
            .container()
            .rtree()
            .locate_in_envelope(&AABB::from_corners((0, 0), (2, 2)))
            .count(),
        2
    );
}

#[test]
fn test() {
    main();
}
