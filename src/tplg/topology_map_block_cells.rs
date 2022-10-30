use byte::{BytesExt, TryRead, ctx::Endian};

use super::{topology_map::TopologyMap, byte_array_bit_set::ByteArrayBitSet};

pub struct TopologyMapBlockCells {
	pub topology_map: TopologyMap,
	blocked_cells: ByteArrayBitSet
}

impl<'a> TryRead<'a> for TopologyMapBlockCells {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let topology_map: TopologyMap = bytes.read(offset)?;

		let blocked_cells: Vec<u8> = bytes
			.read_iter(offset, Endian::default())
			.take(ByteArrayBitSet::get_data_length(324))
			.collect();

		let result = TopologyMapBlockCells {
			topology_map,
			blocked_cells: ByteArrayBitSet::new(blocked_cells)
		};

		Ok((result, *offset))
	}
}

impl TopologyMapBlockCells {
	pub fn is_in_map(&self, x: i32, y: i32) -> bool {
		self.topology_map.is_in_map(x, y)
	}

	pub fn is_cell_blocked(&self, x: i32, y: i32) -> bool {
		self.blocked_cells.get(((y - self.topology_map.y) * 18 + x - self.topology_map.x) as u32)
	}
}