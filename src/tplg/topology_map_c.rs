use byte::{BytesExt, TryRead};

use super::{TopologyMapTrait, topology_map_block_cells::TopologyMapBlockCells, cell_path_data::CellPathData};

pub struct TopologyMapC {
	topology_map_blocked_cells: TopologyMapBlockCells,
	cost: Vec<i8>,
	murfin: Vec<i8>,
	property: Vec<i16>,
	zs: Vec<i16>,
	heights: Vec<i8>,
	mov_los: Vec<i8>
}

impl<'a> TryRead<'a> for TopologyMapC {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let topology_map_blocked_cells: TopologyMapBlockCells = bytes.read(offset)?;

		let len = 324;
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

		let result = TopologyMapC {
			topology_map_blocked_cells,
			cost,
			murfin,
			property,
			zs,
			heights,
			mov_los
		};

		Ok((result, *offset))
	}
}

impl TopologyMapTrait for TopologyMapC {
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

impl TopologyMapC {
	pub fn get_index(&self, x: i32, y: i32) -> usize {
		let x_index = x - self.topology_map_blocked_cells.topology_map.x;
		let y_index = y - self.topology_map_blocked_cells.topology_map.y;
		(y_index * 18 + x_index) as usize
	}
}