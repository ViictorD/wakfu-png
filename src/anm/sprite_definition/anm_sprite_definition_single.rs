use anyhow::Result;
use bytebuffer::ByteBuffer;

use crate::anm::{anm::AnmTransformDataTable, processing::anm_transform::AnmTransform};
use crate::anm::sprite_definition::shape_factory::anm_frame_processor::read_and_process;

use super::{
	sprite_def::SpriteDef,
	sprite_definition::SpriteDefinitionTrait,
	anm_frame_data::{AnmFrameData, DataContainer}
};

#[derive(Clone)]
pub struct AnmSpriteDefinitionSingle {
	table: AnmTransformDataTable,
	sprite_def: SpriteDef,
	frame_data: DataContainer,
	action_info: Vec<i16>,
	sprite_id: i16
}

impl SpriteDefinitionTrait for AnmSpriteDefinitionSingle {
	fn new(table: &AnmTransformDataTable, _optimized: bool) -> Result<Self> {
		let result = AnmSpriteDefinitionSingle {
			table: table.clone(),
			sprite_def: SpriteDef::new(),
			frame_data: DataContainer::None,
			action_info: Vec::new(),
			sprite_id: 0
		};
		Ok(result)
	}
	fn load(&mut self, buffer: &mut ByteBuffer) {
		self.sprite_def.load(buffer);
		self.sprite_id = buffer.read_i16().unwrap();
		let count = buffer.read_i16().unwrap();
		for _ in 0..count {
			self.action_info.push(buffer.read_i16().unwrap());
		}
		self.frame_data = AnmFrameData::create(buffer).unwrap();
		self.sprite_def.max_sprite_count = 1;
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
		1
	}

	fn next_sprite(&mut self) {}

	fn process(&mut self, p0: &AnmTransform, p1: &mut AnmTransform) -> i16 {
		let fp_type = self.frame_data.read();
		read_and_process(fp_type, &mut self.frame_data, &self.table, p0, p1);
		self.sprite_id
	}
}