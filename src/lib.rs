#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![allow(incomplete_features)]

mod idable;
mod eka;

pub use idable::*;
pub use eka::*;
pub use eka_derive::{Idable, generate_enum};

pub use eka_derive::RangeEnum;

#[test]
fn run_tests() {
	
	// demo test case 1 
	generate_enum!{ 
		#[derive(Debug)]
		enum FunctionKey {
			F1: F _,
			...
			F24
		}
	}

	let e = FunctionKey::F12;
	println!("{:?}", e);

	// test case 2 - suffix
	generate_enum!{ 
		#[derive(Debug)]
		enum ExampleEnum {
			Prefix1Suffix:  Prefix _ Suffix,
			...
			Prefix5Suffix
		}
	}


	let e = ExampleEnum::Prefix3Suffix;
	let e2 = ExampleEnum::Prefix4Suffix;	
	assert!(e as u8 != e2 as u8);
}