<!--
SPDX-FileCopyrightText: 2025 undoredo contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# undoredo

`undoredo` is an undo-redo Rust library that works by wrapping a collection
inside a recorder decorator that observes the incoming insertions, removals and
pushes while recording the changes in a reversible incremental diff structure.

This decorator approach makes `undoredo` easier to use than other
undo-redo libraries. Storing incremental diffs typically results in
much more succint and reliable code than the commonly used [Command
pattern](https://en.wikipedia.org/wiki/Command_pattern), which is what
the popular and venerable [`undo`](https://docs.rs/undo/latest/undo/) and
[`undo_2`](https://docs.rs/undo_2/latest/undo_2/) crates use. The programmer
is relieved from having to maintain application-specific implementations of
commands, often complicated and prone to elusive runtime bugs, on which the
Command pattern has to operate.

This library is `no_std`-compatible and has no mandatory dependencies except
for [`alloc`](https://doc.rust-lang.org/alloc/index.html). For ease of use,
`undoredo` has convenience implementations for standard library collections,
`HashMap` and `BTreeMap`, and for some foreign feature-gated types,
`StableVec`, `thunderdome::Arena` and `rstar::RTree`  (read more in [Supported
collections](#supported-collections) section).

## Usage

First, add `undoredo` to your `Cargo.toml`:

```
[dependencies]
undoredo = "0.2"
```

### Basic usage

Following is a basic usage example of `undoredo` over
`std::collections::HashMap`. You can find more examples in the
[examples/](./examples) directory.

```rust
use std::collections::HashMap;
use undoredo::{Insert, Recorder, UndoRedo};

#[allow(unused_mut)]
fn main() {
    // The recorder records the ongoing changes to the underlying collection.
    let mut recorder: Recorder<usize, char, HashMap<usize, char>> = Recorder::new(HashMap::new());

    // The undo-redo struct maintains the undo-redo bistack.
    let mut undoredo: UndoRedo<HashMap<usize, char>> = UndoRedo::new();

    // Push elements while recording the changes in an edit.
    recorder.insert(1, 'A');
    recorder.insert(2, 'B');
    recorder.insert(3, 'C');

    // Flush the recorder and commit the recorded edit of pushing 'A', 'B', 'C'
    // into the undo-redo history.
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

### Storing and accessing command metadata along with edits

It is often desirable to store some metadata along with every recorded edit,
usually a representation of the command that originated it. This can be done by
instead committing the edit using the `.cmd_commit()` method.

The bistack of done and undone committed edits, together with their command
metadatas ("cmd") if present, can be accessed as slices from the `.done()` and
`.undone()` accessor methods.

```rust
use std::collections::HashMap;
use undoredo::{Insert, Recorder, UndoRedo};

// Representation of the command that originated the recorded edit.
#[derive(Debug, Clone, PartialEq)]
enum Command {
    PushChar,
}

fn main() {
    let mut recorder: Recorder<usize, char, HashMap<usize, char>> = Recorder::new(HashMap::new());
    let mut undoredo: UndoRedo<HashMap<usize, char>, Command> = UndoRedo::new();

    recorder.insert(1, 'A');
    recorder.insert(2, 'B');

    // Commit `Command::PushChar` enum variant as command metadata ("cmd") along
    // with the recorded edit.
    undoredo.cmd_commit(Command::PushChar, recorder.flush());

    // `Command::PushChar` is now the top element of the stack of done cmd-edits.
    assert_eq!(undoredo.done().last().unwrap().cmd, Command::PushChar);

    undoredo.undo(&mut recorder);

    // After undo, `Command::PushChar` is now the top element of the stack of
    // undone cmd-edits.
    assert_eq!(undoredo.undone().last().unwrap().cmd, Command::PushChar);
}
```

### Undo-redo on sets

Some data structures have set semantics: they operate only on values, without
exposing any usable notion of key or index. `undoredo` can provide its
functionality to a set by treating it as a `()`-valued map whose keys are the
set's values. This is actually also how Rust's standard library
[represents](https://docs.rs/hashbrown/latest/src/hashbrown/set.rs.html#115)
`HashSet`
[and](https://doc.rust-lang.org/stable/src/alloc/collections/btree/set.rs.html#82)
`BTreeSet` internally.

As an example, the following code will construct a recorder and an undo-redo
bistack for a `BTreeSet`:

```rust
let mut recorder: Recorder<usize, char, BTreeSet<char, ()>> = Recorder::new(BTreeSet::new());
let mut undoredo: UndoRedo<BTreeSet<char, ()>> = UndoRedo::new();
```

Keeping in mind to pass values as keys, `recorder` and
`undoredo` can then be used the same way as with maps above. See
[examples/btreeset.rs](./examples/btreeset.rs) for a complete example.

Among the supported foreign types, `rstar::RTree` is an instance of a
data structure with a convenience implementation over set semantics. See
[examples/rstar.rs](./examples/rstar.rs) for an example of its usage.

**NOTE:** Some set-like data structures are actually multisets: they allow
to insert the same value multiple times without overriding the first one. In
fact, `rstar::RTree` is a multiset. `undoredo` will work correctly with such
data structures, seeing them as sets, but only if you never make use of their
multiset property: you must never insert a value to the collection that is
already there.

## Supported collections

### Standard library

Rust's standard library maps and sets are supported via built-in convenience
implementations:

- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) behind the `std` feature (enabled by default),
- [`HashSet`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html) behind the `std` feature (enabled by default),
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) (not feature-gated),
- [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html) (not feature-gated).

### Foreign types

In addition to the standard library, `undoredo` has built-in feature-gated
convenience implementations for data structures from certain external crates:

- [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/)
  behind the `stable-vec` feature. (example usage:
  [examples/stable_vec.rs](./examples/stable_vec.rs)),
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/)
  behind the `thunderdome` feature. (example usage:
  [examples/thunderdome.rs](./examples/thunderdome.rs)).
- [`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/index.html) behind the
  `rstar` feature. (example usage: [examples/rstar.rs](./examples/rstar.rs)).

To use these, enabled their features next to your `undoredo` dependency in your
`Cargo.toml`. For example, to enable all foreign type implementations, write

```
[dependencies]
undoredo = { version = "0.2", features = ["stable-vec", "thunderdome", "rstar"]}
```

**Technical sidenote:** Unlike maps and sets, not all stable vec data structures
allow insertion and removal at arbitrary indexes regardless of whether they are
vacant, occupied or out of bounds at the time of insertion. For `StableVec`, we
managed to implement inserting at out-of-bound indexes by changing the length
before insertion using the
[`.reserve_for()`](https://docs.rs/stable-vec/latest/stable_vec/struct.StableVecFacade.html#method.reserve_for)
method. For `thunderdome::Arena`, we insert at arbitrary key directly via the
[`.insert_at()`](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html#method.insert_at)
method. Collections for which we could not achieve this are documented in the
section below.

## Unsupported collections

[`Slab`](https://docs.rs/slab/latest/slab/),
[`SlotMap`](https://docs.rs/slotmap/latest/slotmap/),
[`generational-arena`](https://docs.rs/generational-arena/latest/generational_arena/)
cannot be supported because they lack interfaces for insertion at an arbitrary
key.

**Technical sidenote:** For `Slab`, such interface is missing apparently
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

`undoredo` is dual-licensed as under either of

- [MIT license](./LICENSES/MIT.txt),
- [Apache License, Version 2.0](./LICENSES/Apache-2.0.txt).

at your option.
