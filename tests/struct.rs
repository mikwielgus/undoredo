// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[path = "common/mod.rs"]
mod common;

use std::collections::BTreeMap;

use undoredo::aliases::BTreeMapDelta;
use undoredo::{Delta, Recorder, UndoRedo};

#[derive(Delta, Clone, Debug, Eq, Hash, PartialEq)]
enum TestEnum {
    Unit,
    Tuple(Vec<i32>, Vec<i32>),
}

#[derive(Delta, Clone, Debug, Eq, Hash, PartialEq)]
struct TestStruct {
    v: Recorder<Vec<TestEnum>>,
    i: Recorder<i32>,
}

#[test]
fn test_struct() {
    let mut s = TestStruct {
        v: Recorder::new(vec![]),
        i: Recorder::new(0),
    };
    let mut undoredo: UndoRedo<TestStructDelta> = UndoRedo::new();
    assert_eq!(*s.v.container(), vec![]);
    assert_eq!(s.i.container(), &0);

    s.v.push(TestEnum::Unit);
    s.i.assign(123);
    undoredo.commit(&mut s);

    assert_eq!(*s.v.container(), vec![TestEnum::Unit]);
    assert_eq!(s.i.container(), &123);

    s.v.push(TestEnum::Tuple(vec![4], vec![5, 6]));
    s.i.assign(7);
    undoredo.commit(&mut s);

    assert_eq!(
        *s.v.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![4], vec![5, 6])]
    );
    assert_eq!(s.i.container(), &7);

    assert!(undoredo.undo(&mut s).is_some());
    assert_eq!(*s.v.container(), vec![TestEnum::Unit]);
    assert_eq!(s.i.container(), &123);

    assert!(undoredo.undo(&mut s).is_some());
    assert_eq!(*s.v.container(), vec![]);
    assert_eq!(s.i.container(), &0);

    assert!(undoredo.undo(&mut s).is_none());
    assert_eq!(*s.v.container(), vec![]);
    assert_eq!(s.i.container(), &0);

    assert!(undoredo.redo(&mut s).is_some());
    assert_eq!(*s.v.container(), vec![TestEnum::Unit]);
    assert_eq!(s.i.container(), &123);

    assert!(undoredo.redo(&mut s).is_some());
    assert_eq!(
        *s.v.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![4], vec![5, 6])]
    );
    assert_eq!(s.i.container(), &7);

    assert!(undoredo.redo(&mut s).is_none());
    assert_eq!(
        *s.v.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![4], vec![5, 6])]
    );
    assert_eq!(s.i.container(), &7);
}

#[test]
fn test_struct_vec() {
    let mut recorder: Recorder<Vec<TestStruct>> = Recorder::new(vec![]);
    let mut undoredo: UndoRedo<BTreeMapDelta<usize, TestStruct>> = UndoRedo::new();
    assert_eq!(*recorder.container(), vec![]);

    recorder.push(TestStruct {
        v: Recorder::new(vec![TestEnum::Unit]),
        i: Recorder::new(10),
    });
    undoredo.commit(&mut recorder);
    assert_eq!(
        *recorder.container(),
        vec![TestStruct {
            v: Recorder::new(vec![TestEnum::Unit]),
            i: Recorder::new(10),
        }]
    );

    recorder.push(TestStruct {
        v: Recorder::new(vec![TestEnum::Tuple(vec![1], vec![2, 3])]),
        i: Recorder::new(20),
    });
    undoredo.commit(&mut recorder);
    assert_eq!(
        *recorder.container(),
        vec![
            TestStruct {
                v: Recorder::new(vec![TestEnum::Unit]),
                i: Recorder::new(10),
            },
            TestStruct {
                v: Recorder::new(vec![TestEnum::Tuple(vec![1], vec![2, 3])]),
                i: Recorder::new(20),
            }
        ]
    );

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(
        *recorder.container(),
        vec![TestStruct {
            v: Recorder::new(vec![TestEnum::Unit]),
            i: Recorder::new(10),
        }]
    );

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(*recorder.container(), vec![]);

    assert!(undoredo.undo(&mut recorder).is_none());
    assert_eq!(*recorder.container(), vec![]);

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(
        *recorder.container(),
        vec![TestStruct {
            v: Recorder::new(vec![TestEnum::Unit]),
            i: Recorder::new(10),
        }]
    );

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(
        *recorder.container(),
        vec![
            TestStruct {
                v: Recorder::new(vec![TestEnum::Unit]),
                i: Recorder::new(10),
            },
            TestStruct {
                v: Recorder::new(vec![TestEnum::Tuple(vec![1], vec![2, 3])]),
                i: Recorder::new(20),
            }
        ]
    );

    assert!(undoredo.redo(&mut recorder).is_none());
    assert_eq!(
        *recorder.container(),
        vec![
            TestStruct {
                v: Recorder::new(vec![TestEnum::Unit]),
                i: Recorder::new(10),
            },
            TestStruct {
                v: Recorder::new(vec![TestEnum::Tuple(vec![1], vec![2, 3])]),
                i: Recorder::new(20),
            }
        ]
    );
}

#[test]
fn test_snapshot_undo_redo() {
    common::test_snapshot_undo_redo::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
}

#[test]
fn test_history_tree_command_checkout() {
    common::test_history_tree_command_checkout();
}
