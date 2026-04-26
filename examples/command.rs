// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Example showing Command pattern using `undoredo` crate.

use undoredo::UndoRedo;

// Commands that we store in the undo-redo bistack.
#[derive(Debug, Clone, PartialEq)]
enum Command {
    PushChar(char),
}

/// Execute command and store it in the undo-redo bistack.
fn command(s: &mut String, undoredo: &mut UndoRedo<(), Command>, cmd: Command) {
    // Since there are no stored edits (deltas or snapshots), it is now the
    // responsibility of the library user to implement application logic.

    // Manually implemented application logic.
    match &cmd {
        Command::PushChar(c) => s.push(*c),
    }

    // Same as `.cmd_commit(cmd, ())`.
    undoredo.command(cmd);
}

/// Undo the latest command.
fn undo(s: &mut String, undoredo: &mut UndoRedo<(), Command>) {
    if let Some(cmd) = undoredo.undo(s) {
        // Manually implemented application logic.
        match &cmd {
            Command::PushChar(_) => {
                s.pop();
            }
        }
    }
}

/// Redo the last undone command.
fn redo(s: &mut String, undoredo: &mut UndoRedo<(), Command>) {
    if let Some(cmd) = undoredo.redo(s) {
        // Manually implemented application logic.
        match &cmd {
            Command::PushChar(c) => s.push(*c),
        }
    }
}

fn main() {
    let mut s = String::new();
    let mut undoredo: UndoRedo<(), Command> = UndoRedo::new();

    command(&mut s, &mut undoredo, Command::PushChar('a'));
    command(&mut s, &mut undoredo, Command::PushChar('b'));

    assert_eq!(s, "ab");

    undo(&mut s, &mut undoredo);
    assert_eq!(s, "a");

    undo(&mut s, &mut undoredo);
    assert_eq!(s, "");

    redo(&mut s, &mut undoredo);
    assert_eq!(s, "a");

    redo(&mut s, &mut undoredo);
    assert_eq!(s, "ab");
}

#[test]
fn test() {
    main();
}
