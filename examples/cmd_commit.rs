// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use undoredo::{Insert, Recorder, UndoRedo};

#[derive(Debug, Clone, PartialEq)]
enum Command {
    PushChars,
}

fn main() {
    let mut recorder: Recorder<usize, char, HashMap<usize, char>> = Recorder::new(HashMap::new());
    let mut undoredo: UndoRedo<HashMap<usize, char>, Command> = UndoRedo::new();

    // Push an element while recording.
    recorder.insert(1, 'A');

    // Commit `Command::PushChar` enum variant as command metadata ("cmd") along
    // with the recorded edit.
    undoredo.cmd_commit(Command::PushChars, recorder.flush());

    // `Command::PushChar` is now the top element of the stack of done cmd-edits.
    assert_eq!(undoredo.done().last().unwrap().cmd, Command::PushChars);

    // Now undo the action.
    undoredo.undo(&mut recorder);

    // `Command::PushChar` is now the top element of the stack of undone cmd-edits.
    assert_eq!(undoredo.undone().last().unwrap().cmd, Command::PushChars);
}
