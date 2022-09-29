use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeCT {
	pub id: i16,
	pub translation_x: i16,
	pub translation_y: i16
}

impl AnmShapeTrait for AnmShapeCT {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let translation_x = buffer.read_i16()?;
		let translation_y = buffer.read_i16()?;

		let result = AnmShapeCT {
			id,
			translation_x,
			translation_y
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
		if parent.rotation_is_identity {
			result.translation_x = self.translation_x as f32 / 256. + parent.translation_x;
			result.translation_y = self.translation_y as f32 / 256. + parent.translation_y;
		}
		else {
			result.translation_x = self.translation_x as f32 / 256. * parent.rotation_skew_x0 + self.translation_y as f32 / 256. * parent.rotation_skew_x1 + parent.translation_x;
			result.translation_y = self.translation_x as f32 / 256. * parent.rotation_skew_y0 + self.translation_y as f32 / 256. * parent.rotation_skew_y1 + parent.translation_y;
		}
		result.red = parent.red;
		result.green = parent.green;
		result.blue = parent.blue;
		result.alpha = parent.alpha;
	}
}