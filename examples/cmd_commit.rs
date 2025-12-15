// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use undoredo::{Insert, Recorder, UndoRedo};

// Representation of the command that originated the recorded edit.
#[derive(Debug, Clone, PartialEq)]
enum Command {
    PushChars,
}

fn main() {
    let mut recorder: Recorder<usize, char, HashMap<usize, char>> = Recorder::new(HashMap::new());
    let mut undoredo: UndoRedo<HashMap<usize, char>, Command> = UndoRedo::new();

    recorder.insert(1, 'A');

    // Commit `Command::PushChar` enum variant as command metadata ("cmd") along
    // with the recorded edit.
    undoredo.cmd_commit(Command::PushChars, recorder.flush());

    // `Command::PushChar` is now the top element of the stack of done cmd-edits.
    assert_eq!(undoredo.done().last().unwrap().cmd, Command::PushChars);

    undoredo.undo(&mut recorder);

    // After undo, `Command::PushChar` is now the top element of the stack of
    // undone cmd-edits.
    assert_eq!(undoredo.undone().last().unwrap().cmd, Command::PushChars);
}

#[test]
fn test() {
    main();
}
