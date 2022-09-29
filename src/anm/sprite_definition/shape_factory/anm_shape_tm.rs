use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait, anm_shape_t::AnmShapeT};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeTM {
	pub shape_t: AnmShapeT,
	pub mul_red: i8,
	pub mul_green: i8,
	pub mul_blue: i8,
	pub mul_alpha: i8
}

impl AnmShapeTrait for AnmShapeTM {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let shape_t = AnmShapeT::load(buffer, id)?;
		let mul_red = buffer.read_i8()?;
		let mul_green = buffer.read_i8()?;
		let mul_blue = buffer.read_i8()?;
		let mul_alpha = buffer.read_i8()?;

		let result = AnmShapeTM {
			shape_t,
			mul_red,
			mul_green,
			mul_blue,
			mul_alpha,
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
		result.red = parent.red * (self.mul_red as f32 * 0.007874016);
		result.green = parent.green * (self.mul_green as f32 * 0.007874016);
		result.blue = parent.blue * (self.mul_blue as f32 * 0.007874016);
		result.alpha = parent.alpha * (self.mul_alpha as f32 * 0.007874016);
	}
}