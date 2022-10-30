use std::io::{Read, Seek};
use anyhow::{anyhow, Result};
use byte::{BytesExt, TryRead};

use self::{
	topology_map_a::TopologyMapA,
	topology_map_b::TopologyMapB,
	topology_map_bi::TopologyMapBi,
	topology_map_c::TopologyMapC,
	topology_map_ci::TopologyMapCi,
	topology_map_di::TopologyMapDi,
	cell_path_data::CellPathData
};

mod topology_map;
mod topology_map_a;
mod topology_map_block_cells;
mod topology_map_b;
mod topology_map_bi;
mod topology_map_c;
mod topology_map_ci;
mod topology_map_di;
mod cell_path_data;
mod topology_indexer_helper;
pub mod byte_array_bit_set;

pub trait TopologyMapTrait {
	fn is_in_map(&self, x: i32, y: i32) -> bool;
	fn get_path_data(&self, x: i32, y: i32, cell_path_data: &mut Vec<CellPathData>) -> usize;
	fn is_cell_blocked(&self, x: i32, y: i32) -> bool;
}

pub struct TplgChunk {
	topology_map: Box<dyn TopologyMapTrait>
}

impl<'a> TryRead<'a> for TplgChunk {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let header: i8 = bytes.read(offset)?;

		let topology_map: Box<dyn TopologyMapTrait> = match header {
			0 => Box::new(bytes.read::<TopologyMapA>(offset)?),
			1 => Box::new(bytes.read::<TopologyMapB>(offset)?),
			2 => Box::new(bytes.read::<TopologyMapBi>(offset)?),
			3 => Box::new(bytes.read::<TopologyMapC>(offset)?),
			4 => Box::new(bytes.read::<TopologyMapCi>(offset)?),
			5 => Box::new(bytes.read::<TopologyMapDi>(offset)?),
			_ => return Err(byte::Error::BadInput { err: "Unknown topology map type" }),
		};

		let result = TplgChunk {
			topology_map
		};
		Ok((result, *offset))
	}
}

pub struct Tplg {
	chunks: Vec<TplgChunk>
}

impl Tplg {
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
				let chunk = buffer
					.read::<TplgChunk>(&mut 0)
					.map_err(|err| anyhow!("Read error: {:?}", err))?;
				chunks.push(chunk);
			}
		}
		Ok(Tplg { chunks })
	}

	pub fn is_blocked(&self, x: i32, y: i32, z: i32, height: i8) -> bool {
		for chunk in &self.chunks {
			if chunk.topology_map.is_in_map(x, y) {
				if chunk.topology_map.is_cell_blocked(x, y) {
					return true;
				}
				let mut cell_path_data = Vec::with_capacity(1);
				chunk.topology_map.get_path_data(x, y, &mut cell_path_data);
				for data in &cell_path_data {
					if data.z as i32 == z && height == data.height {
						return if data.cost == -1 { true } else { false };
					}
				}
				return true;
			}
		}
		true
	}
}