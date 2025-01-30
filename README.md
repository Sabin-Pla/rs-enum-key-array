This crate provides a datastructure for an array-backed O(1) mapping between enum variants and array indexes.

```
[dependencies]
eka = "0.1.0"
```

Currently relies on incomplete features:
```
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![allow(incomplete_features)]
```

This demo, as well as any project using this crate must built with `cargo +nightly`. 

Projects using this crate also need to enable
`#![feature(generic_const_exprs)]`

## How to use:

First, define the enum you want to use as your array key.

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

Next, define the enum key array, which will allow us to associate each of the 5 variants of ExampleKey with an index in the structure below. See idable.rs if each variant does not necessarily map to each element, and a different implementation is required.
```
// Create an EKA with randomly assigned data (unsafe)
let mut eka = EKA::<ExampleKey, S>::new();
```

EKA can be indexed by keys.
```
eka[ExampleKey::A] = S { a: 4 };
let val = eka[ExampleKey::B];
```