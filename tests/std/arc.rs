// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "std")]

use std::sync::Arc;

use undoredo::aliases::ArcHalfDelta;
use undoredo::{ApplyDelta, Delta, Recorder};

#[test]
fn test_arc_apply_delta_replace() {
    let mut value = Arc::new(1);

    value.apply_delta(Delta::with_removed_inserted(
        ArcHalfDelta::from([(0, 1)]),
        ArcHalfDelta::from([(0, 2)]),
    ));

    assert_eq!(*value, 2);
}

#[test]
fn test_arc_recorder() {
    let mut recorder = Recorder::<Arc<i32>>::new(Arc::new(10));

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
