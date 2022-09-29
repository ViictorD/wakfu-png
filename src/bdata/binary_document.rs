use std::{io::{Read, Seek}, collections::HashMap};
use anyhow::{Result};
use bytebuffer::{ByteBuffer, Endian};
use super::random_byte_buffer_reader::RandomByteBufferReader;
use super::index::Index;
use std::mem::size_of;

#[derive(Debug)]
pub struct InteractiveElementModelBinaryData {
	pub view_model_id: i32,
	pub view_type_id: i16,
	pub gfx: i32,
	pub color: i32,
	pub height: i8,
	pub particle_id: i32,
	pub particle_offset_z: i32
}

#[derive(Debug)]
struct Entry {
	_id: i64,
	position: i32,
	_size: i32,
	seed: i8
}

pub struct BinaryDocument {
	entries: Vec<Entry>,
	_indexes: HashMap<String, Index>,
	buffer: RandomByteBufferReader
}

impl BinaryDocument {
	pub fn load<R: Read + Seek>(input: R) -> Result<Self> {
		let mut archive = zip::ZipArchive::new(input)?;
		let mut file = archive.by_index(0)?;

		let mut bin_data: Vec<u8> = Vec::with_capacity(file.size() as usize);
		file.read_to_end(&mut bin_data)?;

		let mut buffer = ByteBuffer::from_vec(bin_data);
		buffer.set_endian(Endian::LittleEndian);

		let version = buffer.read_i32().unwrap() + 756423;

		let slice = &buffer.to_bytes()[..];
		let mut sliced_buffer = ByteBuffer::from_bytes(slice);
		sliced_buffer.set_rpos(buffer.get_rpos());
		sliced_buffer.set_endian(Endian::LittleEndian);

		let mut rand_buffer = RandomByteBufferReader::load(
			sliced_buffer,
			34,
			version
		)?;
		
		let entry_count: i32 = rand_buffer.get_int();


		let mut entries: Vec<Entry> = Vec::with_capacity(entry_count as usize * size_of::<Entry>());
		for _ in 0..entry_count {
			let _id = rand_buffer.get_long();
			let position = rand_buffer.get_int();
			let _size = rand_buffer.get_int();
			let seed = rand_buffer.get_byte();
			let entry = Entry {
				_id,
				position,
				_size,
				seed
			};
			entries.push(entry);
		}

		let index_count: i8 = rand_buffer.get_byte();

		let mut _indexes: HashMap<String, Index> = HashMap::new();
		for _ in 0..index_count {
			let index = Index::create_index(&mut rand_buffer)?;
			_indexes.insert(index.get_name().clone(), index);
		}

		let pos = rand_buffer.get_buffer_rpos();
		let slice = &buffer.to_bytes()[pos..buffer.len()];

		let mut sliced_buffer = ByteBuffer::from_bytes(slice);
		sliced_buffer.set_endian(Endian::LittleEndian);

		let res_buffer = RandomByteBufferReader::load(
			sliced_buffer,
			34,
			version
		)?;

		let result = BinaryDocument {
			entries,
			_indexes,
			buffer: res_buffer
		};

		Ok(result)
		
	}

	pub fn read_iem(&mut self) -> Result<HashMap<i32, InteractiveElementModelBinaryData>> {
		let mut iem: HashMap<i32, InteractiveElementModelBinaryData> = HashMap::new();
		
		for i in 0..self.entries.len() {
			let entry = self.entries.get(i).unwrap();
			self.buffer.position(entry.position, entry.seed);

			let view_model_id: i32 = self.buffer.get_int();
			let view_type_id: i16 = self.buffer.get_short();
			let gfx: i32 = self.buffer.get_int();
			let color: i32 = self.buffer.get_int();
			let height: i8 = self.buffer.get_byte();
			let particle_id: i32 = self.buffer.get_int();
			let particle_offset_z: i32 = self.buffer.get_int();

			let result = InteractiveElementModelBinaryData {
				view_model_id,
				view_type_id,
				gfx,
				color,
				height,
				particle_id,
				particle_offset_z
			};
			iem.insert(view_model_id, result);
		}
		Ok(iem)
	}
}