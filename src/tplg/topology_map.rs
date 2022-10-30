use byte::{BytesExt, TryRead};

pub struct TopologyMap {
	pub x: i32,
	pub y: i32,
	pub z: i16
}

impl<'a> TryRead<'a> for TopologyMap {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let x: i32 = bytes.read::<i16>(offset)? as i32 * 18;
		let y: i32 = bytes.read::<i16>(offset)? as i32 * 18;
		let z: i16 = bytes.read(offset)?;

		let result = TopologyMap {
			x,
			y,
			z
		};

		Ok((result, *offset))
	}
}

impl TopologyMap {
	pub fn is_in_map(&self, x: i32, y: i32) -> bool {
		x >= self.x && x < self.x + 18 && y >= self.y && y < self.y + 18
	}
}