// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::{BTreeMap, BTreeSet};

use undoredo::{ApplyEdit, Edit};

#[derive(Clone, undoredo::ApplyEdit)]
struct State {
    map: BTreeMap<i32, i32>,
    set: BTreeSet<i32>,
}

#[test]
fn test_derive_apply_edit_on_flat_struct() {
    let mut state = State {
        map: BTreeMap::from([(1, 10), (2, 20), (3, 30), (4, 40), (5, 50)]),
        set: BTreeSet::from([10, 20, 30, 40, 50]),
    };

    let edit = Edit::with_removed_inserted(
        State {
            map: BTreeMap::from([(2, 20)]),
            set: BTreeSet::from([20]),
        },
        State {
            map: BTreeMap::from([(3, 33), (6, 60)]),
            set: BTreeSet::from([30, 60]),
        },
    );

    state.apply_edit(&edit);

    assert_eq!(state.map.get(&1), Some(&10));
    assert_eq!(state.map.get(&2), None);
    assert_eq!(state.map.get(&3), Some(&33));
    assert_eq!(state.map.get(&4), Some(&40));
    assert_eq!(state.map.get(&5), Some(&50));
    assert_eq!(state.map.get(&6), Some(&60));

    assert!(state.set.contains(&10));
    assert!(!state.set.contains(&20));
    assert!(state.set.contains(&30));
    assert!(state.set.contains(&40));
    assert!(state.set.contains(&50));
    assert!(state.set.contains(&60));
}
