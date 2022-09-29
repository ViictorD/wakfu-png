use bytebuffer::ByteBuffer;
use anyhow::Result;
use super::anm_action::AnmActionTrait;
use crate::utils::utils::read_string_without_len;

#[derive(Clone)]
pub struct AnmActionGoToIfPreviousAnimation {
	_previous_animations: Vec<String>,
	_next_animations: Vec<String>,
	_default_animation: String
}

impl AnmActionTrait for AnmActionGoToIfPreviousAnimation {
	fn load(parameters_count: i8, buffer: &mut ByteBuffer) -> Result<Self> {
		let if_count: i32 = (parameters_count as i32 - 1) / 2;
		let mut _previous_animations = Vec::with_capacity(if_count as usize);
		let mut _next_animations = Vec::with_capacity(if_count as usize);

		for _ in 0..if_count {
			_previous_animations.push(read_string_without_len(buffer)?);
			_next_animations.push(read_string_without_len(buffer)?);
		}
		let _default_animation =
			if parameters_count % 2 == 1 { read_string_without_len(buffer)? }
			else { String::default() };

		Ok(AnmActionGoToIfPreviousAnimation {
			_previous_animations,
			_next_animations,
			_default_animation
		})
	}

	fn get_type(&self) -> &'static str {
		"GO_TO_IF_PREVIOUS_ANIMATION"
	}
}