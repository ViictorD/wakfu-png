use bytebuffer::ByteBuffer;
use anyhow::Result;
use super::anm_action::AnmActionTrait;

#[derive(Clone)]
pub struct AnmActionDelete;

impl AnmActionTrait for AnmActionDelete {
	fn load(_parameters_count: i8, _buffer: &mut ByteBuffer) -> Result<Self> {
		Ok(AnmActionDelete)
	}

	fn get_type(&self) -> &'static str {
		"DELETE"
	}
}