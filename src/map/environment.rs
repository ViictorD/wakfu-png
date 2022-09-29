use byte::{TryRead, BytesExt};
use byte::ctx::{Endian, Str};
use anyhow::{anyhow, Result};
use std::mem::size_of;
use std::io::{Read, Seek};
use std::str::FromStr;
use super::binar_serial_part::{BinarSerialParts, BinarSerialPartsEnum};
use bytebuffer::ByteBuffer;

#[derive(Debug, Clone)]
pub struct ElementCoord {
	pub x: i8,
	pub y: i8,
	pub z: i16,
}

impl<'a> TryRead<'a> for ElementCoord {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let x: i8 = bytes.read(offset)?;
		let y: i8 = bytes.read(offset)?;
		let z: i16 = bytes.read(offset)?;

		let result = ElementCoord {
			x,
			y,
			z,
		};

		Ok((result, *offset))
	}
}

#[derive(Debug)]
pub struct Particle {
	_coord: ElementCoord,
	_system_id: i32,
	_level: u8,
	_unknown: f32,
	_offset_x: i8,
	_offset_y: i8,
	_offset_z: i8,
	_lod: i8
}

impl<'a> TryRead<'a> for Particle {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let _coord: ElementCoord = bytes.read(offset)?;
		let _system_id: i32 = bytes.read(offset)?;
		let _level: u8 = bytes.read(offset)?;
		let _unknown: f32 = bytes.read(offset)?;
		let _offset_x: i8 = bytes.read(offset)?;
		let _offset_y: i8 = bytes.read(offset)?;
		let _offset_z: i8 = bytes.read(offset)?;
		let _lod: i8 = bytes.read(offset)?;

		let result = Particle {
			_coord,
			_system_id,
			_level,
			_unknown,
			_offset_x,
			_offset_y,
			_offset_z,
			_lod
		};

		Ok((result, *offset))
	}
}

#[derive(Debug)]
pub struct Sound {
	_coord: ElementCoord,
	_sound_id: i32
}

impl<'a> TryRead<'a> for Sound {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let _coord: ElementCoord = bytes.read(offset)?;
		let _sound_id: i32 = bytes.read(offset)?;

		let result = Sound {
			_coord,
			_sound_id
		};

		Ok((result, *offset))
	}
}

#[derive(Debug)]
pub struct InteractiveElementData {
	data: Vec<BinarSerialPartsEnum>
}

impl InteractiveElementData {
	fn read(mut bytes: ByteBuffer, version: i32) -> Result<InteractiveElementData> {
		let toc_length: i8 = bytes.read_i8()?;

		let mut toc_index: Vec<i8> = Vec::with_capacity(toc_length as usize);
		let mut toc_offset: Vec<i32> = Vec::with_capacity(toc_length as usize);

		for _ in 0..toc_length {
			toc_index.push(bytes.read_i8()?);
			toc_offset.push(bytes.read_i32()?);
		}

		let mut data = Vec::new();

		for i in 0..toc_length {
			let index: &i8 = toc_index.get(i as usize).unwrap();
			let offset: &i32 = toc_offset.get(i as usize).unwrap();
			let size: i32 = 
				if i < toc_length - 1 {
					toc_offset.get((i + 1) as usize).unwrap() - offset - 1
				}
				else {
					bytes.len() as i32 - offset - 1
				};

			if *index >= 0 && *index < 6 {
				bytes.set_rpos(*offset as usize + 1);
				let part_buffer = bytes.read_bytes(size as usize)?;
				let serialized_data = ByteBuffer::from_vec(part_buffer);
				let bsp = BinarSerialParts::unserialize(*index as u8, serialized_data, version)?;
				data.push(bsp);
			}
		}

		let result = InteractiveElementData {
			data
		};

		Ok(result)
	}

	pub fn get_data(&self) -> &Vec<BinarSerialPartsEnum> {
		&self.data
	}
}

#[derive(Debug)]
pub struct InteractiveElement {
	pub id: i64,
	pub interactive_type: i16,
	pub views: Vec<i32>,
	pub data: InteractiveElementData,
	pub client_only: u8,
	pub land_mark_type: i16
}

impl<'a> TryRead<'a> for InteractiveElement {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;
		let id: i64 = bytes.read(offset)?;
		let interactive_type: i16 = bytes.read(offset)?;
		let num_view: u8 = bytes.read(offset)?;
		let views: Vec<i32> = bytes
			.read_iter(offset, Endian::default())
			.take(num_view.into())
			.collect();
		let data_size: u16 = bytes.read(offset)?;
		let data_bytes: Vec<u8> = bytes
			.read_iter(offset, Endian::default())
			.take(data_size.into())
			.collect();
		let data = InteractiveElementData::read(
			ByteBuffer::from_vec(data_bytes),
			0
		).unwrap();
		let client_only: u8 = bytes.read(offset)?;
		let land_mark_type: i16 = bytes.read(offset)?;

		let result = InteractiveElement {
			id,
			interactive_type,
			views,
			data,
			client_only,
			land_mark_type
		};

		Ok((result, *offset))
	}
}

#[derive(Debug)]
pub struct DynamicElementDef {
	pub coord: ElementCoord,
	pub id: i32,
	pub gfx_id: i32,
	pub dynamic_type: i16,
	pub direction: i8,
	pub occluder: i8,
	pub height: i8,
	pub base_animation: String,
	pub params: String
}

impl<'a> TryRead<'a> for DynamicElementDef {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;
		
		let coord: ElementCoord = bytes.read(offset)?;

		let id: i32 = bytes.read(offset)?;
		let gfx_id: i32 = bytes.read(offset)?;
		let dynamic_type: i16 = bytes.read(offset)?;
		let direction: i8 = bytes.read(offset)?;
		let occluder: i8 = bytes.read(offset)?;
		let height: i8 = bytes.read(offset)?;

		let base_animation: String = String::from_str(bytes.read_with(offset, Str::Delimiter(0))?).unwrap();
		let params: String = String::from_str(bytes.read_with(offset, Str::Delimiter(0))?).unwrap();

		let result = DynamicElementDef {
			coord,
			id,
			gfx_id,
			dynamic_type,
			direction,
			occluder,
			height,
			base_animation,
			params
		};

		Ok((result, *offset))
	}
}


#[derive(Debug)]
pub struct Environment {
	pub x: i16,
	pub y: i16,
	_particle_data: Vec<Particle>,
	_sound_data: Vec<Sound>,
	_ambiances_id: Vec<i32>,
	_ambiances: Vec<u8>,
	interactive_elements: Vec<InteractiveElement>,
	pub dynamic_elements: Vec<DynamicElementDef>
}

impl<'a> TryRead<'a> for Environment {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let _header: u8 = bytes.read(offset)?;

		let x: i16 = bytes.read(offset)?;
		let y: i16 = bytes.read(offset)?;

		// Particle data
		let num_particle_system: u8 = bytes.read(offset)?;
		let mut _particle_data: Vec<Particle> = Vec::with_capacity(num_particle_system as usize * size_of::<Particle>());
		for _ in 0..num_particle_system {
			_particle_data.push(bytes.read(offset)?);
		}
		
		// Sound
		let num_sounds: u8 = bytes.read(offset)?;
		let mut _sound_data: Vec<Sound> = Vec::with_capacity(num_sounds as usize * size_of::<Sound>());
		for _ in 0..num_sounds {
			_sound_data.push(bytes.read(offset)?);
		}
		
		// Ambiance
		let num_ambiance_id: u8 = bytes.read(offset)?;
		let mut _ambiances_id: Vec<i32> = Vec::with_capacity(num_ambiance_id as usize * size_of::<i32>());
		for _ in 0..num_ambiance_id {
			_ambiances_id.push(bytes.read(offset)?);
		}

		let size: u8 = bytes.read(offset)?;
		let mut _ambiances: Vec<u8> = Vec::with_capacity(size as usize * size_of::<u8>());
		for _ in 0..size {
			_ambiances.push(bytes.read(offset)?);
		}

		// Interactive elements
		let num_interactive_elt: u8 = bytes.read(offset)?;
		let mut interactive_elements: Vec<InteractiveElement> = Vec::with_capacity(num_interactive_elt as usize * size_of::<InteractiveElement>());
		for _ in 0..num_interactive_elt {
			interactive_elements.push(bytes.read(offset)?);
		}

		// Dynamic elements
		let num_dynamic_elt: u8 = bytes.read(offset)?;
		let mut dynamic_elements: Vec<DynamicElementDef> = Vec::with_capacity(num_interactive_elt as usize * size_of::<DynamicElementDef>());
		for _ in 0..num_dynamic_elt {
			dynamic_elements.push(bytes.read(offset)?);
		}

		let result = Environment {
			x,
			y,
			_particle_data,
			_sound_data,
			_ambiances_id,
			_ambiances,
			interactive_elements,
			dynamic_elements
		};

		Ok((result, *offset))
	}
}

impl Environment {
	pub fn get_interactive_elements(&self) -> &Vec<InteractiveElement> {
		&self.interactive_elements
	}

	pub fn get_dynamic_elements(&self) -> &Vec<DynamicElementDef> {
		&self.dynamic_elements
	}
}

pub struct EnvironmentChunk {
	chunks: Vec<Environment>
}

impl EnvironmentChunk {
	pub fn load<R: Read + Seek>(input: R) -> Result<Self> {
		let mut archive = zip::ZipArchive::new(input)?;
		let mut chunks = Vec::with_capacity(archive.len());

		for i in 0..archive.len() {
			let mut file = archive.by_index(i)?;
			if file
				.name()
				.trim_matches(|c| char::is_numeric(c) || c == '-')
				== "_"
			{
				let mut buffer = Vec::with_capacity(file.size() as usize);
				file.read_to_end(&mut buffer)?;
				let chunk: Environment = buffer
					.read(&mut 0)
					.map_err(|err| anyhow!("Read error: {:?}", err))?;
				chunks.push(chunk);
			}
		}
		Ok(EnvironmentChunk { chunks })
	}

	pub fn get_chunks(&self) -> &Vec<Environment> {
		&self.chunks
	}
}