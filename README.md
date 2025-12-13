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
The programmer is relieved from having to maintain application-specific
implementations of commands, often complicated and prone to elusive runtime
bugs, on which the Command pattern operates.

## Basic usage

First, add `undoredo` to your `Cargo.toml`:

```
[dependencies]
undoredo = "0.1"
```

Following is a basic usage example of `undoredo` over
`std::collections::HashMap`. You can find more examples in our
[examples/](./examples) directory.

```rust
use std::collections::HashMap;
use undoredo::{Insert, Recorder, UndoRedo};

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

    // Once you are done recording, you can dissolve the recorder to regain
    // ownership and mutability over the underlying collection.
    let (mut hashmap, ..) = recorder.dissolve();
    assert!(hashmap == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));
}
```

## Supported collections

### Standard library

Standard library maps
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and
[`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) are
supported via built-in implementations. You can disable them by turning off the
default `std` feature.

### Foreign implementations

In addition to the standard library, `undoredo` has feature-gated convenience
implementations for data structures from some external crates:

- [`StableVec`](https://docs.rs/stable-vec/latest/stable_vec/)
  behind the `stable-vec` feature. (example usage:
  [examples/stable_vec.rs](./examples/stable_vec.rs))
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html)
  behind the `thunderdome` feature. (example usage:
  [examples/thunderdome.rs](./examples/thunderdome.rs))

To use these, specify them next to your `undoredo` dependency in your
`Cargo.toml`. For example, to enable all foreign implementations, write

```
[dependencies]
undoredo = { version = "0.1", features = ["stable-vec", "thunderdome"]}
```

**Technical detail:** Unlike maps, which support insertion and removal at
arbitrary keys, a stable-vec-style data structure can be only supported if
it allows to insert elements at arbitrary indexes, including indexes that are
out of bounds at the time of insertion. For `StableVec`, this is achieved by
changing the length before insertion using the
[`.reserve_for()`](https://docs.rs/stable-vec/latest/stable_vec/struct.StableVecFacade.html#method.reserve_for)
method. With `thunderdome::Arena`, this is achieved directly by inserting via the
[`.insert_at()`](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html#method.insert_at)
method.

## Unsupported collections

[`Slab`](https://docs.rs/slab/latest/slab/),
[`SlotMap`](https://docs.rs/slotmap/latest/slotmap/),
[`generational-arena`](https://docs.rs/generational-arena/latest/generational_arena/)
cannot be supported because they lack interfaces for insertion at an arbitrary
key.

**Technical detail:** For `Slab`, this is apparently
[because](https://github.com/tokio-rs/slab/issues/117#issuecomment-1159741097)
the [freelist](https://en.wikipedia.org/wiki/Free_list) `Slab` uses to keep
track of its vacant indexes is only singly-linked, not doubly-linked. Inserting
an element at an arbitrary vacant index would require removing that index from
the freelist. But since there is no backwards link available at a given key,
doing so would require traversing the freelist from the beginning to find the
position of the previous node, which would incur an overly slow `O(n)` time
cost.

## Contributing

We welcome issues and pull requests from anyone both to our canonical
[repository](https://codeberg.org/topola/undoredo) on Codeberg and to our GitHub
[mirror](https://github.com/mikwielgus/undoredo).

## Licence

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
