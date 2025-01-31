This crate provides a datastructure for an array-backed O(1) mapping between enum variants and array indexes.

```
[dependencies]
eka = "0.1.1"
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
```main.rs
// Create an EKA with randomly assigned data (unsafe)
let mut eka_random_init  = unsafe { EKA::<ExampleKey, S>::uninitialized() };

// Create an EKA with all zero-byte data 
// unsafe because S not being zeroable might induce UB
let mut eka  = unsafe { EKA::<ExampleKey, S>::zeroed() };
```

EKA can be indexed by keys.
```main.rs
eka[ExampleKey::A] = <expression>;
let val = eka[ExampleKey::B];
```