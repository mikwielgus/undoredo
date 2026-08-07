// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec;
use alloc::vec::Vec;

use crate::{ApplyEdit, CmdEdit, ExtractEdit, RevertEdit};

/// Id of a node in [`HistoryTree`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HistoryTreeNodeId(usize);

impl HistoryTreeNodeId {
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

/// A single history node in [`HistoryTree`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HistoryTreeNode<E, Cmd = ()> {
    /// Command metadata together with the recorded edit at this node.
    cmd_edit: CmdEdit<Cmd, E>,
    /// Parent node id. `None` for the root.
    parent: Option<HistoryTreeNodeId>,
    /// Child node ids.
    children: Vec<HistoryTreeNodeId>,
    /// Depth of this node in the tree. Root has depth `0`.
    depth: u64,
}

impl<E, Cmd> HistoryTreeNode<E, Cmd> {
    /// Returns command metadata together with the recorded edit at this node.
    #[inline]
    pub fn cmd_edit(&self) -> &CmdEdit<Cmd, E> {
        &self.cmd_edit
    }

    /// Returns parent node id. `None` for the root.
    #[inline]
    pub fn parent(&self) -> Option<HistoryTreeNodeId> {
        self.parent
    }

    /// Returns child node ids.
    #[inline]
    pub fn children(&self) -> &[HistoryTreeNodeId] {
        &self.children
    }

    /// Returns depth of this node in the tree. Root has depth `0`.
    #[inline]
    pub fn depth(&self) -> u64 {
        self.depth
    }
}

/// A history tree for non-linear undo-redo action.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HistoryTree<E, Cmd = ()> {
    nodes: Vec<HistoryTreeNode<E, Cmd>>,
    curr_node: HistoryTreeNodeId,
}

impl<Cmd: Default, E: Default> HistoryTree<E, Cmd> {
    /// Create a new history tree.
    #[inline]
    pub fn new() -> Self {
        Self {
            nodes: vec![HistoryTreeNode {
                cmd_edit: CmdEdit::default(), // Empty edit.
                parent: None,
                children: Vec::new(),
                depth: 0,
            }],
            curr_node: HistoryTreeNodeId::new(0),
        }
    }
}

impl<Cmd, E> HistoryTree<E, Cmd> {
    /// Returns the id of the current node.
    #[inline]
    pub fn curr_node(&self) -> HistoryTreeNodeId {
        self.curr_node
    }

    /// Returns an immutable reference to the node with the given id.
    #[inline]
    pub fn node(&self, id: HistoryTreeNodeId) -> &HistoryTreeNode<E, Cmd> {
        &self.nodes[id.index()]
    }

    /// Returns parent node id of the node with the given id. `None` for the root.
    #[inline]
    pub fn parent(&self, id: HistoryTreeNodeId) -> Option<HistoryTreeNodeId> {
        self.nodes[id.index()].parent
    }

    /// Returns an immutable reference to the command metadata at the given node.
    #[inline]
    pub fn cmd(&self, id: HistoryTreeNodeId) -> &Cmd {
        &self.nodes[id.index()].cmd_edit.cmd
    }

    /// Returns a mutable reference to the command metadata at the given node.
    #[inline]
    pub fn cmd_mut(&mut self, id: HistoryTreeNodeId) -> &mut Cmd {
        &mut self.nodes[id.index()].cmd_edit.cmd
    }
}

impl<Cmd: Default, E> HistoryTree<E, Cmd> {
    /// Flush the target container and insert its changes as an edit into a new
    /// tree leaf under the current node.
    #[inline]
    pub fn commit<T>(&mut self, target: &mut T)
    where
        E: ExtractEdit<T>,
    {
        self.cmd_commit(Default::default(), target);
    }
}

impl<Cmd, E> HistoryTree<E, Cmd> {
    /// Flush the target container and insert its changes into a new tree leaf
    /// under the current node as an edit along with additional metadata
    /// ("cmd").
    #[inline]
    pub fn cmd_commit<T>(&mut self, cmd: Cmd, target: &mut T)
    where
        E: ExtractEdit<T>,
    {
        self.nodes.push(HistoryTreeNode {
            cmd_edit: CmdEdit {
                cmd,
                edit: E::extract_edit(target),
            },
            parent: Some(self.curr_node),
            children: Vec::new(),
            depth: self.nodes[self.curr_node.index()].depth + 1,
        });

        let node_id = HistoryTreeNodeId::new(self.nodes.len() - 1);
        self.nodes[self.curr_node.index()].children.push(node_id);

        self.curr_node = node_id;
    }
}

impl<Cmd: Clone, E: Clone> HistoryTree<E, Cmd> {
    /// Undo the edit at the current tree node, moving one step upwards to the
    /// node's parent.
    ///
    /// This does not cause any changes to the tree topology, only the target
    /// container and current node are modified.
    #[inline]
    pub fn undo<T>(&mut self, target: &mut T) -> Option<Cmd>
    where
        E: RevertEdit<T>,
    {
        let curr_node = self.curr_node;
        let parent_node = self.nodes[curr_node.index()].parent?;
        let CmdEdit { cmd, edit } = self.nodes[curr_node.index()].cmd_edit.clone();
        edit.revert_edit(target);
        self.curr_node = parent_node;

        Some(cmd)
    }

    /// Redo the edit at the specified node, moving one step downwards.
    ///
    /// This does not cause any changes to the tree topology, only the target
    /// container and current node are modified.
    #[inline]
    pub fn redo<T>(&mut self, target: &mut T, node: HistoryTreeNodeId) -> Cmd
    where
        E: ApplyEdit<T> + RevertEdit<T>,
    {
        assert!(self.nodes[self.curr_node.index()].children.contains(&node));

        let CmdEdit { cmd, edit } = self.nodes[node.index()].cmd_edit.clone();
        edit.apply_edit(target);
        self.curr_node = node;

        cmd
    }

    /// Move from the current node to the target node.
    ///
    /// First, a sequence of calls to [`Self::undo`] is made to climb to the
    /// lowest common ancestor, and then a sequence of calls to [`Self::redo`]
    /// is performed to descend to the target node.
    ///
    /// Returns the list of command metadata produced by the redo phase in the
    /// order they were applied. If already at `target_node`, an empty vector is
    /// returned.
    #[inline]
    pub fn checkout<T>(&mut self, target: &mut T, target_node: HistoryTreeNodeId) -> Vec<Cmd>
    where
        E: ApplyEdit<T> + RevertEdit<T>,
    {
        let mut source_node = self.curr_node;

        let mut target_path = Vec::new();
        let mut source_depth = self.nodes[source_node.index()].depth;
        let mut target_depth = self.nodes[target_node.index()].depth;
        let mut aligned_target = target_node;

        while source_depth > target_depth {
            assert!(self.undo(target).is_some());

            source_node = self.curr_node;
            source_depth -= 1;
        }

        while target_depth > source_depth {
            target_path.push(aligned_target);
            aligned_target = self.nodes[aligned_target.index()].parent.unwrap();
            target_depth -= 1;
        }

        while source_node != aligned_target {
            assert!(self.undo(target).is_some());

            source_node = self.curr_node;
            target_path.push(aligned_target);
            aligned_target = self.nodes[aligned_target.index()].parent.unwrap();
        }

        let mut cmds = Vec::new();

        for node in target_path.into_iter().rev() {
            cmds.push(self.redo(target, node));
        }

        cmds
    }
}
