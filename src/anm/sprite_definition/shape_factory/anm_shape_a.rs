use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::anm_shape::AnmShapeTrait;
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeA {
	pub id: i16,
	pub add_red: i16,
	pub add_green: i16,
	pub add_blue: i16,
	pub add_alpha: i16
}

impl AnmShapeTrait for AnmShapeA {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let add_red = buffer.read_i16()?;
		let add_green = buffer.read_i16()?;
		let add_blue = buffer.read_i16()?;
		let add_alpha = buffer.read_i16()?;

		let result = AnmShapeA {
			id,
			add_red,
			add_green,
			add_blue,
			add_alpha,
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
		result.red = parent.red + self.add_red as f32 / 256.;
		result.green = parent.green + self.add_green as f32 / 256.;
		result.blue = parent.blue + self.add_blue as f32 / 256.;
		result.alpha = parent.alpha + self.add_alpha as f32 / 256.;
	}
}