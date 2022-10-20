use std::{io::{Read, Seek}, collections::HashMap};
use anyhow::{Result};
use bytebuffer::{ByteBuffer, Endian};
use super::random_byte_buffer_reader::RandomByteBufferReader;
use super::index::Index;
use std::mem::size_of;

#[derive(Debug)]
pub struct Entry {
	_id: i64,
	pub position: i32,
	_size: i32,
	pub seed: i8
}

pub struct BinaryDocument {
	pub entries: Vec<Entry>,
	_indexes: HashMap<String, Index>,
	pub buffer: RandomByteBufferReader
}

impl BinaryDocument {
	pub fn load<R: Read + Seek>(input: R, data_type: u32) -> Result<Self> {
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
			data_type as i32,
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
			data_type as i32,
			version
		)?;

		let result = BinaryDocument {
			entries,
			_indexes,
			buffer: res_buffer
		};

		Ok(result)
		
	}
}