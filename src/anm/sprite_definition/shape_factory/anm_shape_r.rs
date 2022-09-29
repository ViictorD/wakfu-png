use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::anm_shape::AnmShapeTrait;
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeR {
	pub id: i16,
	pub rotation_skew_x0: i16,
	pub rotation_skew_y0: i16,
	pub rotation_skew_x1: i16,
	pub rotation_skew_y1: i16
}

impl AnmShapeTrait for AnmShapeR {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let rotation_skew_x0 = buffer.read_i16()?;
		let rotation_skew_y0 = buffer.read_i16()?;
		let rotation_skew_x1 = buffer.read_i16()?;
		let rotation_skew_y1 = buffer.read_i16()?;

		let result = AnmShapeR {
			id,
			rotation_skew_x0,
			rotation_skew_y0,
			rotation_skew_x1,
			rotation_skew_y1
		};

		Ok(result)
	}

	fn get_id(&self) -> i16 {
		self.id
	}

	fn process(&self, parent: &AnmTransform, result: &mut AnmTransform) {
		result.rotation_is_identity = false;

		if parent.translation_is_identity {
			result.rotation_skew_x0 = self.rotation_skew_x0 as f32 / 256.;
			result.rotation_skew_y0 = self.rotation_skew_y0 as f32 / 256.;
			result.rotation_skew_x1 = self.rotation_skew_x1 as f32 / 256.;
			result.rotation_skew_y1 = self.rotation_skew_y1 as f32 / 256.;
		}
		else {
			result.rotation_skew_x0 = self.rotation_skew_x0 as f32 / 256. * parent.rotation_skew_x0 as f32 + self.rotation_skew_y0 as f32 / 256. * parent.rotation_skew_x1;
			result.rotation_skew_y0 = self.rotation_skew_x0 as f32 / 256. * parent.rotation_skew_y0 as f32 + self.rotation_skew_y0 as f32 / 256. * parent.rotation_skew_y1;
			result.rotation_skew_x1 = self.rotation_skew_x1 as f32 / 256. * parent.rotation_skew_x0 as f32 + self.rotation_skew_y1 as f32 / 256. * parent.rotation_skew_x1;
			result.rotation_skew_y1 = self.rotation_skew_x1 as f32 / 256. * parent.rotation_skew_y0 as f32 + self.rotation_skew_y1 as f32 / 256. * parent.rotation_skew_y1;
		}
		result.translation_is_identity = parent.translation_is_identity;
		result.translation_x = parent.translation_x;
		result.translation_y = parent.translation_y;
		result.red = parent.red;
		result.green = parent.green;
		result.blue = parent.blue;
		result.alpha = parent.alpha;
	}
}