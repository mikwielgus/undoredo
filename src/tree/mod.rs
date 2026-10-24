// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod edit_tree;
mod shared_edit_tree;

pub use edit_tree::{EditTree, EditTreeNode, EditTreeNodeId};
pub use shared_edit_tree::{
    SharedEditTree, SharedEditTreeCursor, SharedEditTreeNode, SharedEditTreeNodeId,
};
