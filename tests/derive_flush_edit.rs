// SPDX-FileCopyrightText: 2026 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "derive")]

use std::collections::{BTreeMap, BTreeSet};

use undoredo::{FlushEdit, Recorder};

#[derive(undoredo::FlushEdit, undoredo::CompositeEdit)]
struct FlatStruct {
    map: Recorder<BTreeMap<i32, i32>>,
    set: Recorder<BTreeSet<i32>>,
}

#[test]
fn test_derive_flush_edit_flat_struct() {
    let mut flat_struct = FlatStruct {
        map: Recorder::new(BTreeMap::from([
            (1, 10),
            (2, 20),
            (3, 30),
            (4, 40),
            (5, 50),
        ])),
        set: Recorder::new(BTreeSet::from([10, 20, 30, 40, 50])),
    };

    flat_struct.map.remove(&2);
    flat_struct.map.insert(3, 30);
    flat_struct.map.insert(6, 60);

    flat_struct.set.remove(&20);
    flat_struct.set.insert(30, ());
    flat_struct.set.insert(60, ());

    let edit = flat_struct.flush_edit();
    let (removed, inserted) = edit.dissolve();

    assert_eq!(removed.map, BTreeMap::from([(2, 20), (3, 30)]));
    assert_eq!(inserted.map, BTreeMap::from([(3, 30), (6, 60)]));
    assert_eq!(removed.set, BTreeMap::from([(20, ()), (30, ())]));
    assert_eq!(inserted.set, BTreeMap::from([(30, ()), (60, ())]));
}
