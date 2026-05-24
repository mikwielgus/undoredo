// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bidimap::BiHashMap;
use undoredo::aliases::{BiHashMapDelta, BiHashMapHalfDelta};
use undoredo::{Recorder, UndoRedo};

fn main() {
    let mut map = BiHashMap::new();
    map.insert("A", 1);
    map.insert("BB", 2);
    map.insert("CCC", 3);

    let mut recorder: Recorder<BiHashMap<&str, usize>, BiHashMapHalfDelta<&str, usize>> =
        Recorder::new(map);
    let mut undoredo: UndoRedo<BiHashMapDelta<&str, usize>> = UndoRedo::new();

    assert_eq!(recorder.get_by_left(&"A"), Some(&1));
    assert_eq!(recorder.get_by_right(&2), Some(&"BB"));

    assert_eq!(recorder.remove_by_left(&"A"), Some(1));
    assert_eq!(recorder.remove_by_right(&3), Some("CCC"));
    undoredo.commit(&mut recorder);

    assert_eq!(recorder.get_by_left(&"A"), None);
    assert_eq!(recorder.get_by_right(&3), None);

    undoredo.undo(&mut recorder);
    assert_eq!(recorder.get_by_left(&"A"), Some(&1));
    assert_eq!(recorder.get_by_left(&"CCC"), Some(&3));

    undoredo.redo(&mut recorder);
    assert_eq!(recorder.get_by_left(&"A"), None);
    assert_eq!(recorder.get_by_right(&3), None);
}

#[test]
fn test() {
    main();
}
