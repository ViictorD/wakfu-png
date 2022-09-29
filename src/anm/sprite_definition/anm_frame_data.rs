use anyhow::{Result, anyhow};
use bytebuffer::ByteBuffer;

#[derive(Clone)]
pub enum DataContainer {
	None,
	I8(I8Container),
	I16(I16Container),
	I32(I32Container)
}

impl DataContainer {
	pub fn read(&mut self) -> i32 {
		match self {
			DataContainer::I8(e) => e.read(),
			DataContainer::I16(e) => e.read(),
			DataContainer::I32(e) => e.read(),
			_ => 0
		}
	}
}

#[derive(Clone)]
pub struct I8Container {
	cur: i32,
	data: Vec<i8>
}

impl I8Container {
	fn new(data: Vec<i8>) -> Self {
		I8Container {
			cur: 0,
			data
		}
	}

	pub fn begin(&mut self, offset: i32) {
		self.cur = offset
	}

	pub fn read(&mut self) -> i32 {
		let result = self.data.get(self.cur as usize).unwrap();
		self.cur += 1;
		*result as i32
	}
}

#[derive(Clone)]
pub struct I16Container {
	cur: i32,
	data: Vec<i16>
}

impl I16Container {
	fn new(data: Vec<i16>) -> Self {
		I16Container {
			cur: 0,
			data
		}
	}

	pub fn begin(&mut self, offset: i32) {
		self.cur = offset
	}

	pub fn read(&mut self) -> i32 {
		let result = self.data.get(self.cur as usize).unwrap();
		self.cur += 1;
		*result as i32
	}
}

#[derive(Clone)]
pub struct I32Container {
	cur: i32,
	data: Vec<i32>
}

impl I32Container {
	fn new(data: Vec<i32>) -> Self {
		I32Container {
			cur: 0,
			data
		}
	}

	pub fn begin(&mut self, offset: i32) {
		self.cur = offset
	}

	pub fn read(&mut self) -> i32 {
		let result = self.data.get(self.cur as usize).unwrap();
		self.cur += 1;
		*result as i32
	}
}

pub struct AnmFrameData;

impl AnmFrameData {
	pub fn create(buffer: &mut ByteBuffer) -> Result<DataContainer> {
		let data_type = buffer.read_i8()?;
		let size = buffer.read_i32()?;
		match data_type {
			1 => {
				let mut data = Vec::with_capacity(size as usize);
				for _ in 0..size {
					data.push(buffer.read_i8()?);
				}
				Ok(DataContainer::I8(I8Container::new(data)))
			},
			2 => {
				let mut data = Vec::with_capacity(size as usize);
				for _ in 0..size {
					data.push(buffer.read_i16()?);
				}
				Ok(DataContainer::I16(I16Container::new(data)))
			}
			4 => {
				let mut data = Vec::with_capacity(size as usize);
				for _ in 0..size {
					data.push(buffer.read_i32()?);
				}
				Ok(DataContainer::I32(I32Container::new(data)))
			}
			_ => Err(anyhow!("Type not found"))
		}
	}
}