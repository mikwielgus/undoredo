// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::undoredo::{Extract, Revert};

/// A snapshot of the whole state of a target.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot<S> {
    state: S,
}

impl<S> Snapshot<S> {
    /// Create a new snapshot from a state of a target.
    pub fn new(state: S) -> Self {
        Self { state }
    }

    /// Dissolve the snapshot, returning the state it was holding.
    pub fn dissolve(self) -> S {
        self.state
    }
}

impl<T: Clone> Revert<T> for Snapshot<T> {
    fn revert(self, target: &mut T) -> Self {
        let reverse = Self::new(target.clone());
        *target = self.state;

        reverse
    }
}

impl<T: Clone> Extract<T> for Snapshot<T> {
    fn extract(target: &mut T) -> Self {
        Self::new(target.clone())
    }
}
