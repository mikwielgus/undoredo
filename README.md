<!--
SPDX-FileCopyrightText: 2025 undoredo Developers

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# undoredo

`undoredo` is an undo-redo library that works by wrapping a collection inside a
decorator that observes the incoming insertions, removals, and pushes, recording
the changes in a reversible incremental diff structure.

This approach makes `undoredo` easier to use than other undo-redo libraries.
Storing incremental diffs is much more succint and reliable than the
commonly used [Command pattern](https://en.wikipedia.org/wiki/Command_pattern).
This is because the Command pattern requires implementing custom commands
and their behavior using traits, which results in having to maintain verbose,
application-specific logic that is prone to elusive runtime bugs.

## Basic usage

```rust
use std::collections::HashMap;
use undoredo::{Insert, Recorder, UndoRedo};

#[test]
fn main() {
    let mut recorder: Recorder<usize, char, HashMap<usize, char>> = Recorder::new(HashMap::new());
    let mut undoredo: UndoRedo<HashMap<usize, char>> = UndoRedo::new();

    // Push elements while recording this into an action.
    recorder.insert(1, 'A');
    recorder.insert(2, 'B');
    recorder.insert(3, 'C');

    // Commit the recorded action of pushing ['A', 'B', 'C'] into the undo-redo
    // history.
    undoredo.commit(recorder.flush());

    // The pushed elements are now present in the collection.
    assert!(*recorder.collection() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Now undo the action.
    undoredo.undo(&mut recorder);

    // The collection is now empty; the action of pushing elements has been undone.
    assert!(*recorder.collection() == HashMap::from([]));

    // Now redo the action.
    undoredo.redo(&mut recorder);

    // The elements are back in the collection; the action has been redone.
    assert!(*recorder.collection() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));
}
```

## Supported collections

### Standard library

Standard library maps
[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and
[BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) are
supported via built-in implementations. You can disable them by turning off the
default `std` feature.

### Foreign implementations

In addition to the standard library, `undoredo` is implemented for data
structures from some external crates:

- [StableVec](https://docs.rs/stable-vec/latest/stable_vec/) behind the `stable-vec` feature.
- [thunderdome::Arena](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html) behind the `thunderdome` feature.

Unlike maps, which support insertion and removal under arbitrary keys, a
stable-vec-style data structure can be only supported if it has an interface
to add values at indexes that are equal or greater than the current length. For
`StableVec`, this is achieved by changing the length before insertion using the
[.reserve_for](https://docs.rs/stable-vec/latest/stable_vec/struct.StableVecFacade.html#method.reserve_for)
method. In `thunderdome::Arena`, this is achieved by inserting via the
[.insert_at](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html#method.insert_at)
method.

## Unsupported collections

[Slab](https://docs.rs/slab/latest/slab/) and
[SlotMap](https://docs.rs/slotmap/latest/slotmap/) cannot be supported because
they do not have an interface to insert values at indexes that are equal or
greater than the current length.

## Contributing

We welcome issues and pull requests from anyone both to our canonical
[repository](https://codeberg.org/topola/undoredo) on Codeberg and to our GitHub
[mirror](https://github.com/mikwielgus/undoredo).

## Licence

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
