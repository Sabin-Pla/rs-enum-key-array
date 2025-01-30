#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![allow(incomplete_features)]

mod idable;
mod eka;

pub use idable::*;
pub use eka::*;
pub use eka_derive::Idable;

