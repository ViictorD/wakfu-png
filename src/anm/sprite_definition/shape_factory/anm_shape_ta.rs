use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait, anm_shape_t::AnmShapeT};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeTA {
	pub shape_t: AnmShapeT,
	pub add_red: i16,
	pub add_green: i16,
	pub add_blue: i16,
	pub add_alpha: i16
}

impl AnmShapeTrait for AnmShapeTA {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let shape_t = AnmShapeT::load(buffer, id)?;
		let add_red = buffer.read_i16()?;
		let add_green = buffer.read_i16()?;
		let add_blue = buffer.read_i16()?;
		let add_alpha = buffer.read_i16()?;

		let result = AnmShapeTA {
			shape_t,
			add_red,
			add_green,
			add_blue,
			add_alpha,
		};

		Ok(result)
	}

	fn get_id(&self) -> i16 {
		self.shape_t.id
	}

	fn process(&self, parent: &AnmTransform, result: &mut AnmTransform) {
		result.rotation_is_identity = parent.rotation_is_identity;
		result.rotation_skew_x0 = parent.rotation_skew_x0;
		result.rotation_skew_x1 = parent.rotation_skew_x1;
		result.rotation_skew_y0 = parent.rotation_skew_y0;
		result.rotation_skew_y1 = parent.rotation_skew_y1;
		result.translation_is_identity = false;

		let tx = self.shape_t.translation_x as f32 / 16.;
		let ty = self.shape_t.translation_y as f32 / 16.;
		if parent.rotation_is_identity {
			result.translation_x = tx + parent.translation_x;
			result.translation_y = ty + parent.translation_y;
		}
		else {
			result.translation_x = tx * parent.rotation_skew_x0 + ty * parent.rotation_skew_x1 + parent.translation_x;
			result.translation_y = tx + parent.rotation_skew_y0 + ty * parent.rotation_skew_y1 + parent.translation_y;
		}
		result.red = parent.red + self.add_red as f32 * 0.00390625;
		result.green = parent.green + self.add_green as f32 * 0.00390625;
		result.blue = parent.blue + self.add_blue as f32 * 0.00390625;
		result.alpha = parent.alpha + self.add_alpha as f32 * 0.00390625;
	}
}