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
    A{field1: usize, field2: bool},
    B(usize),
    D,
    E,
    C,
}

fn main() {

    // Create an EKA with randomly assigned data (unsafe)
    let mut eka_random_init  = unsafe { EKA::<ExampleKey, S>::uninitialized() };
    
    // Create an EKA with all zero-byte data 
    // unsafe because S not being zeroable might induce UB
    let mut eka_zeroed  = unsafe { EKA::<ExampleKey, S>::zeroed() };

    // if S implements Default + Copy, we can create the datastructure safely
    let mut eka_default  = EKA::<ExampleKey, S>::new();

    for i in 0..5 {
        assert!(eka_zeroed[i].a == 0);
        assert!(eka_default[i].a == 0);
    }

    eka_random_init[ExampleKey::A{field1: 0, field2: true}] = S { a: 4 };
    eka_random_init[ExampleKey::B(0)] = S { a: 6 };

    // provides IndexMut[ExampleKey]
    // EKA { buf: [S { a: 4 }, S { a: 6 }, S { a: <undefined> }, S { a: <undefined> }, S { a: <undefined> }] }
    println!("{:?}", eka_random_init);

    // provides Index[ExampleKey]
    // EKA S { a: 6 }
    println!("{:?}", eka_random_init[ExampleKey::C]);
}
