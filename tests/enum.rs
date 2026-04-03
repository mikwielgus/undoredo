// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use maplike::{Assign, Container};
use undoredo::{Recorder, UndoRedo};
use undoredo_derive::ApplyDelta;

#[derive(ApplyDelta, Assign, Container, Clone, Debug, PartialEq)]
enum TestEnum {
    Unit,
    Tuple(Vec<i32>, Vec<i32>),
    Fields { i: i32, u: u32 },
}

#[test]
fn test_enum() {
    let mut recorder: Recorder<TestEnum> = Recorder::new(TestEnum::Unit);
    let mut undoredo: UndoRedo<_> = UndoRedo::new();
    assert_eq!(recorder.container(), &TestEnum::Unit);

    recorder.assign(TestEnum::Tuple(Vec::new(), Vec::new()));
    undoredo.commit(&mut recorder);
    assert_eq!(recorder.container(), &TestEnum::Tuple(vec![], vec![]));

    recorder.assign(TestEnum::Fields { i: 1, u: 2 });
    undoredo.commit(&mut recorder);
    assert_eq!(recorder.container(), &TestEnum::Fields { i: 1, u: 2 });

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(recorder.container(), &TestEnum::Tuple(vec![], vec![]));

    assert!(undoredo.undo(&mut recorder).is_some());
    assert_eq!(recorder.container(), &TestEnum::Unit);

    assert_eq!(undoredo.undo(&mut recorder), None);
    assert_eq!(recorder.container(), &TestEnum::Unit);

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(recorder.container(), &TestEnum::Tuple(vec![], vec![]));

    assert!(undoredo.redo(&mut recorder).is_some());
    assert_eq!(recorder.container(), &TestEnum::Fields { i: 1, u: 2 });

    assert_eq!(undoredo.redo(&mut recorder), None);
    assert_eq!(recorder.container(), &TestEnum::Fields { i: 1, u: 2 });
}
