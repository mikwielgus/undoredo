// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "common/mod.rs"]
mod common;

use std::collections::BTreeMap;

use undoredo::BTreeMapDelta;
use undoredo::{Delta, Recorder, UndoRedo};

#[derive(Delta, Clone, Debug, PartialEq)]
enum TestEnum {
    Unit,
    Tuple(Vec<i32>, Vec<i32>),
    Fields { i: i32, u: u32 },
}

#[test]
fn test_enum() {
    let mut recorder: Recorder<TestEnum> = Recorder::new(TestEnum::Unit);
    let mut undoredo: UndoRedo<BTreeMapDelta<usize, TestEnum>> = UndoRedo::new();
    assert_eq!(recorder.container(), &TestEnum::Unit);

    recorder.assign(TestEnum::Tuple(vec![1], vec![2, 3]));
    undoredo.commit(&mut recorder);
    assert_eq!(recorder.container(), &TestEnum::Tuple(vec![1], vec![2, 3]));

    recorder.assign(TestEnum::Fields { i: 1, u: 2 });
    undoredo.commit(&mut recorder);
    assert_eq!(recorder.container(), &TestEnum::Fields { i: 1, u: 2 });

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(recorder.container(), &TestEnum::Tuple(vec![1], vec![2, 3]));

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(recorder.container(), &TestEnum::Unit);

    assert!(undoredo.undo(&mut recorder).is_none());
    assert_eq!(recorder.container(), &TestEnum::Unit);

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(recorder.container(), &TestEnum::Tuple(vec![1], vec![2, 3]));

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(recorder.container(), &TestEnum::Fields { i: 1, u: 2 });

    assert!(undoredo.redo(&mut recorder).is_none());
    assert_eq!(recorder.container(), &TestEnum::Fields { i: 1, u: 2 });
}

#[test]
fn test_enum_vec() {
    let mut recorder: Recorder<Vec<TestEnum>> = Recorder::new(vec![]);
    let mut undoredo: UndoRedo<BTreeMapDelta<usize, TestEnum>> = UndoRedo::new();
    assert_eq!(*recorder.container(), vec![]);

    recorder.push(TestEnum::Unit);
    undoredo.commit(&mut recorder);
    assert_eq!(*recorder.container(), vec![TestEnum::Unit]);

    recorder.push(TestEnum::Tuple(vec![1], vec![2, 3]));
    undoredo.commit(&mut recorder);
    assert_eq!(
        *recorder.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![1], vec![2, 3])]
    );

    recorder.push(TestEnum::Fields { i: 1, u: 2 });
    undoredo.commit(&mut recorder);
    assert_eq!(
        *recorder.container(),
        vec![
            TestEnum::Unit,
            TestEnum::Tuple(vec![1], vec![2, 3]),
            TestEnum::Fields { i: 1, u: 2 }
        ]
    );

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(
        *recorder.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![1], vec![2, 3])]
    );

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(*recorder.container(), vec![TestEnum::Unit]);

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(*recorder.container(), vec![]);

    assert!(undoredo.undo(&mut recorder).is_none());
    assert_eq!(*recorder.container(), vec![]);

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(*recorder.container(), vec![TestEnum::Unit]);

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(
        *recorder.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![1], vec![2, 3])]
    );

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(
        *recorder.container(),
        vec![
            TestEnum::Unit,
            TestEnum::Tuple(vec![1], vec![2, 3]),
            TestEnum::Fields { i: 1, u: 2 }
        ]
    );

    assert!(undoredo.redo(&mut recorder).is_none());
    assert_eq!(
        *recorder.container(),
        vec![
            TestEnum::Unit,
            TestEnum::Tuple(vec![1], vec![2, 3]),
            TestEnum::Fields { i: 1, u: 2 }
        ]
    );
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
}
