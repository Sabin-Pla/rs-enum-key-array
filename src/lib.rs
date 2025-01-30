#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![allow(incomplete_features)]

mod idable;
mod id_map;

pub use idable::*;
pub use id_map::*;
pub use id_map_derive::Idable;

