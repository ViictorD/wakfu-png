use std::{collections::HashMap};
use anyhow::{Result};
use super::random_byte_buffer_reader::RandomByteBufferReader;

#[derive(Debug)]
struct Unique {
	indexes: HashMap<i64, i32>
}

impl Hash for Unique {
	fn load(count: i32) -> Result<Self> {
		let indexes: HashMap<i64, i32> = HashMap::with_capacity(count as usize);
		let result = Unique {
			indexes
		};
		Ok(result)
	}

	fn read_entry(&mut self, idx: i64, buffer: &mut RandomByteBufferReader) {
		self.indexes.insert(idx, buffer.get_int());
	}
}

// #[derive(Debug)]
// struct Multi {
// 	indexes: HashMap<i64, Vec<i32>>
// }
//
// impl Hash for Multi {
// 	fn load(count: i32) -> Result<Self> {
// 		let indexes: HashMap<i64, Vec<i32>> = HashMap::with_capacity(count as usize);
// 		let result = Multi {
// 			indexes
// 		};
// 		Ok(result)
// 	}
//
// 	fn read_entry(&mut self, idx: i64, buffer: &mut RandomByteBufferReader) {
// 		self.indexes.insert(idx, buffer.read_int_array());
// 	}
// }

pub struct Index {
	name: String,
	_hash: Box<dyn Hash>
}

impl Index {
	pub fn create_index(buffer: &mut RandomByteBufferReader) -> Result<Index> {
		let is_unique = buffer.get_byte() != 0;
		let name: String = buffer.read_utf8();
		let count = buffer.get_int();

		if is_unique {
			let mut unique = Unique::load(count)?;
			for _ in 0..count {
				let idx: i64 = buffer.get_long();
				unique.read_entry(idx, buffer);
			}

			let result = Index {
				name,
				_hash: Box::new(unique)
			};

			Ok(result)
		}
		else {
			let mut multi = Unique::load(count)?;
			for _ in 0..count {
				let idx: i64 = buffer.get_long();
				multi.read_entry(idx, buffer);
			}

			let result = Index {
				name,
				_hash: Box::new(multi)
			};
			Ok(result)
		}
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}

}

pub trait Hash {
	fn load(count: i32) -> Result<Self> where Self: Sized;
	fn read_entry(&mut self, idx: i64, buffer: &mut RandomByteBufferReader);
}
