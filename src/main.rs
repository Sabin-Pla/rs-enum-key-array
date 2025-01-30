#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![allow(incomplete_features)]

mod idable;
mod eka;

pub use idable::*;
pub use eka::*;
pub use eka_derive::Idable;


#[derive(Debug, Default, Clone, Copy)]
struct S {
    a: u8
}

#[derive(Debug, Idable)]
enum ExampleKey {
    A,
    B,
    C,
    D,
    E
}

fn main() {
        
    // Create an IdMap with randomly assigned data (unsafe)
    let mut id_map  = EKA::<ExampleKey, S>::new();

    id_map[ExampleKey::A] = S { a: 4 };
    id_map[ExampleKey::B] = S { a: 6 };

    // provides blanket IndexMut[ExampleKey]
    // IdMap { buf: [S { a: 4 }, S { a: 6 }, S { a: <undefined> }, S { a: <undefined> }, S { a: <undefined> }] }
    println!("{:?}", id_map);

    // provides blanket Index[ExampleKey]
    // IdMap S { a: 6 }
    println!("{:?}", id_map[ExampleKey::B]);
}
