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
pub struct AnmSpriteDefinitionIndexed {
	table: AnmTransformDataTable,
	sprite_def: SpriteDef,
	current_sprite: i32,
	frame_pos: Vec<i32>,
	sprite_info: Vec<i16>,
	action_info: Vec<i16>,
	frame_data: DataContainer
}

impl SpriteDefinitionTrait for AnmSpriteDefinitionIndexed {
	fn new(table: &AnmTransformDataTable, _optimized: bool) -> Result<Self> {
		let result = AnmSpriteDefinitionIndexed {
			table: table.clone(),
			sprite_def: SpriteDef::new(),
			current_sprite: -1,
			frame_pos: Vec::new(),
			sprite_info: Vec::new(),
			action_info: Vec::new(),
			frame_data: DataContainer::None
		};
		Ok(result)
	}
	fn load(&mut self, buffer: &mut ByteBuffer) {
		self.sprite_def.load(buffer);
		let mut count = buffer.read_i16().unwrap();

		self.frame_pos = Vec::with_capacity(count as usize);
		for _ in 0..count {
			self.frame_pos.push(buffer.read_i32().unwrap());
		}

		count = buffer.read_i16().unwrap();
		self.sprite_info = Vec::with_capacity(count as usize);
		for _ in 0..count {
			self.sprite_info.push(buffer.read_i16().unwrap());
		}

		count = buffer.read_i16().unwrap();
		for _ in 0..count {
			self.action_info.push(buffer.read_i16().unwrap());
		}


		self.frame_data = AnmFrameData::create(buffer).unwrap();

		for i in 0..self.sprite_info.len() {
			if *self.sprite_info.get(i as usize).unwrap() as i32 > self.sprite_def.max_sprite_count {
				self.sprite_def.max_sprite_count = *self.sprite_info.get(i as usize).unwrap() as i32;
			}
		}
	}

	fn get_frame_count(&self) -> i32 {
		if self.action_info.len() == 0 {
			return (self.frame_pos.len() / 2) as i32;
		}
		return (self.frame_pos.len() / 3) as i32;
	}
	
	fn get_sprite_def(&self) -> &SpriteDef {
		&self.sprite_def
	}

	fn begin_process_frame(&mut self, index: i32) -> i32 {
		let i =
			if self.action_info.len() == 0 { index * 2 }
			else { index * 3 };
		
		match self.frame_data {
			DataContainer::I8(ref mut e) => e.begin(*self.frame_pos.get(i as usize).unwrap()),
			DataContainer::I16(ref mut e) => e.begin(*self.frame_pos.get(i as usize).unwrap()),
			DataContainer::I32(ref mut e) => e.begin(*self.frame_pos.get(i as usize).unwrap()),
			_ => {}
		}
		self.current_sprite = *self.frame_pos.get((i + 1) as usize).unwrap();
		*self.sprite_info.get(self.current_sprite as usize).unwrap() as i32
	}

	fn next_sprite(&mut self) {
		self.current_sprite += 1;
	}

	fn process(&mut self, p0: &AnmTransform, p1: &mut AnmTransform) -> i16 {
		let fp_type = self.frame_data.read();
		read_and_process(fp_type, &mut self.frame_data, &self.table, p0, p1);
		*self.sprite_info.get(self.current_sprite as usize).unwrap()
	}
}