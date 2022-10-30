use byte::{BytesExt, TryRead};

use super::{TopologyMapTrait, topology_map_block_cells::TopologyMapBlockCells, cell_path_data::CellPathData};

pub struct TopologyMapB {
	topology_map_blocked_cells: TopologyMapBlockCells,
	cost: Vec<i8>,
	murfin: Vec<i8>,
	property: Vec<i16>
}

impl<'a> TryRead<'a> for TopologyMapB {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let topology_map_blocked_cells: TopologyMapBlockCells = bytes.read(offset)?;

		let len = 324;
		let mut cost = Vec::with_capacity(len);
		let mut murfin = Vec::with_capacity(len);
		let mut property = Vec::with_capacity(len);

		for _ in 0..len {
			cost.push(bytes.read(offset)?);
			murfin.push(bytes.read(offset)?);
			property.push(bytes.read(offset)?);
		}

		let result = TopologyMapB {
			topology_map_blocked_cells,
			cost,
			murfin,
			property
		};

		Ok((result, *offset))
	}
}

impl TopologyMapTrait for TopologyMapB {
	fn is_in_map(&self, x: i32, y: i32) -> bool {
		self.topology_map_blocked_cells.is_in_map(x, y)
	}

	fn get_path_data(&self, x: i32, y: i32, cell_path_data: &mut Vec<CellPathData>) -> usize {
		let cell_index = self.get_index(x, y);
		let data = CellPathData {
			x,
			y,
			z: self.topology_map_blocked_cells.topology_map.z,
			height: 0,
			hollow: false,
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

impl TopologyMapB {
	pub fn get_index(&self, x: i32, y: i32) -> usize {
		let x_index = x - self.topology_map_blocked_cells.topology_map.x;
		let y_index = y - self.topology_map_blocked_cells.topology_map.y;
		(y_index * 18 + x_index) as usize
	}
}