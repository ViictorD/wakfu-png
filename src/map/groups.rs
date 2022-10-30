use std::io::{Read, Seek};

use anyhow::{Result, anyhow};
use byte::{TryRead, BytesExt, ctx::Endian};

use crate::tplg::byte_array_bit_set::ByteArrayBitSet;

pub struct Groups {
	layers: ByteArrayBitSet,
	layer_count: u32
}

impl<'a> TryRead<'a> for Groups {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let layer_count = (1 + bytes.read::<u16>(offset)?) as u32;
		
		let mut layers = ByteArrayBitSet::new_empty((layer_count * layer_count) as usize);

		let len: u16 = bytes.read(offset)?;
		for _ in 0..len {
			let id: i32 = bytes.read::<i16>(offset)? as i32;
			let n = bytes.read::<u16>(offset)? as usize;
			let layers_visible: Vec<i16> = bytes
				.read_iter(offset, Endian::default())
				.take(n)
				.collect();
			
			let offset = (id.abs() as u32 * layer_count) as u32;
			for i in 0..layers_visible.len() {
				layers.insert(offset + layers_visible.get(i).unwrap().abs() as u32, true);
			}
		}

		let result = Groups {
			layers,
			layer_count
		};
		Ok((result, *offset))
	}
}

impl Groups {
	pub fn load<R: Read + Seek>(input: R) -> Result<Groups> {
		let mut archive = zip::ZipArchive::new(input)?;

		for i in 0..archive.len() {
			let mut file = archive.by_index(i)?;
			if file.name().eq("groups.lib") {
				let mut buffer = Vec::with_capacity(file.size() as usize);
				file.read_to_end(&mut buffer)?;
				let groups = buffer
					.read(&mut 0)
					.map_err(|err| anyhow!("Read error: {:?}", err))?;
				return Ok(groups);
			}
		}
		Err(anyhow!("groups.lib file not found"))
	}

	pub fn is_layer_visible(&self, from: i32, layer: i32) -> bool {
		if from == 0 {
			return layer <= 0;
		}
		self.layers.get(from.abs() as u32 * self.layer_count + layer.abs() as u32)
	}
}