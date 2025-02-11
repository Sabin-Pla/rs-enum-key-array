#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![allow(incomplete_features)]

mod idable;
mod eka;

pub use idable::*;
pub use eka::*;
pub use eka_derive::Idable;

pub use eka_derive::RangeEnum;

#[derive(RangeEnum)]
#[variant_range(1, 23)]
enum TestRange {}

#[test]
fn main() {
	//let t = TestRange::One;
	println!("dddd");
	assert!(false);
}