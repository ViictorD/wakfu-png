use bytebuffer::ByteBuffer;
use anyhow::Result;
use super::anm_action::AnmActionTrait;

#[derive(Clone)]
pub struct AnmActionEnd;

impl AnmActionTrait for AnmActionEnd {
	fn load(_parameters_count: i8, _buffer: &mut ByteBuffer) -> Result<Self> {
		Ok(AnmActionEnd)
	}

	fn get_type(&self) -> &'static str {
		"END"
	}
}