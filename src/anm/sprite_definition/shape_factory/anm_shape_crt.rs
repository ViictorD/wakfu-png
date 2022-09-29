use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait, anm_shape_cr::AnmShapeCR};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeCRT {
	pub shape_cr: AnmShapeCR,
	pub translation_x: i8,
	pub translation_y: i8
}

impl AnmShapeTrait for AnmShapeCRT {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let shape_cr = AnmShapeCR::load(buffer, id)?;
		let translation_x = buffer.read_i8()?;
		let translation_y = buffer.read_i8()?;

		let result = AnmShapeCRT {
			shape_cr,
			translation_x,
			translation_y
		};

		Ok(result)
	}

	fn get_id(&self) -> i16 {
		self.shape_cr.id
	}
	
	fn process(&self, parent: &AnmTransform, result: &mut AnmTransform) {
		result.rotation_is_identity = false;
		
		if parent.rotation_is_identity {
			result.rotation_skew_x0 = self.shape_cr.rotation_skew_x0 as f32 / 127.;
			result.rotation_skew_y0 = self.shape_cr.rotation_skew_y0 as f32 / 127.;
			result.rotation_skew_x1 = self.shape_cr.rotation_skew_x1 as f32 / 127.;
			result.rotation_skew_y1 = self.shape_cr.rotation_skew_y1 as f32 / 127.;
		}
		else {
			result.rotation_skew_x0 = self.shape_cr.rotation_skew_x0 as f32 / 127. * parent.rotation_skew_x0 + self.shape_cr.rotation_skew_y0 as f32 / 127. * parent.rotation_skew_x1;
			result.rotation_skew_y0 = self.shape_cr.rotation_skew_x0 as f32 / 127. * parent.rotation_skew_y0 + self.shape_cr.rotation_skew_y0 as f32 / 127. * parent.rotation_skew_y1;
			result.rotation_skew_x1 = self.shape_cr.rotation_skew_x1 as f32 / 127. * parent.rotation_skew_x0 + self.shape_cr.rotation_skew_y1 as f32 / 127. * parent.rotation_skew_x1;
			result.rotation_skew_y1 = self.shape_cr.rotation_skew_x1 as f32 / 127. * parent.rotation_skew_y0 + self.shape_cr.rotation_skew_y1 as f32 / 127. * parent.rotation_skew_y1;
		}

		result.translation_is_identity = false;
		let tx = self.translation_x as f32 * 16. / 127.;
		let ty = self.translation_y as f32 * 16. / 127.;

		if parent.rotation_is_identity {
			result.translation_x = tx + parent.translation_x;
			result.translation_y = ty+ parent.translation_y;
		}
		else {
			result.translation_x = tx * parent.rotation_skew_x0 + ty * parent.rotation_skew_x1 + parent.translation_x;
			result.translation_y = tx * parent.rotation_skew_y0 + ty * parent.rotation_skew_y1 + parent.translation_y;
		}

		result.red = parent.red;
		result.green = parent.green;
		result.blue = parent.blue;
		result.alpha = parent.alpha;
	}
}