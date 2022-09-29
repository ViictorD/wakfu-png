use anyhow::Result;
use bytebuffer::ByteBuffer;
use crate::utils::utils::read_string_without_len;
use super::anm_action::AnmActionTrait;


#[derive(Clone)]
pub struct AnmActionGoToAnimation {
	_animation_name: String,
	_percent: i8,
}


impl AnmActionTrait for AnmActionGoToAnimation {
	fn load(parameters_count: i8, buffer: &mut ByteBuffer) -> Result<Self> {
		let _animation_name = read_string_without_len(buffer)?;
		let _percent = 
			if parameters_count == 2 { buffer.read_i8()? }
			else { 0 };
		let result = AnmActionGoToAnimation {
			_animation_name,
			_percent
		};
		Ok(result)
	}

	fn get_type(&self) -> &'static str {
		"GO_TO_ANIMATION"
	}
}