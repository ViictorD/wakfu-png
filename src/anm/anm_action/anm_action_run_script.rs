use bytebuffer::ByteBuffer;
use anyhow::Result;
use super::anm_action::AnmActionTrait;
use crate::utils::utils::read_string_without_len;

#[derive(Clone)]
pub struct AnmActionRunScript {
	_script_id: i64
}

impl AnmActionTrait for AnmActionRunScript {
	fn load(_parameters_count: i8, buffer: &mut ByteBuffer) -> Result<Self> {
		let str_script_id = read_string_without_len(buffer)?;
		
		let mut _script_id = -1;
		if let Ok(id) = str_script_id.parse::<i64>() {
			_script_id = id;
		}
		
		let result = AnmActionRunScript {
			_script_id
		};

		Ok(result)
	}

	fn get_type(&self) -> &'static str {
		"RUN_SCRIPT"
	}
}