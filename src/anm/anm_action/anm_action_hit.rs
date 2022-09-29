use bytebuffer::ByteBuffer;
use anyhow::Result;
use super::anm_action::AnmActionTrait;

#[derive(Clone)]
pub struct AnmActionHit;

impl AnmActionTrait for AnmActionHit {
	fn load(_parameters_count: i8, _buffer: &mut ByteBuffer) -> Result<Self> {
		Ok(AnmActionHit)
	}

	fn get_type(&self) -> &'static str {
		"HIT"
	}
}