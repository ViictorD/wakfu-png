pub struct CellPathData {
	pub x: i32,
	pub y: i32,
	pub z: i16,
	pub height: i8,
	pub hollow: bool,
	pub cost: i8,
	pub murfin_info: i8,
	pub misc_properties: i16
}

impl Default for CellPathData {
	fn default() -> Self {
		CellPathData {
			x: 0,
			y: 0,
			z: 0,
			height: 0,
			hollow: false,
			cost: 0,
			murfin_info: 0,
			misc_properties: 0
		}
	}
}