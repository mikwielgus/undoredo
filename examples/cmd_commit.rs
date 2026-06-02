// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Example showing how to store metadata ("cmd") along with each edit.

use std::collections::HashMap;
use undoredo::aliases::{HashMapDelta, HashMapHalfDelta};
use undoredo::{Recorder, UndoRedo};

/// Representation of the command that originated the recorded delta.
#[derive(Debug, Clone, PartialEq)]
enum Command {
    PushChar,
}

fn main() {
    let mut recorder: Recorder<HashMap<usize, char>, HashMapHalfDelta<usize, char>> =
        Recorder::new(HashMap::new());
    let mut undoredo: UndoRedo<HashMapDelta<usize, char>, Command> = UndoRedo::new();

    recorder.insert(1, 'A');

    // Commit `Command::PushChar` enum variant as command metadata ("cmd") along
    // with the recorded delta.
    undoredo.cmd_commit(Command::PushChar, &mut recorder);

    // `Command::PushChar` is now the top element of the stack of done cmd-deltas.
    assert_eq!(undoredo.done().last().unwrap().cmd, Command::PushChar);

    undoredo.undo(&mut recorder);

    // After undo, `Command::PushChar` is now the top element of the stack of
    // undone cmd-deltas.
    assert_eq!(undoredo.undone().last().unwrap().cmd, Command::PushChar);
}

#[test]
fn test() {
    main();
}
