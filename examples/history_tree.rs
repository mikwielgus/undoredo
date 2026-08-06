// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use undoredo::aliases::VecDelta;
use undoredo::{HistoryTree, Recorder};

fn main() {
    let mut history_tree: HistoryTree<VecDelta<&str>> = HistoryTree::new();
    // History tree can have an arbitrary number of cursors. Each cursor holds a
    // node id and its own copy of the working state.
    let mut cursor = history_tree.root_cursor(Recorder::new(vec!["root"]));

    // Right below the root we push a "fork" node, from which we will fork two
    // branches.
    cursor.container_mut().push("fork");
    history_tree.commit(&mut cursor);

    let fork = cursor.curr_node();

    assert_eq!(*cursor.container().as_ref(), vec!["root", "fork"]);

    // Add first node of left branch.
    cursor.container_mut().push("left_branch_1");
    history_tree.commit(&mut cursor);

    let left_branch_1 = cursor.curr_node();

    // Add second node of left branch.
    cursor.container_mut().push("left_branch_2");
    history_tree.commit(&mut cursor);

    let left_branch_2 = cursor.curr_node();

    // Now we have assembled the whole of the left branch.
    assert_eq!(
        *cursor.container().as_ref(),
        vec!["root", "fork", "left_branch_1", "left_branch_2"]
    );

    // Let's now undo the whole left branch up to the fork node.
    history_tree.undo(&mut cursor);
    history_tree.undo(&mut cursor);

    assert_eq!(*cursor.container().as_ref(), vec!["root", "fork"]);
    assert_eq!(cursor.curr_node(), fork);

    // Add first node of right branch.
    cursor.container_mut().push("right_branch_1");
    history_tree.commit(&mut cursor);

    let right_branch_1 = cursor.curr_node();

    // Add second node of right branch.
    cursor.container_mut().push("right_branch_2");
    history_tree.commit(&mut cursor);

    let right_branch_2 = cursor.curr_node();

    // Now we have assembled the whole of the right branch.
    assert_eq!(
        *cursor.container().as_ref(),
        vec!["root", "fork", "right_branch_1", "right_branch_2"]
    );

    let root_children = history_tree
        .node(history_tree.node(fork).parent())
        .children();
    assert_eq!(root_children, &[fork]);

    assert_eq!(
        history_tree.node(fork).children(),
        &[left_branch_1, right_branch_1]
    );
    assert_eq!(
        history_tree.node(left_branch_1).children(),
        &[left_branch_2]
    );
    assert_eq!(
        history_tree.node(right_branch_1).children(),
        &[right_branch_2]
    );

    // Check out left branch from the apex of the right branch.
    history_tree.checkout(&mut cursor, left_branch_2);

    assert_eq!(cursor.curr_node(), left_branch_2);
    assert_eq!(
        *cursor.container().as_ref(),
        vec!["root", "fork", "left_branch_1", "left_branch_2"]
    );

    // Check out the right branch from the apex of the left branch.
    history_tree.checkout(&mut cursor, right_branch_2);

    assert_eq!(cursor.curr_node(), right_branch_2);
    assert_eq!(
        *cursor.container().as_ref(),
        vec!["root", "fork", "right_branch_1", "right_branch_2"]
    );

    // Let's now undo the whole right branch up to the fork node.
    history_tree.undo(&mut cursor);
    history_tree.undo(&mut cursor);

    assert_eq!(*cursor.container().as_ref(), vec!["root", "fork"]);
    assert_eq!(cursor.curr_node(), fork);
}

#[test]
fn test() {
    main();
}
