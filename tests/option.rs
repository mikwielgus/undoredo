// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use undoredo::aliases::OptionHalfDelta;
use undoredo::{ApplyDelta, Delta, Recorder};

#[test]
fn test_option_apply_delta_replace() {
    let mut value = Some(1);

    value.apply_delta(Delta::with_removed_inserted(
        OptionHalfDelta::from([(0, 1)]),
        OptionHalfDelta::from([(0, 2)]),
    ));

    assert_eq!(value, Some(2));
}

#[test]
fn test_option_apply_delta_clear() {
    let mut value = Some(1);

    value.apply_delta(Delta::with_removed_inserted(
        OptionHalfDelta::from([(0, 1)]),
        OptionHalfDelta::new(),
    ));

    assert_eq!(value, None);
}

#[test]
fn test_option_recorder() {
    let mut recorder = Recorder::<Option<i32>>::new(None);

    recorder.set(0, 10);
    assert_eq!(*recorder.container(), Some(10));

    let delta = recorder.flush_delta();
    recorder.apply_delta(delta.reverse());
    assert_eq!(*recorder.container(), None);

    recorder.set(0, 20);
    let _ = recorder.flush_delta();
    recorder.remove(&0);
    assert_eq!(*recorder.container(), None);

    let delta = recorder.flush_delta();
    recorder.apply_delta(delta.reverse());

    assert_eq!(*recorder.container(), Some(20));
}
