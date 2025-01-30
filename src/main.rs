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
        
    // Create an enum key array with randomly assigned data (unsafe)
    let mut eka = EKA::<ExampleKey, S>::new();

    eka[ExampleKey::A] = S { a: 4 };
    eka[ExampleKey::B] = S { a: 6 };

    // provides blanket IndexMut[ExampleKey]
    // EKA { buf: [S { a: 4 }, S { a: 6 }, S { a: <undefined> }, S { a: <undefined> }, S { a: <undefined> }] }
    println!("{:?}", eka);

    // provides blanket Index[ExampleKey]
    // EKA S { a: 6 }
    println!("{:?}", eka[ExampleKey::B]);
}
