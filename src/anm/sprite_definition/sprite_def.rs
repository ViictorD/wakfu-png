use bytebuffer::ByteBuffer;
use crate::utils::utils::{java_string_hashcode, read_string_without_len};

#[derive(Clone)]
pub struct SpriteDef {
	pub max_sprite_count: i32,
	pub id: i16,
	pub flags: i8,
	pub name: String,
	pub name_crc: i32,
	pub base_name_crc: i32,
	pub is_animation_node: bool
}

impl SpriteDef {
	pub fn new() -> Self {
		SpriteDef {
			max_sprite_count: -1,
			id: 0,
			flags: 0,
			name: String::default(),
			name_crc: 0,
			base_name_crc: 0,
			is_animation_node: false
		}
	}

	pub fn load(&mut self, buffer: &mut ByteBuffer) {
		self.id = buffer.read_i16().unwrap();
		let flags = buffer.read_i8().unwrap();
		if (flags & 0x40) != 0x0 {
			self.name = read_string_without_len(buffer).unwrap();
			self.name_crc = java_string_hashcode(&self.name);
			if let Some(index) = self.name.clone().into_bytes().iter().position(|b| *b == 95) {
				self.base_name_crc = java_string_hashcode(&(self.name[(index + 1) as usize..]).to_string());
			}
			else {
				self.base_name_crc = 0;
			}
		}
		else {
			self.base_name_crc = 0;
			self.name_crc = 0;
		}
		buffer.read_i32().unwrap();
		buffer.read_i32().unwrap();
	}

	pub fn is_loop(&self) -> bool {
		(self.flags as u8 & 0x80) != 0x0
	}

	pub fn get_color_index(&self) -> i32 {
		(self.flags & 0x3F) as i32
	}
}