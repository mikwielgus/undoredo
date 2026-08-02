// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::boxed::Box;

use undoredo::aliases::BoxHalfDelta;
use undoredo::{ApplyDelta, Delta, Recorder};

#[test]
fn test_box_apply_delta_replace() {
    let mut value = Box::new(1);

    value.apply_delta(Delta::with_removed_inserted(
        BoxHalfDelta::from([(0, 1)]),
        BoxHalfDelta::from([(0, 2)]),
    ));

    assert_eq!(*value, 2);
}

// No test for clearing, since `Box` can never be empty, obviously.

#[test]
fn test_box_recorder() {
    let mut recorder = Recorder::<Box<i32>>::new(Box::new(10));

    recorder.set(0, 20);
    assert_eq!(**recorder.container(), 20);

    let delta = recorder.flush_delta();
    recorder.apply_delta(delta.reverse());
    assert_eq!(**recorder.container(), 10);

    recorder.set(0, 30);
    let _ = recorder.flush_delta();
    recorder.set(0, 40);
    assert_eq!(**recorder.container(), 40);

    let delta = recorder.flush_delta();
    recorder.apply_delta(delta.reverse());

    assert_eq!(**recorder.container(), 30);
}
