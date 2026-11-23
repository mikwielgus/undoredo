// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

use numlike::alg::AddGroupOpsSensible;
use numlike::types::{Object, Ratio, Vector2};
use undoredo::maplike::abc::{Container, Keyed};
use undoredo::maplike::ops::{Assign, Get};
use undoredo::{ApplyDelta, Delta, Recorder, UndoRedo};

fn test_numlike_scalar_undo_redo<T>(initial: T, addend1: T, addend2: T)
where
    T: AddGroupOpsSensible
        + Assign
        + Container<Value = T>
        + Keyed<Key = usize, Value = T>
        + Get<usize>
        + ApplyDelta<BTreeMap<usize, T>>,
{
    let first_edit = initial.clone() + addend1.clone();
    let second_edit = first_edit.clone() + addend2.clone();

    let mut undoredo: UndoRedo<Delta<BTreeMap<usize, T>>> = UndoRedo::new();
    let mut recorder = Recorder::new(initial.clone());

    recorder.assign(recorder.container().clone() + addend1.clone());
    undoredo.commit(&mut recorder);
    assert!(recorder.container() == &first_edit);

    recorder.assign(recorder.container().clone() + addend2.clone());
    undoredo.commit(&mut recorder);
    assert!(recorder.container() == &second_edit);

    assert!(undoredo.undo(&mut recorder).is_some());
    assert!(recorder.container() == &first_edit);

    assert!(undoredo.redo(&mut recorder).is_some());
    assert!(recorder.container() == &second_edit);
}

#[test]
fn test_ratio_delta_undo_redo() {
    test_numlike_scalar_undo_redo(Ratio::new(1, 2), Ratio::new(1, 4), Ratio::new(1, 12));
}

#[test]
fn test_vector2_delta_undo_redo() {
    test_numlike_scalar_undo_redo(
        Vector2::new(1, 2),
        Vector2::new(9, 18),
        Vector2::new(20, 20),
    );
}

#[test]
fn test_object_delta_undo_redo() {
    test_numlike_scalar_undo_redo(Object::new(1), Object::new(9), Object::new(10));
}
