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

    // Create an EKA with randomly assigned data (unsafe)
    let mut eka_random_init  = unsafe { EKA::<ExampleKey, S>::uninitialized() };
    
    // Create an EKA with all zero-byte data 
    // unsafe because S not being zeroable might induce UB
    let mut eka  = unsafe { EKA::<ExampleKey, S>::zeroed() };
    for i in 0..5 {
        assert!(eka[i].a == 0)
    }

    eka_random_init[ExampleKey::A] = S { a: 4 };
    eka_random_init[ExampleKey::B] = S { a: 6 };

    // provides IndexMut[ExampleKey]
    // EKA { buf: [S { a: 4 }, S { a: 6 }, S { a: <undefined> }, S { a: <undefined> }, S { a: <undefined> }] }
    println!("{:?}", eka_random_init);

    // provides Index[ExampleKey]
    // EKA S { a: 6 }
    println!("{:?}", eka_random_init[ExampleKey::B]);
}
