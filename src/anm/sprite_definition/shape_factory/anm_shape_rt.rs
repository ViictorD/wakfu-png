use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait, anm_shape_r::AnmShapeR};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeRT {
	pub shape_r: AnmShapeR,
	pub translation_x: i16,
	pub translation_y: i16,

}

impl AnmShapeTrait for AnmShapeRT {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let shape_r = AnmShapeR::load(buffer, id)?;
		let translation_x = (buffer.read_f32()? * 16.) as i16;
		let translation_y = (buffer.read_f32()? * 16.) as i16;

		let result = AnmShapeRT {
			shape_r,
			translation_x,
			translation_y,
		};

		Ok(result)
	}

	fn get_id(&self) -> i16 {
		self.shape_r.id
	}

	fn process(&self, parent: &AnmTransform, result: &mut AnmTransform) {
		result.rotation_is_identity = false;
		result.translation_is_identity = false;

		let tx = self.translation_x as f32 / 16.;
		let ty = self.translation_y as f32 / 16.;
		let rx0 = self.shape_r.rotation_skew_x0 as f32 / 256.;
		let ry0 = self.shape_r.rotation_skew_y0 as f32 / 256.;
		let rx = self.shape_r.rotation_skew_x1 as f32 / 256.;
		let ry = self.shape_r.rotation_skew_y1 as f32 / 256.;
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
		result.red = parent.red;
		result.green = parent.green;
		result.blue = parent.blue;
		result.alpha = parent.alpha;
	}
}