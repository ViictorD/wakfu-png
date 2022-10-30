use byte::{BytesExt, TryRead};

use super::{topology_map::TopologyMap, TopologyMapTrait, cell_path_data::CellPathData};

pub struct TopologyMapA {
	topology_map: TopologyMap,
	cost: i8,
	murfin: i8,
	property: i16
}

impl<'a> TryRead<'a> for TopologyMapA {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let topology_map: TopologyMap = bytes.read(offset)?;
		let cost: i8 = bytes.read(offset)?;
		let murfin: i8 = bytes.read(offset)?;
		let property: i16 = bytes.read(offset)?;

		let result = TopologyMapA {
			topology_map,
			cost,
			murfin,
			property
		};

		Ok((result, *offset))
	}
}

impl TopologyMapTrait for TopologyMapA {
	fn is_in_map(&self, x: i32, y: i32) -> bool {
		self.topology_map.is_in_map(x, y)
	}

	fn get_path_data(&self, x: i32, y: i32, cell_path_data: &mut Vec<CellPathData>) -> usize {
		let data = CellPathData {
			x,
			y,
			z: self.topology_map.z,
			height: 0,
			hollow: false,
			cost: self.cost,
			murfin_info: self.murfin,
			misc_properties: self.property
		};
		cell_path_data.push(data);
		1
	}

	fn is_cell_blocked(&self, _x: i32, _y: i32) -> bool {
		self.cost == -1
	}
}