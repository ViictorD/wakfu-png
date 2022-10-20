use anyhow::{Result};
use bytebuffer::ByteBuffer;

pub struct RandomByteBufferReader {
	buffer: ByteBuffer,
	seed: i8,
	add: i32,
	mult: i32,
}

impl RandomByteBufferReader {
	pub fn load(buffer: ByteBuffer, mult: i32, add: i32) -> Result<Self> {
		let seed = (mult ^ add) as i8;
		
		let result = RandomByteBufferReader {
			buffer,
			seed,
			add,
			mult
		};
		Ok(result)
	}

	pub fn position(&mut self, position: i32, seed: i8) {
		self.seed = seed;
		self.buffer.set_rpos(position as usize);
	}

	pub fn get_buffer_rpos(self) -> usize {
		self.buffer.get_rpos()
	}

	fn inc(&mut self) {
		let pos = self.buffer.get_rpos() as i32;
		let mul = self.mult.overflowing_mul(pos).0;
		let add = mul.overflowing_add(self.add).0 as i8;
		self.seed = self.seed.overflowing_add(add).0;
	}
	
	pub fn get_byte(&mut self) -> i8 {
		self.inc();
		self.buffer.read_i8().unwrap().overflowing_sub(self.seed).0
	}

	// pub fn read_boolean(&mut self) -> bool {
	// 	self.inc();
	// 	self.buffer.read_i8().unwrap().overflowing_sub(self.seed).0 != 0
	// }

	pub fn get_short(&mut self) -> i16 {
		self.inc();
		self.buffer.read_i16().unwrap().overflowing_sub(self.seed as i16).0
	}

	pub fn get_int(&mut self) -> i32 {
		self.inc();
		self.buffer.read_i32().unwrap().overflowing_sub(self.seed as i32).0
	}
	
	pub fn get_long(&mut self) -> i64 {
		self.inc();
		self.buffer.read_i64().unwrap().overflowing_sub(self.seed as i64).0
	}

	pub fn read_utf8(&mut self) -> String {
		let size = self.get_int();
		let str_bytes = self.buffer.read_bytes(size as usize).unwrap();
		String::from_utf8(str_bytes).unwrap()
	}

	pub fn read_int_array(&mut self) -> Vec<i32>{
		let size: i32 = self.get_int();
		let mut data: Vec<i32> = Vec::with_capacity(size as usize);

		for _ in 0..size {
			data.push(self.get_int())
		}
		return data;
	}

}
