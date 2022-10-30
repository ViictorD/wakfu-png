use byte::{BytesExt, TryRead, ctx::Endian};

use super::{TopologyMapTrait, topology_map_block_cells::TopologyMapBlockCells, topology_indexer_helper::TopologyIndexerHelper, cell_path_data::CellPathData};

pub struct TopologyMapCi {
	topology_map_blocked_cells: TopologyMapBlockCells,
	cost: Vec<i8>,
	murfin: Vec<i8>,
	property: Vec<i16>,
	zs: Vec<i16>,
	heights: Vec<i8>,
	mov_los: Vec<i8>,
	cells: Vec<i64>
}

impl<'a> TryRead<'a> for TopologyMapCi {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let topology_map_blocked_cells: TopologyMapBlockCells = bytes.read(offset)?;

		let len = bytes.read::<u8>(offset)? as usize;
		let mut cost = Vec::with_capacity(len);
		let mut murfin = Vec::with_capacity(len);
		let mut property = Vec::with_capacity(len);
		let mut zs = Vec::with_capacity(len);
		let mut heights = Vec::with_capacity(len);
		let mut mov_los = Vec::with_capacity(len);

		for _ in 0..len {
			cost.push(bytes.read(offset)?);
			murfin.push(bytes.read(offset)?);
			property.push(bytes.read(offset)?);
			zs.push(bytes.read(offset)?);
			heights.push(bytes.read(offset)?);
			mov_los.push(bytes.read(offset)?);
		}

		let cell_len = bytes.read::<u8>(offset)? as usize;
		let cells: Vec<i64> = bytes
			.read_iter(offset, Endian::default())
			.take(cell_len)
			.collect();

		let result = TopologyMapCi {
			topology_map_blocked_cells,
			cost,
			murfin,
			property,
			zs,
			heights,
			mov_los,
			cells
		};

		Ok((result, *offset))
	}
}

impl TopologyMapTrait for TopologyMapCi {
	fn is_in_map(&self, x: i32, y: i32) -> bool {
		self.topology_map_blocked_cells.is_in_map(x, y)
	}

	fn get_path_data(&self, x: i32, y: i32, cell_path_data: &mut Vec<CellPathData>) -> usize {
		let cell_index = self.get_index(x, y);
		let data = CellPathData {
			x,
			y,
			z: *self.zs.get(cell_index).unwrap(),
			height: *self.heights.get(cell_index).unwrap(),
			hollow: (*self.mov_los.get(cell_index).unwrap()) & 0x1 == 0x1,
			cost: *self.cost.get(cell_index).unwrap(),
			murfin_info: *self.murfin.get(cell_index).unwrap(),
			misc_properties: *self.property.get(cell_index).unwrap()
		};
		cell_path_data.push(data);
		1
	}

	fn is_cell_blocked(&self, x: i32, y: i32) -> bool {
		self.topology_map_blocked_cells.is_cell_blocked(x, y)
	}
}

impl TopologyMapCi {
	pub fn get_index(&self, x: i32, y: i32) -> usize {
		let x_index = x - self.topology_map_blocked_cells.topology_map.x;
		let y_index = y - self.topology_map_blocked_cells.topology_map.y;
		let cell_index = (y_index * 18 + x_index) as usize;
		TopologyIndexerHelper::get_index_i64(&self.cells, cell_index, self.cost.len())
	}
}