use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::anm_shape::AnmShapeTrait;
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeT {
	pub id: i16,
	pub translation_x: i16,
	pub translation_y: i16,

}

impl AnmShapeTrait for AnmShapeT {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let translation_x = buffer.read_i16()?;
		let translation_y = buffer.read_i16()?;

		let result = AnmShapeT {
			id,
			translation_x,
			translation_y,
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
		result.translation_is_identity = false;

		let tx = self.translation_x as f32 / 16.;
		let ty = self.translation_y as f32 / 16.;
		if parent.rotation_is_identity {
			result.translation_x = tx + parent.translation_x;
			result.translation_y = ty + parent.translation_y;
		}
		else {
			result.translation_x = tx * parent.rotation_skew_x0 + ty * parent.rotation_skew_x1 + parent.translation_x;
			result.translation_y = tx + parent.rotation_skew_y0 + ty * parent.rotation_skew_y1 + parent.translation_y;
		}
		result.red = parent.red;
		result.green = parent.green;
		result.blue = parent.blue;
		result.alpha = parent.alpha;
	}
}