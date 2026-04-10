<!--
SPDX-FileCopyrightText: 2025 undoredo contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Docs](https://docs.rs/undoredo/badge.svg)](https://docs.rs/undoredo/)
[![Crates.io](https://img.shields.io/crates/v/undoredo.svg)](https://crates.io/crates/undoredo)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/undoredo.svg)](#licence)

# undoredo

`undoredo` is an undo-redo Rust library that wraps a data structure (*container*)
inside a recorder [decorator](https://en.wikipedia.org/wiki/Decorator_pattern)
that observes the incoming insertions, removals and pushes while passively and
incrementally recording the changes in reversible *deltas*.

This approach makes `undoredo` easier to use than other
undo-redo libraries. Storing deltas typically results in much
more succint and reliable code than the commonly used [Command
pattern](https://en.wikipedia.org/wiki/Command_pattern), which is what the
popular and venerable [`undo`](https://docs.rs/undo/latest/undo/) crate relies
on. The programmer is relieved from having to maintain application-specific
implementations of commands, often complicated and prone to elusive runtime
bugs, on which the Command pattern has to operate.

Every delta is just a diff (alternative terms: *patch*, *edit*) between
subsequent states, so using them to store the changes requires substantially
less memory than storing whole *snapshots* of past states, another common method
for implementing the undo-redo pattern.

This library is `no_std`-compatible and has no mandatory third-party dependencies except
for [`alloc`](https://doc.rust-lang.org/alloc/). For ease of use, `undoredo` has
convenience implementations for standard library collections:
[`HashMap`](https://doc.rust-lang.org/std/containers/struct.HashMap.html),
[`HashSet`](https://doc.rust-lang.org/stable/std/containers/struct.HashSet.html),
[`BTreeMap`](https://doc.rust-lang.org/std/containers/struct.BTreeMap.html),
[`BTreeSet`](https://doc.rust-lang.org/stable/std/containers/struct.BTreeSet.html),
and for some third-party feature-gated types:
[`StableVec`](https://docs.rs/stable-vec/latest/stable_vec/type.StableVec.html),
[`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
[`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/struct.RTree.html),
[`rstared::RTreed`](https://docs.rs/rstared/latest/rstared/) (read more in the
[Supported containers](#supported-containers) section).

## Usage

First, add `undoredo` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
undoredo = "0.9"
```

### Basic usage

Following is a basic usage example of `undoredo`
over `HashMap`. You can find more examples in the
[examples/](https://codeberg.org/topola/undoredo/src/branch/develop/examples)
directory.

```rust,ignore
use std::collections::{BTreeMap, HashMap};
use undoredo::{Delta, Recorder, UndoRedo};

fn main() {
    // The recorder records the ongoing changes to the recorded container.
    let mut recorder: Recorder<HashMap<usize, char>> =
        Recorder::new(HashMap::new());

    // The undo-redo struct maintains the undo-redo bistack.
    let mut undoredo: UndoRedo<Delta<BTreeMap<usize, char>>> = UndoRedo::new();

    // Push elements while recording the changes in an delta.
    recorder.insert(1, 'A');
    recorder.insert(2, 'B');
    recorder.insert(3, 'C');

    // Flush the recorder and commit the recorded delta of pushing 'A', 'B', 'C'
    // into the undo-redo bistack.
    undoredo.commit(&mut recorder);

    // The pushed elements are now present in the container.
    assert!(*recorder.container() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Now undo the action.
    undoredo.undo(&mut recorder);

    // The container is now empty; the action of pushing elements has been undone.
    assert!(*recorder.container() == HashMap::from([]));

    // Now redo the action.
    undoredo.redo(&mut recorder);

    // The elements are back in the container; the action has been redone.
    assert!(*recorder.container() == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));

    // Once you are done recording, you can dissolve the recorder to regain
    // ownership and mutability over the recorded container.
    let (mut hashmap, ..) = recorder.dissolve();
    assert!(hashmap == HashMap::from([(1, 'A'), (2, 'B'), (3, 'C')]));
}
```

### Storing and accessing command metadata along with deltas

It is often desirable to store some metadata along with every recorded delta,
usually a representation of the command that originated it. This can be done by
instead committing the delta using the
[`.cmd_commit()`](https://docs.rs/undoredo/latest/undoredo/struct.UndoRedo.html#method.cmd_commit)
method.

The bistack of done and undone committed deltas, together with their
command metadatas ("cmd") if present, can be accessed as slices from the
[`.done()`](https://docs.rs/undoredo/latest/undoredo/struct.UndoRedo.html#method.undone)
and
[`.undone()`](https://docs.rs/undoredo/latest/undoredo/struct.UndoRedo.html#method.undone)
accessor methods.

```rust,ignore
use std::collections::{BTreeMap, HashMap};
use undoredo::{Delta, Insert, Recorder, UndoRedo};

// Representation of the command that originated the recorded delta.
#[derive(Debug, Clone, PartialEq)]
enum Command {
    PushChar,
}

fn main() {
    let mut recorder: Recorder<HashMap<usize, char>> =
        Recorder::new(HashMap::new());
    let mut undoredo: UndoRedo<Delta<BTreeMap<usize, char>>, Command> = UndoRedo::new();

    recorder.insert(1, 'A');
    recorder.insert(2, 'B');

    // Commit `Command::PushChar` enum variant as command metadata ("cmd") along
    // with the recorded delta.
    undoredo.cmd_commit(Command::PushChar, recorder.flush_delta());

    // `Command::PushChar` is now the top element of the stack of done cmd-deltas.
    assert_eq!(undoredo.done().last().unwrap().cmd, Command::PushChar);

    undoredo.undo(&mut recorder);

    // After undo, `Command::PushChar` is now the top element of the stack of
    // undone cmd-deltas.
    assert_eq!(undoredo.undone().last().unwrap().cmd, Command::PushChar);
}
```

### Undo-redo on maps with pushing

Some data structures with map semantics also provide a special type of insertion
where a value is inserted without specifying a key, which the structure
instead automatically generates and returns by itself. This operation is called
"pushing".

If a supported type has a push interface, you can record its changes just as
easily as insertions and removals by calling
[`.push()`](https://docs.rs/undoredo/latest/undoredo/struct.Recorder.html#impl-Push%3CK%3E-for-Recorder%3CK,+V,+C,+DC%3E)
on the recorder, like this:

```rust,ignore
recorder.push('A');
```

`StableVec` and `thunderdome::Arena` are instances of supported pushable maps.
[examples/stable_vec.rs](https://codeberg.org/topola/undoredo/src/branch/develop/examples/stable_vec.rs)
and
[examples/thunderdome.rs](https://codeberg.org/topola/undoredo/src/branch/develop/examples/thunderdome.rs)
for complete examples of their usage.

### Undo-redo on sets

Some data structures have set semantics: they operate only on values, without
exposing any usable notion of key or index. `undoredo` can provide its
functionality to a set by treating it as a `()`-valued map whose keys are the
set's values. This is actually also how Rust's standard library
internally
[represents](https://docs.rs/hashbrown/latest/src/hashbrown/set.rs.html#115) its
two set types, `HashSet`
[and](https://doc.rust-lang.org/stable/src/alloc/containers/btree/set.rs.html#82)
`BTreeSet`.

As an example, the following code will construct a recorder and an undo-redo
bistack for a `BTreeSet`:

```rust,ignore
let mut recorder: Recorder<BTreeSet<char>> = Recorder::new(BTreeSet::new());
let mut undoredo: UndoRedo<Delta<BTreeSet<char>>> = UndoRedo::new();
```

Keeping in mind to pass values as keys, `recorder` and
`undoredo` can then be used the same way as with maps above. See
[examples/btreeset.rs](https://codeberg.org/topola/undoredo/src/branch/develop/examples/btreeset.rs)
for a complete example.

Among the supported third-party types, `rstar::RTree` is an instance of a data
structure with a convenience implementation over set semantics. See
[examples/rstar.rs](https://codeberg.org/topola/undoredo/src/branch/develop/examples/rstar.rs)
for an example of its usage.

**NOTE:** Some set-like data structures are actually multisets: they allow
to insert the same value multiple times without overriding the first one. In
fact, `rstar::RTree` is a multiset. `undoredo` will work correctly with such
data structures, seeing them as sets, but only if you never make use of their
multiset property: you must never insert a key that is already present in
a multiset.

### Undo-redo on custom types

To make `undoredo` work with a map-like data structure for which there is no
convenience implementation, you can create one on your own by implementing
the traits from the [maplike](https://docs.rs/maplike/latest/maplike/) crate.
Refer to that crate's documentation for details. These traits are also
re-exported by `undoredo`, so it is not necessary to add another dependency.

If you believe that other people could benefit from your implementation,
consider contributing it to `maplike`. We will integrate it in `undoredo` on our
own afterwards (no need to open more than one pull request).

## Supported containers

### Standard library

Rust's standard library maps and sets are supported via built-in convenience
implementations:

- [`HashMap`](https://doc.rust-lang.org/std/containers/struct.HashMap.html), gated by the `std` feature (enabled by default);
- [`HashSet`](https://doc.rust-lang.org/stable/std/containers/struct.HashSet.html), gated by the `std` feature (enabled by default);
- [`BTreeMap`](https://doc.rust-lang.org/std/containers/struct.BTreeMap.html), not feature-gated;
- [`BTreeSet`](https://doc.rust-lang.org/stable/std/containers/struct.BTreeSet.html), not feature-gated.

### Third-party types

In addition to the standard library, `undoredo` has built-in feature-gated
convenience implementations for data structures from certain external crates:

- [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/),
  gated by the `stable-vec` feature (example usage:
  [examples/stable_vec.rs](https://codeberg.org/topola/undoredo/src/branch/develop/examples/stable_vec.rs)),
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
  gated by the `thunderdome` feature (example usage:
  [examples/thunderdome.rs](https://codeberg.org/topola/undoredo/src/branch/develop/examples/thunderdome.rs));
- [`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/index.html), gated by the
  `rstar` feature (example usage: [examples/rstar.rs](https://codeberg.org/topola/undoredo/src/branch/develop/examples/rstar.rs));
- [`rstared::RTreed`](https://docs.rs/rstared/latest/rstared/), gated by the
  `rstared` feature (example usage: [examples/rstared.rs](https://codeberg.org/topola/undoredo/src/branch/develop/examples/rstared.rs)).

To use these, enable their corresponding features next to your `undoredo`
dependency in your `Cargo.toml`. For example, to enable all third-party type
implementations, write

```toml
[dependencies]
undoredo = { version = "0.6", features = ["stable-vec", "thunderdome", "rstar", "rstared"] }
```

## Unsupported containers

Some containers cannot be supported because they lack an interface
on which `maplike`'s traits could be implemented. See the [Unsupported
containers](https://docs.rs/maplike/latest/maplike/#unsupported-containers)
section in `maplike`'s documentation for details.

## Documentation

See the [documentation](https://docs.rs/undoredo/latest/undoredo) for more information
on `undoredo`'s usage.

## Packaging

`undoredo` is published as a [crate](https://crates.io/crates/undoredo) on the
[Crates.io](https://crates.io/) registry.

## Contributing

We welcome issues and pull requests from anyone both to our canonical
[repository](https://codeberg.org/topola/undoredo) on Codeberg and to our GitHub
[mirror](https://github.com/mikwielgus/undoredo).

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
