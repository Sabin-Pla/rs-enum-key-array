This crate provides a datastructure for an array-backed O(1) mapping between an enum and data.

Currently relies on incomplete features:
```
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![allow(incomplete_features)]
```

Must be ran with `cargo +nightly run`

How to use:

``` main.rs
#[derive(Debug, Idable)]
enum ExampleKey {
    A,
    B,
    C,
    D,
    E
}
``` 

Next defined an id map, which will allow us to associate each of the 5 variants of ExampleKey with an index in the structure below. See idable.rs if each variant does not necessarily map to each element, and a different implementation is required.
```
// Create an IdMap with randomly assigned data (unsafe)
let mut id_map  = IdMap::<ExampleKey, S>::new();
```

Id map can be indexed by keys.
```
id_map[ExampleKey::A] = S { a: 4 };
let val = id_map[ExampleKey::B];
```