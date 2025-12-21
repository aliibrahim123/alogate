// generated from /home/ali/dev/rust/alogate/back/src/loader/decl-src'
#![allow(warnings)]
use std::any::Any;
use structom::encoding::*;

pub mod package;

pub fn decode(data: &[u8]) -> Option<Box<dyn Any>> {
	let mut ind = 0;
	let value: Box<dyn Any> = match decode_str(data, &mut ind)?.as_str() {
		"package.stomd" => match decode_vuint(data, &mut ind)? {
			0 => Box::new(package::decode_Package(data, &mut ind)?),
			_ => return None,
		},
		_ => return None,
	};
	if ind != data.len() {
		None
	} else {
		Some(value)
	}
}
