use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeM {
	pub id: i16,
	pub mul_red: i8,
	pub mul_green: i8,
	pub mul_blue: i8,
	pub mul_alpha: i8
}

impl AnmShapeTrait for AnmShapeM {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let mul_red = buffer.read_i8()?;
		let mul_green = buffer.read_i8()?;
		let mul_blue = buffer.read_i8()?;
		let mul_alpha = buffer.read_i8()?;

		let result = AnmShapeM {
			id,
			mul_red,
			mul_green,
			mul_blue,
			mul_alpha,
		};

		Ok(result)
	}

	fn get_id(&self) -> i16 {
		self.id
	}

	fn process(&self, parent: &AnmTransform, result: &mut AnmTransform) {
		result.rotation_is_identity = parent.rotation_is_identity;
		result.rotation_skew_x0 = parent.rotation_skew_x0;
		result.rotation_skew_x1 = parent.rotation_skew_x1;
		result.rotation_skew_y0 = parent.rotation_skew_y0;
		result.rotation_skew_y1 = parent.rotation_skew_y1;
		result.translation_is_identity = parent.translation_is_identity;
		result.translation_x = parent.translation_x;
		result.translation_y = parent.translation_y;
		result.red = parent.red * (self.mul_red as f32 / 127.);
		result.green = parent.green * (self.mul_green as f32 / 127.);
		result.blue = parent.blue * (self.mul_blue as f32 / 127.);
		result.alpha = parent.alpha * (self.mul_alpha as f32 / 127.);
	}
}