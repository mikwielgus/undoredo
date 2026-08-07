// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use undoredo::aliases::VecDelta;
use undoredo::{HistoryTree, Recorder};

fn main() {
    let mut history_tree: HistoryTree<VecDelta<&str>> = HistoryTree::new();
    let mut state = Recorder::new(vec!["root"]);

    // Right below the root we push a "fork" node, from which we will fork two
    // branches.
    state.push("fork");
    history_tree.commit(&mut state);

    let fork = history_tree.curr_node();

    assert_eq!(*state.as_ref(), vec!["root", "fork"]);

    // Add first node of left branch.
    state.push("left_branch_1");
    history_tree.commit(&mut state);

    let left_branch_1 = history_tree.curr_node();

    // Add second node of left branch.
    state.push("left_branch_2");
    history_tree.commit(&mut state);

    let left_branch_2 = history_tree.curr_node();

    // Now we have assembled the whole of the left branch.
    assert_eq!(
        *state.as_ref(),
        vec!["root", "fork", "left_branch_1", "left_branch_2"]
    );

    // Let's now undo the whole left branch up to the fork node.
    history_tree.undo(&mut state);
    history_tree.undo(&mut state);

    assert_eq!(*state.as_ref(), vec!["root", "fork"]);
    assert_eq!(history_tree.curr_node(), fork);

    // Add first node of right branch.
    state.push("right_branch_1");
    history_tree.commit(&mut state);

    let right_branch_1 = history_tree.curr_node();

    // Add second node of right branch.
    state.push("right_branch_2");
    history_tree.commit(&mut state);

    let right_branch_2 = history_tree.curr_node();

    // Now we have assembled the whole of the right branch.
    assert_eq!(
        *state.as_ref(),
        vec!["root", "fork", "right_branch_1", "right_branch_2"]
    );

    let root_children = history_tree
        .node(history_tree.node(fork).parent().unwrap())
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
    history_tree.checkout(&mut state, left_branch_2);

    assert_eq!(history_tree.curr_node(), left_branch_2);
    assert_eq!(
        *state.as_ref(),
        vec!["root", "fork", "left_branch_1", "left_branch_2"]
    );

    // Check out the right branch from the apex of the left branch.
    history_tree.checkout(&mut state, right_branch_2);

    assert_eq!(history_tree.curr_node(), right_branch_2);
    assert_eq!(
        *state.as_ref(),
        vec!["root", "fork", "right_branch_1", "right_branch_2"]
    );

    // Let's now undo the whole right branch up to the fork node.
    history_tree.undo(&mut state);
    history_tree.undo(&mut state);

    assert_eq!(*state.as_ref(), vec!["root", "fork"]);
    assert_eq!(history_tree.curr_node(), fork);
}

#[test]
fn test() {
    main();
}
