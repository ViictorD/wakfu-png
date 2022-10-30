use byte::{BytesExt, TryRead, ctx::Endian};

use super::{TopologyMapTrait, topology_map_block_cells::TopologyMapBlockCells, cell_path_data::CellPathData, topology_indexer_helper::TopologyIndexerHelper};

pub struct TopologyMapDi {
	topology_map_blocked_cells: TopologyMapBlockCells,
	cost: Vec<i8>,
	murfin: Vec<i8>,
	property: Vec<i16>,
	zs: Vec<i16>,
	heights: Vec<i8>,
	mov_los: Vec<i8>,
	cells: Vec<i64>,
	cells_with_multi_z: Vec<i32>
}

impl<'a> TryRead<'a> for TopologyMapDi {
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

		let remains_count = bytes.read::<u16>(offset)? as usize;
		let cells_with_multi_z: Vec<i32> = bytes
			.read_iter(offset, Endian::default())
			.take(remains_count)
			.collect();

		let result = TopologyMapDi {
			topology_map_blocked_cells,
			cost,
			murfin,
			property,
			zs,
			heights,
			mov_los,
			cells,
			cells_with_multi_z
		};

		Ok((result, *offset))
	}
}

impl TopologyMapTrait for TopologyMapDi {
	fn is_in_map(&self, x: i32, y: i32) -> bool {
		self.topology_map_blocked_cells.is_in_map(x, y)
	}

	fn get_path_data(&self, x: i32, y: i32, cell_path_data: &mut Vec<CellPathData>) -> usize {
		let cell_index = self.get_index(x, y);
		if cell_index != 0 {
			let mut data = CellPathData::default();
			data.x = x;
			data.y = y;
			self.fill_path_data(&mut data, cell_index - 1);
			cell_path_data.push(data);
			return 1;
		}
		let tab = self.get_multi_index(
			x - self.topology_map_blocked_cells.topology_map.x,
			y - self.topology_map_blocked_cells.topology_map.y
		);
		let len = tab.len();
		for i in 0..len {
			let mut data = CellPathData::default();
			data.x = x;
			data.y = y;
			self.fill_path_data(&mut data, *tab.get(i).unwrap());
			cell_path_data.push(data);
		}
		return len;
	}

	fn is_cell_blocked(&self, x: i32, y: i32) -> bool {
		self.topology_map_blocked_cells.is_cell_blocked(x, y)
	}
}

impl TopologyMapDi {
	pub fn get_index(&self, x: i32, y: i32) -> usize {
		let x_index = x - self.topology_map_blocked_cells.topology_map.x;
		let y_index = y - self.topology_map_blocked_cells.topology_map.y;
		let cell_index = (y_index * 18 + x_index) as usize;
		TopologyIndexerHelper::get_index_i64(&self.cells, cell_index, self.cost.len() + 1)
	}

	pub fn fill_path_data(&self, data: &mut CellPathData, cell_index: usize) {
		data.z = *self.zs.get(cell_index).unwrap();
		data.hollow = (*self.mov_los.get(cell_index).unwrap()) & 0x1 == 0x1;
		data.height = *self.heights.get(cell_index).unwrap();
		data.cost = *self.cost.get(cell_index).unwrap();
		data.murfin_info = *self.murfin.get(cell_index).unwrap();
		data.misc_properties = *self.property.get(cell_index).unwrap();
	}

	pub fn get_multi_index(&self, x: i32, y: i32) -> Vec<usize> {
		let mut lst = Vec::new();
		for cell_data in &self.cells_with_multi_z {
			let cy = (*cell_data >> 8) & 0xFF;
			if cy >= y {
				if cy > y {
					break ;
				}
				let cx = *cell_data & 0xFF;
				if cx >= x {
					if cx > x {
						break ;
					}
					let index = (*cell_data >> 16) & 0xFFFF;
					lst.push(index as usize);
				}
			}
		}
		lst
	}
}