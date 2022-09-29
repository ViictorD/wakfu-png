use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait, anm_shape_r::AnmShapeR};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeRM {
	pub shape_r: AnmShapeR,
	pub mul_red: i8,
	pub mul_green: i8,
	pub mul_blue: i8,
	pub mul_alpha: i8
}

impl AnmShapeTrait for AnmShapeRM {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let shape_r = AnmShapeR::load(buffer, id)?;
		let mul_red = buffer.read_i8()?;
		let mul_green = buffer.read_i8()?;
		let mul_blue = buffer.read_i8()?;
		let mul_alpha = buffer.read_i8()?;

		let result = AnmShapeRM {
			shape_r,
			mul_red,
			mul_green,
			mul_blue,
			mul_alpha,
		};

		Ok(result)
	}

	fn get_id(&self) -> i16 {
		self.shape_r.id
	}

	fn process(&self, parent: &AnmTransform, result: &mut AnmTransform) {
		result.rotation_is_identity = false;

		if parent.translation_is_identity {
			result.rotation_skew_x0 = self.shape_r.rotation_skew_x0 as f32 / 256.;
			result.rotation_skew_y0 = self.shape_r.rotation_skew_y0 as f32 / 256.;
			result.rotation_skew_x1 = self.shape_r.rotation_skew_x1 as f32 / 256.;
			result.rotation_skew_y1 = self.shape_r.rotation_skew_y1 as f32 / 256.;
		}
		else {
			result.rotation_skew_x0 = self.shape_r.rotation_skew_x0 as f32 / 256. * parent.rotation_skew_x0 as f32 + self.shape_r.rotation_skew_y0 as f32 / 256. * parent.rotation_skew_x1;
			result.rotation_skew_y0 = self.shape_r.rotation_skew_x0 as f32 / 256. * parent.rotation_skew_y0 as f32 + self.shape_r.rotation_skew_y0 as f32 / 256. * parent.rotation_skew_y1;
			result.rotation_skew_x1 = self.shape_r.rotation_skew_x1 as f32 / 256. * parent.rotation_skew_x0 as f32 + self.shape_r.rotation_skew_y1 as f32 / 256. * parent.rotation_skew_x1;
			result.rotation_skew_y1 = self.shape_r.rotation_skew_x1 as f32 / 256. * parent.rotation_skew_y0 as f32 + self.shape_r.rotation_skew_y1 as f32 / 256. * parent.rotation_skew_y1;
		}
		result.translation_is_identity = parent.translation_is_identity;
		result.translation_x = parent.translation_x;
		result.translation_y = parent.translation_y;
		result.red = parent.red * (self.mul_red as f32 / 127.);
		result.green = parent.green * (self.mul_green as f32 / 127.);
		result.blue = parent.blue * (self.mul_blue as f32/ 127.);
		result.alpha = parent.alpha * (self.mul_alpha as f32 / 127.);
	}
}