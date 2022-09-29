use anyhow::Result;
use bytebuffer::ByteBuffer;

use crate::anm::{anm::AnmTransformDataTable, processing::anm_transform::AnmTransform};

use super::{
	sprite_def::SpriteDef,
	sprite_definition::SpriteDefinitionTrait,
	anm_frame_data::{AnmFrameData, DataContainer},
	shape_factory::anm_frame_processor::read_and_process
};

#[derive(Clone)]
pub struct AnmSpriteDefinitionSingleFrame {
	table: AnmTransformDataTable,
	sprite_def: SpriteDef,
	sprite_ids: Vec<i16>,
	action_info: Vec<i16>,
	frame_data: DataContainer,
	current_sprite: i32
}

impl SpriteDefinitionTrait for AnmSpriteDefinitionSingleFrame {
	fn new(table: &AnmTransformDataTable, _optimized: bool) -> Result<Self> {
		let result = AnmSpriteDefinitionSingleFrame {
			table: table.clone(),
			sprite_def: SpriteDef::new(),
			sprite_ids: Vec::new(),
			action_info: Vec::new(),
			frame_data: DataContainer::None,
			current_sprite: 0
		};
		Ok(result)
	}
	fn load(&mut self, buffer: &mut ByteBuffer) {
		self.sprite_def.load(buffer);
		let sprite_count = buffer.read_i16().unwrap();
		if sprite_count == 0 {
			self.sprite_ids = Vec::new();
		}
		else {
			self.sprite_ids = Vec::with_capacity(sprite_count as usize);
			for _ in 0..sprite_count {
				self.sprite_ids.push(buffer.read_i16().unwrap());
			}
		}
		let action_count = buffer.read_i16().unwrap();
		if action_count == 0 {
			self.action_info = Vec::new()
		}
		else {
			self.action_info = Vec::with_capacity(action_count as usize);
			for _ in 0..action_count {
				self.action_info.push(buffer.read_i16().unwrap());
			}
		}

		self.frame_data = AnmFrameData::create(buffer).unwrap();
		if sprite_count == 0 && action_count != 0 {
			self.sprite_def.is_animation_node = true;
		}
		self.sprite_def.max_sprite_count = sprite_count as i32;
	}

	fn get_frame_count(&self) -> i32 {
		1
	}
	
	fn get_sprite_def(&self) -> &SpriteDef {
		&self.sprite_def
	}

	fn begin_process_frame(&mut self, _index: i32) -> i32 {
		match self.frame_data {
			DataContainer::I8(ref mut e) => e.begin(0),
			DataContainer::I16(ref mut e) => e.begin(0),
			DataContainer::I32(ref mut e) => e.begin(0),
			_ => {}
		}
		self.current_sprite = -1;
		self.sprite_ids.len() as i32
	}

	fn next_sprite(&mut self) {
		self.current_sprite += 1;
	}

	fn process(&mut self, p0: &AnmTransform, p1: &mut AnmTransform) -> i16 {
		let fp_type = self.frame_data.read();
		read_and_process(fp_type, &mut self.frame_data, &self.table, p0, p1);
		*self.sprite_ids.get(self.current_sprite as usize).unwrap()
	}
}