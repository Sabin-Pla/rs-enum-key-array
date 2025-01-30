#![feature(generic_const_items)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![allow(incomplete_features)]

mod idable;
mod id_map;

pub use idable::*;
pub use id_map::*;


#[derive(Debug, Default, Clone, Copy)]
struct S {
    a: u8
}


#[derive(Debug)]
enum Test {
    A,
    B
}

impl Idable for Test {
    const MAX: usize = 2;
    fn idx(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1
        }
    }
}



fn main() {
    let mut id_map  = IdMap::<Test, S>::new();
    id_map[Test::A] = S { a: 4 };
    id_map[Test::B] = S { a: 6 };
    println!("{:?}", id_map);
    println!("{:?}",     id_map[Test::B]);
}
