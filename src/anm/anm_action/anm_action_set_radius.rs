use bytebuffer::ByteBuffer;
use anyhow::Result;
use super::anm_action::AnmActionTrait;

#[derive(Clone)]
pub struct AnmActionSetRadius {
	_radius: i8
}

impl AnmActionTrait for AnmActionSetRadius {
	fn load(_parameters_count: i8, buffer: &mut ByteBuffer) -> Result<Self> {
		let _radius = buffer.read_i8()?;

		let result = AnmActionSetRadius {
			_radius
		};
		Ok(result)
	}

	fn get_type(&self) -> &'static str {
		"SET_RADIUS"
	}
}