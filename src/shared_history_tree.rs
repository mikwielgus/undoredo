// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec;
use alloc::vec::Vec;

use crate::{ApplyEdit, CmdEdit, ExtractEdit, RevertEdit};

/// Id of a node in [`SharedHistoryTree`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SharedHistoryTreeNodeId(usize);

impl SharedHistoryTreeNodeId {
    /// Creates an id from a node index.
    #[inline]
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying node index.
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

/// A single history node in [`SharedHistoryTree`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SharedHistoryTreeNode<E, Cmd = ()> {
    /// Command metadata together with the recorded edit at this node.
    cmd_edit: CmdEdit<Cmd, E>,
    /// Parent node id. `None` for the root.
    parent: Option<SharedHistoryTreeNodeId>,
    /// Child node ids.
    children: Vec<SharedHistoryTreeNodeId>,
    /// Depth of this node in the tree. Root has depth `0`.
    depth: u64,
}

impl<E, Cmd> SharedHistoryTreeNode<E, Cmd> {
    /// Returns command metadata together with the recorded edit at this node.
    #[inline]
    pub fn cmd_edit(&self) -> &CmdEdit<Cmd, E> {
        &self.cmd_edit
    }

    /// Returns parent node id. `None` for the root.
    #[inline]
    pub fn parent(&self) -> Option<SharedHistoryTreeNodeId> {
        self.parent
    }

    /// Returns child node ids.
    #[inline]
    pub fn children(&self) -> &[SharedHistoryTreeNodeId] {
        &self.children
    }

    /// Returns depth of this node in the tree. Root has depth `0`.
    #[inline]
    pub fn depth(&self) -> u64 {
        self.depth
    }
}

/// A cursor that points to a node and owns the container subject to editing.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SharedHistoryTreeCursor<C> {
    curr_node: SharedHistoryTreeNodeId,
    container: C,
}

impl<C> SharedHistoryTreeCursor<C> {
    /// Returns the node the cursor is attached to.
    #[inline]
    pub fn curr_node(&self) -> SharedHistoryTreeNodeId {
        self.curr_node
    }

    /// Consume the cursor and detach the container, ceding ownership over it.
    #[inline]
    pub fn into_container(self) -> C {
        self.container
    }

    /// Returns an immutable reference to the container owned by the cursor.
    #[inline]
    pub fn container(&self) -> &C {
        &self.container
    }

    /// Returns a mutable reference to the container owned by the cursor.
    #[inline]
    pub fn container_mut(&mut self) -> &mut C {
        &mut self.container
    }
}

impl<C> AsRef<C> for SharedHistoryTreeCursor<C> {
    #[inline]
    fn as_ref(&self) -> &C {
        &self.container
    }
}

/// A history tree for non-linear undo-redo action with multiple possible
/// external cursors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SharedHistoryTree<E, Cmd = ()> {
    nodes: Vec<SharedHistoryTreeNode<E, Cmd>>,
}

impl<Cmd: Default, E: Default> SharedHistoryTree<E, Cmd> {
    /// Create a new history tree with a single, root node.
    #[inline]
    pub fn new() -> Self {
        Self {
            nodes: vec![SharedHistoryTreeNode {
                cmd_edit: CmdEdit::default(), // Empty edit.
                parent: None,
                children: Vec::new(),
                depth: 0,
            }],
        }
    }
}

impl<Cmd, E> SharedHistoryTree<E, Cmd> {
    /// Returns an immutable reference to the node with the given id.
    #[inline]
    pub fn node(&self, id: SharedHistoryTreeNodeId) -> &SharedHistoryTreeNode<E, Cmd> {
        &self.nodes[id.index()]
    }

    /// Returns parent node id of the node with the given id. `None` for the root.
    #[inline]
    pub fn parent(&self, id: SharedHistoryTreeNodeId) -> Option<SharedHistoryTreeNodeId> {
        self.nodes[id.index()].parent
    }

    /// Returns an immutable reference to the command metadata at the given node.
    #[inline]
    pub fn cmd(&self, id: SharedHistoryTreeNodeId) -> &Cmd {
        &self.nodes[id.index()].cmd_edit.cmd
    }

    /// Returns a mutable reference to the command metadata at the given node.
    #[inline]
    pub fn cmd_mut(&mut self, id: SharedHistoryTreeNodeId) -> &mut Cmd {
        &mut self.nodes[id.index()].cmd_edit.cmd
    }

    /// Creates a new cursor to the root that owns the target container.
    #[inline]
    pub fn root_cursor<T>(&self, target: T) -> SharedHistoryTreeCursor<T> {
        self.cursor(SharedHistoryTreeNodeId::new(0), target)
    }

    #[inline]
    fn cursor<T>(
        &self,
        curr_node: SharedHistoryTreeNodeId,
        target: T,
    ) -> SharedHistoryTreeCursor<T> {
        SharedHistoryTreeCursor {
            curr_node,
            container: target,
        }
    }
}

impl<Cmd: Default, E> SharedHistoryTree<E, Cmd> {
    /// Flush the cursor's target container and insert its changes as an edit
    /// into a new tree leaf.
    #[inline]
    pub fn commit<T>(&mut self, cursor: &mut SharedHistoryTreeCursor<T>)
    where
        E: ExtractEdit<T>,
    {
        self.cmd_commit(Default::default(), cursor);
    }
}

impl<Cmd, E> SharedHistoryTree<E, Cmd> {
    /// Flush the cursor's target container and insert its changes into a new
    /// tree leaf as an edit along with additional metadata ("cmd").
    #[inline]
    pub fn cmd_commit<T>(&mut self, cmd: Cmd, cursor: &mut SharedHistoryTreeCursor<T>)
    where
        E: ExtractEdit<T>,
    {
        self.nodes.push(SharedHistoryTreeNode {
            cmd_edit: CmdEdit {
                cmd,
                edit: E::extract_edit(cursor.container_mut()),
            },
            parent: Some(cursor.curr_node),
            children: Vec::new(),
            depth: self.nodes[cursor.curr_node.index()].depth + 1,
        });

        let node_id = SharedHistoryTreeNodeId::new(self.nodes.len() - 1);
        self.nodes[cursor.curr_node.index()].children.push(node_id);

        cursor.curr_node = node_id;
    }
}

impl<Cmd: Clone, E: Clone> SharedHistoryTree<E, Cmd> {
    /// Undo the edit at the cursor's current tree node, moving the cursor one
    /// step upwards to the node's parent.
    ///
    /// This does not cause any changes to the tree, only the target container
    /// and cursor are modified.
    #[inline]
    pub fn undo<T>(&mut self, cursor: &mut SharedHistoryTreeCursor<T>) -> Option<Cmd>
    where
        E: RevertEdit<T>,
    {
        let curr_node = cursor.curr_node;
        let parent_node = self.nodes[curr_node.index()].parent?;
        let CmdEdit { cmd, edit } = self.nodes[curr_node.index()].cmd_edit.clone();
        edit.revert_edit(&mut cursor.container);
        cursor.curr_node = parent_node;

        Some(cmd)
    }

    /// Redo the edit at the specified node, moving the cursor to it one step
    /// downwards.
    ///
    /// This does not cause any changes to the tree, only the target container
    /// and cursor are modified.
    #[inline]
    pub fn redo<T>(
        &mut self,
        cursor: &mut SharedHistoryTreeCursor<T>,
        node: SharedHistoryTreeNodeId,
    ) -> Cmd
    where
        E: ApplyEdit<T> + RevertEdit<T>,
    {
        assert!(
            self.nodes[cursor.curr_node.index()]
                .children
                .contains(&node)
        );

        let CmdEdit { cmd, edit } = self.nodes[node.index()].cmd_edit.clone();
        edit.apply_edit(&mut cursor.container);
        cursor.curr_node = node;

        cmd
    }

    /// Move the cursor from its current node to the target node.
    ///
    /// First, a sequence of calls to [`Self::undo`] is made to climb to the
    /// lowest common ancestor, and then a sequence of calls to [`Self::redo`]
    /// is performed to descend to the target node
    ///
    /// Returns the list of command metadata produced by the redo phase in the
    /// order they were applied. If the cursor is already at `target_node`, an
    /// empty vector is returned.
    #[inline]
    pub fn checkout<T>(
        &mut self,
        cursor: &mut SharedHistoryTreeCursor<T>,
        target_node: SharedHistoryTreeNodeId,
    ) -> Vec<Cmd>
    where
        E: ApplyEdit<T> + RevertEdit<T>,
    {
        let mut source_node = cursor.curr_node;

        let mut target_path = Vec::new();
        let mut source_depth = self.nodes[source_node.index()].depth;
        let mut target_depth = self.nodes[target_node.index()].depth;
        let mut aligned_target = target_node;

        while source_depth > target_depth {
            assert!(self.undo(cursor).is_some());

            source_node = cursor.curr_node;
            source_depth -= 1;
        }

        while target_depth > source_depth {
            target_path.push(aligned_target);
            aligned_target = self.nodes[aligned_target.index()].parent.unwrap();
            target_depth -= 1;
        }

        while source_node != aligned_target {
            assert!(self.undo(cursor).is_some());

            source_node = cursor.curr_node;
            target_path.push(aligned_target);
            aligned_target = self.nodes[aligned_target.index()].parent.unwrap();
        }

        let mut cmds = Vec::new();

        for node in target_path.into_iter().rev() {
            cmds.push(self.redo(cursor, node));
        }

        cmds
    }
}
