use anyhow::Result;
use bytebuffer::ByteBuffer;

use crate::anm::processing::anm_transform::AnmTransform;

pub trait AnmShapeTrait: AnmShapeClone {
	fn load(buffer: &mut ByteBuffer, id: i16) -> Result<Self> where Self: Sized;
	fn process(&self, parent: &AnmTransform, result: &mut AnmTransform);
	fn get_id(&self) -> i16;
}

pub trait AnmShapeClone {
	fn clone_box(&self) -> Box<dyn AnmShapeTrait>;
}

impl<T> AnmShapeClone for T
where
	T: 'static + AnmShapeTrait + Clone,
{	
	fn clone_box(&self) -> Box<dyn AnmShapeTrait> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn AnmShapeTrait> {
	fn clone(&self) -> Self {
		self.clone_box()
	}
}

#[derive(Clone)]
pub struct AnmShape {
	id: i16
}

impl AnmShapeTrait for AnmShape {
	fn load(_buffer: &mut ByteBuffer, id: i16) -> Result<Self> {
		let result = AnmShape {
			id
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
		result.translation_is_identity = parent.translation_is_identity;
		result.translation_x = parent.translation_x;
		result.translation_y = parent.translation_y;
		result.red = parent.red;
		result.green = parent.green;
		result.blue = parent.blue;
		result.alpha = parent.alpha;
	}
}