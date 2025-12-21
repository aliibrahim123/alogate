// generated from file: /home/ali/dev/rust/alogate/back/src/loader/decl-src/package.stomd

use std::collections::HashMap;
use structom::{Value, Key, encoding::*};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Package {
	pub name: String,
	pub disc: String,
}
impl Serialized for Package {
	fn encode(&self) -> Vec<u8> {
		let mut data = Vec::new();
		data.extend_from_slice(&[
			0x0d, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x2e, 0x73, 0x74, 0x6f, 0x6d, 0x64, 0x00, 
		]);
		encode_Package(&mut data, self);
		data
	}
	fn encode_inline(&self, data: &mut Vec<u8>) {
		encode_Package(data, self);
	}
	fn decode(data: &[u8]) -> Option<Package> {
		let mut ind = 0;
		if decode_str(data, &mut ind)? != "package.stomd" {
			return None;
		}
		if decode_vuint(data, &mut ind)? != 0 {
			return None;
		}
		let value = decode_Package(data, &mut ind)?;
		if ind != data.len() { None } else { Some(value) }
	}
	fn decode_headless(data: &[u8]) -> Option<Package> {
		let mut ind = 0;
		let value = decode_Package(data, &mut ind)?;
		if ind != data.len() { None } else { Some(value) }
	}
	fn decode_inline(data: &[u8], ind: &mut usize) -> Option<Package> {
		decode_Package(data, ind)
	}
}

pub fn encode_Package(data: &mut Vec<u8>, value: &Package) {
	let Package {
		name: f_name, disc: f_disc, 
	} = value else { unreachable!() };
	encode_vuint(data, 2);
	encode_vuint(data, 5);
	encode_str(data, &f_name);
	encode_vuint(data, 13);
	encode_str(data, &f_disc);
}
pub fn decode_Package(data: &[u8], ind: &mut usize) -> Option<Package> {
	let mut f_name = None; let mut f_disc = None; 
	for _ in 0..decode_vuint(data, ind)? {
		let header = decode_vuint(data, ind)?;
		let tag = header >> 3;
		if tag == 0 {
			f_name = Some(decode_str(data, ind)?);
		} else if tag == 1 {
			f_disc = Some(decode_str(data, ind)?);
		} else { skip_field(data, ind, header)? }
	}
	let Some(f_name) = f_name else { return None; };
	let Some(f_disc) = f_disc else { return None; };
	Some(Package {
		name: f_name, disc: f_disc, 
	})
}

impl Into<Value> for Package {
	fn into(self) -> Value {
		let mut map = HashMap::new();
		map.insert(Key::from("name"), self.name.into());
		map.insert(Key::from("disc"), self.disc.into());
		Value::Map(map)
	}
}
impl TryFrom<Value> for Package {
	type Error = ();
	fn try_from(value: Value) -> Result<Package, ()> {
		let Value::Map(mut map) = value else { return Err(()); };
		let f_name = map.remove(&"name".into()).ok_or(())?.try_into()?;
		let f_disc = map.remove(&"disc".into()).ok_or(())?.try_into()?;
		if !map.is_empty() { return Err(()); }
		Ok(Package {
			name: f_name, disc: f_disc, 
		})
	}
}
