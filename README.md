<!--
SPDX-FileCopyrightText: 2025 undoredo contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Docs](https://docs.rs/undoredo/badge.svg)](https://docs.rs/undoredo/)
[![Crates.io](https://img.shields.io/crates/v/undoredo.svg)](https://crates.io/crates/undoredo)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/undoredo.svg)](#licence)

# undoredo

`undoredo` is a Rust library that implements the Undo/Redo pattern on
arbitrary data structures by automatically recording sparse *deltas* (aka.
*diffs*, *patches*) of changes, or whole *snapshots* of past states ([Memento
pattern](https://en.wikipedia.org/wiki/Memento_pattern)). These edits can then
be stored in linear undo-redo bistack or non-linear history tree provided by
the library.

This approach is much easier than the commonly used [Command
pattern](https://en.wikipedia.org/wiki/Command_pattern), which is commonly
the principle of operation of other Undo/Redo crates, as having to implement
commands requires maintenance of additional application logic that is often
complicated and can lead to elusive bugs. But if needed, `undoredo` can also
store a command or other metadata along with every edit, allowing for easy use
of the Command pattern as well.

Delta-recording undo-redo requires creating a separate delta edit type for each
data structure. For ease of use, `undoredo` has derive macro
[`#[derive(Delta)]`](https://docs.rs/undoredo/latest/undoredo/derive.Delta.html)
to automatically generate these types on arbitrary custom `struct`s and `enum`s.
There are also convenience implementations for standard library collections:
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
[`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html),
[`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html),
[`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html),
[`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html),
and for some third-party feature-gated types:
[`bidimap::BiBTreeMap` and `bidimap::BiHashMap`](https://docs.rs/bidimap/latest/bidimap/),
[`indexmap::IndexMap` and `indexmap::IndexSet`](https://docs.rs/indexmap/latest/indexmap/),
[`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/struct.RTree.html),
[`rstared::RTreed`](https://docs.rs/rstared/latest/rstared/),
[`StableVec`](https://docs.rs/stable-vec/latest/stable_vec/type.StableVec.html),
[`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
(read more in the [Supported containers](#supported-containers) section).

This library is compatible with `no_std` and `serde` and has no mandatory
third-party dependencies except for [`alloc`](https://doc.rust-lang.org/alloc/).

## Demo

![Animation showing polygons being added and subtracted in the demo stored in
`demos/polygon_set/` directory of the repository of the polygon_unionfind crate.
](https://raw.githubusercontent.com/mikwielgus/undoredo/refs/heads/develop/polygon_set_demo.gif)

The above demo animation shows Undo/Redo action over dynamically added and
subtracted polygons with R-tree spatial indexing. Neither commands nor snapshots
are stored, but all edits in history are made of sparse deltas of the polygon
container and the associated R-tree.

Source code with instructions for running the above interactive visualization
can be found in the repository of the
[polygon_unionfind](https://github.com/mikwielgus/polygon_unionfind) crate (see
its README for details).

## Usage

### Adding dependency

First, add `undoredo` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
undoredo = { version = "0.12.1", features = ["derive"] }
```

The `derive` feature flag is only required when using deltas on custom `struct`
or `enum` types to derive delta edit types. Snapshots and commands work without
any derives.

### Usage examples

#### Delta recorder over `BTreeMap`

Following is a basic usage example of delta-recording undo-redo over `BTreeMap`.
You can find more examples in the
[examples/](https://github.com/mikwielgus/undoredo/tree/develop/examples)
directory.

```rust
use std::collections::BTreeMap;
use undoredo::aliases::BTreeMapDelta;
use undoredo::{Recorder, UndoRedo};

fn main() {
    // The recorder records the ongoing changes to the recorded container.
    let mut recorder: Recorder<BTreeMap<usize, char>> =
        Recorder::new(BTreeMap::new());

    // The undo-redo struct that holds and maintains the undo-redo bistack.
    let mut undoredo: UndoRedo<BTreeMapDelta<usize, char>> = UndoRedo::new();

    // Push elements while recording the changes in a delta.
    recorder.insert(1, 'A');
    recorder.insert(2, 'B');
    recorder.insert(3, 'C');

    // The pushed elements are now present in the container.
    assert!(*recorder.container() == BTreeMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Flush the recorder and commit the recorded delta of pushing 'A', 'B', 'C'
    // into the undo-redo bistack.
    undoredo.commit(&mut recorder);

    // Now undo the action.
    undoredo.undo(&mut recorder);

    // The container is now empty; the action of pushing elements has been undone.
    assert!(*recorder.container() == BTreeMap::from([]));

    // Now redo the action.
    undoredo.redo(&mut recorder);

    // The elements are back in the container; the action has been redone.
    assert!(*recorder.container() == BTreeMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Once you are done recording, you can dissolve the recorder to regain
    // ownership and mutability over the recorded container.
    let (btreemap, ..) = recorder.dissolve();
    assert!(btreemap == BTreeMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));
}
```

#### Delta recorder over custom `struct` or `enum`

A data structure may not be one of the supported containers, but it may be
a custom `struct` or `enum`, made of multiple containers or other members,
possibly with additional generics and fields that should not be subject to
undo-redo. To be able to perform delta recording in such case, you need to
implement a delta structure for it together with two operations,
[`.apply_delta()`](https://docs.rs/undoredo/latest/undoredo/delta/trait.ApplyDelta.html#tymethod.apply_delta)
and
[`.flush_delta()`](https://docs.rs/undoredo/latest/undoredo/trait.FlushDelta.html#tymethod.flush_delta).
This can usually be easily done automatically by just applying a provided derive
macro,
[`#[derive(Delta)]`](https://docs.rs/undoredo/latest/undoredo/derive.Delta.html)
.

Below is an example of an undoredoable
[struct-of-arrays](https://en.wikipedia.org/wiki/AoS_and_SoA)
storage with generic coordinate type `T` that could be used for a simple
[entity-component-system](https://en.wikipedia.org/wiki/Entity_component_system):

```rust,ignore
// No need for `#[derive(Delta)]` for types stored in containers, only the
// containers themselves need this.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

// `#[derive(Delta)]` generates the `EntitiesDelta` type, which is needed to
// store deltas as edits in an `UndoRedo` bistack.
#[derive(Delta)]
// You can choose the name for the generated delta edit type through the
// `undoredo` attribute with key `delta`. If you omit this, the default is
// the name of the input type followed by the word `Delta`. The name chosen in
// this example, `EntitiesDelta`, happens to be the same as the default.
#[undoredo(delta = EntitiesDelta)]
// Each delta is made of two collections of elements called "half-deltas".
// `#[derive(Delta)]` generates a type for them as well, named similarly to full
// deltas. If that name does not suit you, you can analogously use the
// `undoredo` attribute with key `half_delta` to rename them.
#[undoredo(half_delta = EntitiesHalfDelta)]
pub struct Entities<T> {
    positions: Recorder<Vec<Vector2<T>>>,
    velocities: Recorder<Vec<Vector2<T>>>,
    healths: Recorder<Vec<i64>>,
    // You can record changes to primitive types too, not just collections.
    turn_counter: Recorder<u64>,
    // You can make fields not be subject to undo-redo by marking them with
    // `#[undoredo(skip)]`.
    // Note that this skipping only works for delta-based undo-redo because
    // snapshots and commands rely on different mechanisms.
    #[undoredo(skip)]
    not_in_delta: String,
}
```

See
[examples/entities.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/entities.rs)
for the full example.

#### Storing command metadata along with edits

It is often desirable to store some metadata along with every recorded edit,
usually some representation of the command that originated it. This can be done
by instead committing the edit using the
[`.cmd_commit()`](https://docs.rs/undoredo/latest/undoredo/struct.UndoRedo.html#method.cmd_commit)
method.

The bistack of done and undone committed edits, together with their command
metadatas ("cmd") if present, can be accessed as slices from the
[`.done()`](https://docs.rs/undoredo/latest/undoredo/struct.UndoRedo.html#method.done)
and
[`.undone()`](https://docs.rs/undoredo/latest/undoredo/struct.UndoRedo.html#method.undone)
accessor methods.

```rust
use std::collections::BTreeMap;
use undoredo::aliases::BTreeMapDelta;
use undoredo::{Recorder, UndoRedo};

// Representation of the command that originated the recorded delta.
#[derive(Debug, Clone, PartialEq)]
enum Command {
    PushChar,
}

fn main() {
    let mut recorder: Recorder<BTreeMap<usize, char>> =
        Recorder::new(BTreeMap::new());
    let mut undoredo: UndoRedo<BTreeMapDelta<usize, char>, Command> = UndoRedo::new();

    recorder.insert(1, 'A');
    recorder.insert(2, 'B');

    // Commit `Command::PushChar` enum variant as command metadata ("cmd") along
    // with the recorded delta.
    undoredo.cmd_commit(Command::PushChar, &mut recorder);

    // `Command::PushChar` is now the top element of the stack of done cmd-deltas.
    assert_eq!(undoredo.done().last().unwrap().cmd, Command::PushChar);

    undoredo.undo(&mut recorder);

    // After undo, `Command::PushChar` is now the top element of the stack of
    // undone cmd-deltas.
    assert_eq!(undoredo.undone().last().unwrap().cmd, Command::PushChar);
}
```

#### Command pattern

You can also give up edits altogether and only store commands in the metadata
field, thereby implementing the Command pattern. See
[examples/commands.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/commands.rs)
for an example.

#### Snapshot-based undo-redo

You can easily perform undo-redo using snapshots (that is, by storing whole past
states instead of only diffs between them) instead of deltas or commands. If you
were using deltas, you need to remove the
[`Recorder<...>`](https://docs.rs/undoredo/latest/undoredo/struct.Recorder.html)
wrapper over your container and instead use the
[`Snapshot<...>`](https://docs.rs/undoredo/latest/undoredo/struct.Snapshot.html)
wrapper in place of the `Delta<...>` type.

For instance, if your container is `HashMap<usize, char>`, it will look like
this:

```rust,ignore
let mut container: HashMap<usize, char> = HashMap::new();
let mut undoredo: UndoRedo<Snapshot<HashMap<usize, char>>> = UndoRedo::new();
```

This will work for all container types that implement `Clone`, without need
for any derive macro invocations or additional trait implementations as is with
deltas.

However, there are two caveats not present with deltas: if you want to be able
to restore the container to its initial (often empty) state, you need to commit
it first. And if you end up committing your latest changes, to revert them you
will need to invoke undo twice, since the first undo just restores the state to
the top of the *undone* stack.

For a full usage example, see
[examples/snapshots.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/snapshots.rs).

#### Delta-recording on maps with pushing

Some containers with map semantics also provide a special type of insertion
where a value is inserted without specifying a key, which the container instead
automatically generates and returns by itself. In `undoredo`, this operation is
called *pushing*.

If a supported type has such a push interface, you can record its changes just
as easily as insertions and removals by calling
[`.push()`](https://docs.rs/undoredo/latest/undoredo/struct.Recorder.html#impl-Push%3CK%3E-for-Recorder%3CK,+V,+C,+DC%3E)
on the recorder, like this:

```rust,ignore
recorder.push('A');
```

`StableVec` and `thunderdome::Arena` are instances of supported pushable maps.
See
[examples/stable_vec.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/stable_vec.rs)
and
[examples/thunderdome.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/thunderdome.rs)
for complete examples of their usage.

#### Delta-recording on sets

Some containers have set semantics: they operate only on values, without
exposing any usable notion of key or index. `undoredo` can provide delta-based
undo-redo functionality to such a set by treating it as a `()`-valued map whose
keys are the set's values. This is actually also how Rust's standard library
internally
[represents](https://docs.rs/hashbrown/latest/src/hashbrown/set.rs.html#115)
its two set types, `HashSet`
[and](https://doc.rust-lang.org/stable/src/alloc/collections/btree/set.rs.html)
`BTreeSet`.

As an example, the following code will construct a recorder and an undo-redo
bistack for a `BTreeSet`:

```rust,ignore
let mut recorder: Recorder<BTreeSet<char>> = Recorder::new(BTreeSet::new());
let mut undoredo: UndoRedo<BTreeSetDelta<char>> = UndoRedo::new();
```

Keeping in mind to pass values as keys, `recorder` and
`undoredo` can then be used the same way as with maps above. See
[examples/btreeset.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/btreeset.rs)
for a complete example.

Among the supported third-party types, `rstar::RTree` is a data structure for
which `undoredo` has a convenience implementation over set semantics. See
[examples/rstar.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/rstar.rs)
for an example of its usage.

**NOTE:** Some set-like data structures are actually multisets: they allow
to insert the same value multiple times without overriding the first one. In
fact, `rstar::RTree` is a multiset. `undoredo` will work correctly with such
data structures, seeing them as sets, but only if you never make use of their
multiset property: you must never insert a key again when it is already present
in a multiset.

#### Delta-recording on bidirectional maps

Delta-recording can be performed on bidirectional maps (bimaps). `undoredo` has
a convenience implementation for `BiBTreeMap` and `BiHashMap` from the `bidimap`
crate (`bidimap` is a maintained fork of the `bimap` crate).

See
[examples/bihashmap.rs](https://github.com/mikwielgus/undoredo/tree/develop/examples/bihashmap.rs)
for an example of undo-redo performed on a bidirectional map.

## Supported containers

### When using snapshots

All data structures that implement the
[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) trait are
supported for snapshot-based undo-redo.

### When using commands

For command-based undo-redo, obviously all data structures are supported as long
as the library user implements commands for them.

### When using deltas

To be able to use delta-based undo-redo on a data structure, it is necessary
to have a delta structure as well as two operations,
[`.apply_delta()`](https://docs.rs/undoredo/latest/undoredo/delta/trait.ApplyDelta.html#tymethod.apply_delta)
and
[`.flush_delta()`](https://docs.rs/undoredo/latest/undoredo/trait.FlushDelta.html#tymethod.flush_delta),
available as traits. For ease of use, `undoredo` supplies a number of built-in
convenience implementations of all these for various commonly-used container
types.

For custom data structures, there is a derive macro, `#[derive(Delta)]`, that
will generate all the necessary code.

Though we try our best, this derive macro may not work on some highly-generic
data types. And in some cases, it may be desirable to add some additional logic
to the operations. In these situations, it is necessary to implement the delta
structure and operations on it manually.

For an example of a manual implementation, take a look at
[mikwielgus/polygon_unionfind/src/unionfind.rs](https://github.com/mikwielgus/polygon_unionfind/blob/develop/src/unionfind.rs#L141).
This is a delta-undoredoable union-find where every field's type is a generic
parameter.

#### Standard library

Rust's standard library maps and sets have built-in convenience implementations
of delta-editing:

- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), gated by the `std` feature flag (enabled by default);
- [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html), gated by the `std` feature flag (enabled by default);
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), not feature-gated;
- [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html), not feature-gated;
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), not feature-gated.

#### Third-party types

In addition to the standard library, `undoredo` also has built-in feature-gated
convenience implementations of delta-editing for data structures from certain
external crates:

- [`bidimap::BiBTreeMap` and `bidimap::BiHashMap`](https://docs.rs/bidimap/latest/bidimap/),
  gated by the `bidimap` feature flag (usage example:
  [examples/bihashmap.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/bihashmap.rs));
- [`indexmap::IndexMap` and `indexmap::IndexSet`](https://docs.rs/indexmap/latest/indexmap/),
  gated by the `indexmap` feature flag;
- [`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/index.html), gated by the
  `rstar` feature flag (usage example: [examples/rstar.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/rstar.rs));
- [`rstared::RTreed`](https://docs.rs/rstared/latest/rstared/), gated by the
  `rstared` feature flag (usage example: [examples/rstared.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/rstared.rs));
- [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/),
  gated by the `stable-vec` feature flag (usage example:
  [examples/stable_vec.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/stable_vec.rs));
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
  gated by the `thunderdome` feature flag (usage example:
  [examples/thunderdome.rs](https://github.com/mikwielgus/undoredo/blob/develop/examples/thunderdome.rs)).

To use these, enable their corresponding feature flags next to your `undoredo`
dependency in your `Cargo.toml`. For example, to enable all third-party type
implementations, write

```toml
[dependencies]
undoredo = { version = "0.12.1", features = ["bidimap", "indexmap", "rstar", "rstared", "stable-vec", "thunderdome"] }
```

#### Custom types

To make delta-based undo-redo work with a map-like non-composite
container data structure for which there is no convenience implementation,
you can create one on your own by implementing the traits from the
[maplike](https://crates.io/crates/maplike) crate. Refer to that crate's
documentation for details. These traits are also re-exported by `undoredo`, so
it is not necessary to add another dependency.

If you believe that other people could benefit from your implementation,
consider contributing it to `maplike`. We will integrate it in `undoredo` on our
own afterwards (no need to open more than one pull request).

#### Unsupported containers

Some containers cannot be supported for delta-based
undo-redo because they lack an interface on which
`maplike`'s traits could be implemented. See the [Unsupported
containers](https://docs.rs/maplike/latest/maplike/#unsupported-containers)
section in `maplike`'s documentation for details.

## Documentation

See the [documentation](https://docs.rs/undoredo/latest/undoredo) for more information
on `undoredo`'s usage.

## Packaging

`undoredo` is published as a [crate](https://crates.io/crates/undoredo) on the
[Crates.io](https://crates.io/) registry.

## Contributing

We welcome issues, pull requests and any other contributions from anyone to our
[repository](https://github.com/mikwielgus/undoredo) on GitHub.

## Licence

### Outbound licence

`undoredo` is dual-licensed as under either of

- [MIT license](./LICENSES/MIT.txt),
- [Apache License, Version 2.0](./LICENSES/Apache-2.0.txt),

at your option.

### Inbound licence

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this work by you will be dual-licensed as described above,
without any additional terms or conditions.
