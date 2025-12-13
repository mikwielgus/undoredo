# undoredo

`undoredo` is an undo-redo library that works by wrapping a collection inside
a decorator that observes the incoming insertions, removals, and pushes, and
records the changes in a reversible incremental diff structure.

This approach makes `undoredo` easier to use than other undo-redo libraries.
Storing incremental diffs is much more succinct and reliable than the commonly
used [Command pattern](https://en.wikipedia.org/wiki/Command_pattern). This
is because the Command pattern requires implementing custom commands and
their behavior using traits, which results in having to maintain verbose,
application-specific logic that is prone to elusive runtime bugs.

## Usage

```rust
use stable_vec::StableVec;
use undoredo::{Push, Recorder, Remove, UndoRedo};

#[test]
fn main() {
    let mut recorder: Recorder<usize, char, StableVec<char>> = Recorder::new(StableVec::new());
    let mut undoredo: UndoRedo<StableVec<char>> = UndoRedo::new();

    recorder.push('A');
    undoredo.commit(recorder.flush());

    recorder.push('B');
    recorder.push('B');
    undoredo.commit(recorder.flush());

    let key = recorder.push('X');
    recorder.remove(&key);
    recorder.push('C');
    undoredo.commit(recorder.flush());

    assert!(
        recorder
            .container()
            .values()
            .copied()
            .eq(['A', 'B', 'B', 'C'])
    );

    undoredo.undo(&mut recorder);
    assert!(recorder.container().values().copied().eq(['A', 'B', 'B']));

    undoredo.undo(&mut recorder);
    assert!(recorder.container().values().copied().eq(['A']));

    undoredo.redo(&mut recorder);
    assert!(recorder.container().values().copied().eq(['A', 'B', 'B']));

    undoredo.redo(&mut recorder);
    assert!(
        recorder
            .container()
            .values()
            .copied()
            .eq(['A', 'B', 'B', 'C'])
    );
}
```

## Supported containers

### Standard library

Standard library maps
[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and
[BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) are
supported via built-in implementations. You can disable them by turning off the
default `std` feature.

### Foreign implementations

In addition to the standard library, `undoredo` is implemented for data
structures from some external crates:

- [StableVec](https://docs.rs/stable-vec/latest/stable_vec/) behind `stable-vec` feature.
- [thunderdome::Arena](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html) behind `thunderdome` feature.

Unlike maps, which support insertion and removal under arbitrary keys, a
stable-vec-style data structure can be only supported if it has an interface
to add values at indexes that are equal or greater than the current length. In
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
