use bytebuffer::ByteBuffer;
use anyhow::Result;
use super::anm_action::AnmActionTrait;

#[derive(Clone)]
pub struct AnmActionGoToStaticAnimation;

impl AnmActionTrait for AnmActionGoToStaticAnimation {
	fn load(_parameters_count: i8, _buffer: &mut ByteBuffer) -> Result<Self> {
		Ok(AnmActionGoToStaticAnimation)
	}

	fn get_type(&self) -> &'static str {
		"GO_TO_STATIC_ANIMATION"
	}
}