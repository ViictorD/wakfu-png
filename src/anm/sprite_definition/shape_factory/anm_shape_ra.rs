use bytebuffer::ByteBuffer;
use crate::anm::processing::anm_transform::AnmTransform;

use super::{anm_shape::AnmShapeTrait, anm_shape_r::AnmShapeR};
use anyhow::Result;

#[derive(Clone)]
pub struct AnmShapeRA {
	pub shape_r: AnmShapeR,
	pub add_red: i16,
	pub add_green: i16,
	pub add_blue: i16,
	pub add_alpha: i16
}

impl AnmShapeTrait for AnmShapeRA {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let shape_r = AnmShapeR::load(buffer, id)?;
		let add_red = buffer.read_i16()?;
		let add_green = buffer.read_i16()?;
		let add_blue = buffer.read_i16()?;
		let add_alpha = buffer.read_i16()?;

		let result = AnmShapeRA {
			shape_r,
			add_red,
			add_green,
			add_blue,
			add_alpha,
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
		result.red = parent.red + self.add_red as f32 / 256.;
		result.green = parent.green + self.add_green as f32 / 256.;
		result.blue = parent.blue + self.add_blue as f32 / 256.;
		result.alpha = parent.alpha + self.add_alpha as f32 / 256.;
	}
}