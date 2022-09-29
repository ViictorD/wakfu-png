use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait, anm_shape_rta::AnmShapeRTA};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeRTAM {
	pub shape_rta: AnmShapeRTA,
	pub mul_red: i8,
	pub mul_green: i8,
	pub mul_blue: i8,
	pub mul_alpha: i8
}

impl AnmShapeTrait for AnmShapeRTAM {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let shape_rta = AnmShapeRTA::load(buffer, id)?;
		let mul_red = buffer.read_i8()?;
		let mul_green = buffer.read_i8()?;
		let mul_blue = buffer.read_i8()?;
		let mul_alpha = buffer.read_i8()?;

		let result = AnmShapeRTAM {
			shape_rta,
			mul_red,
			mul_green,
			mul_blue,
			mul_alpha,
		};

		Ok(result)
	}

	fn get_id(&self) -> i16 {
		self.shape_rta.shape_rt.shape_r.id
	}

	fn process(&self, parent: &AnmTransform, result: &mut AnmTransform) {
		result.rotation_is_identity = false;
		result.translation_is_identity = false;

		let tx = self.shape_rta.shape_rt.translation_x as f32 / 16.;
		let ty = self.shape_rta.shape_rt.translation_y as f32 / 16.;
		let rx0 = self.shape_rta.shape_rt.shape_r.rotation_skew_x0 as f32 / 256.;
		let ry0 = self.shape_rta.shape_rt.shape_r.rotation_skew_y0 as f32 / 256.;
		let rx = self.shape_rta.shape_rt.shape_r.rotation_skew_x1 as f32 / 256.;
		let ry = self.shape_rta.shape_rt.shape_r.rotation_skew_y1 as f32 / 256.;
		if parent.translation_is_identity {
			result.rotation_skew_x0 = rx0;
			result.rotation_skew_y0 = ry0;
			result.rotation_skew_x1 = rx;
			result.rotation_skew_y1 = ry;
			result.translation_x = tx + parent.translation_x;
			result.translation_y = ty + parent.translation_y;
		}
		else {
			result.rotation_skew_x0 = rx0 * parent.rotation_skew_x0 + ry0 * parent.rotation_skew_x1;
			result.rotation_skew_y0 = rx0 * parent.rotation_skew_y0 + ry0 * parent.rotation_skew_y1;
			result.rotation_skew_x1 = rx * parent.rotation_skew_x0 + ry * parent.rotation_skew_x1;
			result.rotation_skew_y1 = rx * parent.rotation_skew_y0 + ry * parent.rotation_skew_y1;
			result.translation_x = tx * parent.rotation_skew_x0 + ty * parent.rotation_skew_x1 + parent.translation_x;
			result.translation_y = tx * parent.rotation_skew_y0 + ty * parent.rotation_skew_y1 + parent.translation_y;
		}
		result.red = parent.red * (self.mul_red as f32 / 127.) + self.shape_rta.add_red as f32 / 256.;
		result.green = parent.green * (self.mul_green as f32 / 127.) + self.shape_rta.add_green as f32 / 256.;
		result.blue = parent.blue * (self.mul_blue as f32 / 127.) + self.shape_rta.add_blue as f32 / 256.;
		result.alpha = parent.alpha * (self.mul_alpha as f32 / 127.) + self.shape_rta.add_alpha as f32 / 256.;
	}
}