//
// SPDX-License-Identifier: MIT OR Apache-2.0

use undoredo::{Recorder, UndoRedo};
use undoredo_derive::UndoRedo;

#[derive(UndoRedo, Clone, Debug, PartialEq)]
enum TestEnum {
    Unit,
    Tuple(Vec<i32>, Vec<i32>),
}

#[derive(UndoRedo, Clone, Debug, PartialEq)]
struct TestStruct {
    v: Recorder<Vec<TestEnum>>,
    i: Recorder<TestEnum>,
}

#[test]
fn test_struct() {
    let mut s = TestStruct {
        v: Recorder::new(vec![]),
        i: Recorder::new(TestEnum::Unit),
    };
    let mut undoredo: UndoRedo<_> = UndoRedo::new();
    assert_eq!(*s.v.container(), vec![]);
    assert_eq!(s.i.container(), &TestEnum::Unit);

    s.v.push(TestEnum::Unit);
    s.i.assign(TestEnum::Tuple(vec![1], vec![2, 3]));
    undoredo.commit(&mut s);

    assert_eq!(*s.v.container(), vec![TestEnum::Unit]);
    assert_eq!(s.i.container(), &TestEnum::Tuple(vec![1], vec![2, 3]));

    s.v.push(TestEnum::Tuple(vec![4], vec![5, 6]));
    s.i.assign(TestEnum::Unit);
    undoredo.commit(&mut s);

    assert_eq!(
        *s.v.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![4], vec![5, 6])]
    );
    assert_eq!(s.i.container(), &TestEnum::Unit);

    assert!(undoredo.undo(&mut s).is_some());
    assert_eq!(*s.v.container(), vec![TestEnum::Unit]);
    assert_eq!(s.i.container(), &TestEnum::Tuple(vec![1], vec![2, 3]));

    assert!(undoredo.undo(&mut s).is_some());
    assert_eq!(*s.v.container(), vec![]);
    assert_eq!(s.i.container(), &TestEnum::Unit);

    assert!(undoredo.undo(&mut s).is_none());
    assert_eq!(*s.v.container(), vec![]);
    assert_eq!(s.i.container(), &TestEnum::Unit);

    assert!(undoredo.redo(&mut s).is_some());
    assert_eq!(*s.v.container(), vec![TestEnum::Unit]);
    assert_eq!(s.i.container(), &TestEnum::Tuple(vec![1], vec![2, 3]));

    assert!(undoredo.redo(&mut s).is_some());
    assert_eq!(
        *s.v.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![4], vec![5, 6])]
    );
    assert_eq!(s.i.container(), &TestEnum::Unit);

    assert!(undoredo.redo(&mut s).is_none());
    assert_eq!(
        *s.v.container(),
        vec![TestEnum::Unit, TestEnum::Tuple(vec![4], vec![5, 6])]
    );
    assert_eq!(s.i.container(), &TestEnum::Unit);
}
