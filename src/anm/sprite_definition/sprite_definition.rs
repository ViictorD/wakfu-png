use anyhow::{Result, anyhow};
use bytebuffer::ByteBuffer;

use crate::anm::{
	anm::AnmTransformDataTable, processing::anm_transform::AnmTransform,
};

use super::{
	anm_sprite_definition::AnmSpriteDefinition,
	anm_sprite_definition_single::AnmSpriteDefinitionSingle,
	anm_sprite_definition_single_no_action::AnmSpriteDefinitionSingleNoAction,
	anm_sprite_definition_single_frame::AnmSpriteDefinitionSingleFrame,
	anm_sprite_definition_indexed::AnmSpriteDefinitionIndexed, sprite_def::SpriteDef
};

pub trait SpriteDefinitionTrait: SpriteDefinitionClone {
	fn new(table: &AnmTransformDataTable, optimized: bool) -> Result<Self> where Self: Sized;
	fn load(&mut self, buffer: &mut ByteBuffer);
	fn get_frame_count(&self) -> i32;
	fn get_sprite_def(&self) -> &SpriteDef;
	fn begin_process_frame(&mut self, index: i32) -> i32;
	fn next_sprite(&mut self);
	fn process(&mut self, p0: &AnmTransform, p1: &mut AnmTransform) -> i16;
}

pub trait SpriteDefinitionClone {
	fn clone_box(&self) -> Box<dyn SpriteDefinitionTrait>;
}

impl<T> SpriteDefinitionClone for T
where
	T: 'static + SpriteDefinitionTrait + Clone,
{	
	fn clone_box(&self) -> Box<dyn SpriteDefinitionTrait> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn SpriteDefinitionTrait> {
	fn clone(&self) -> Self {
		self.clone_box()
	}
}

#[derive(Clone)]
pub struct EmptySpriteDef {
	sprite_def: SpriteDef
}

impl SpriteDefinitionTrait for EmptySpriteDef {
	fn new(_table: &AnmTransformDataTable, _optimized: bool) -> Result<Self> {
		let result = EmptySpriteDef {
			sprite_def: SpriteDef::new()
		};
		Ok(result)
	}

	fn load(&mut self, _buffer: &mut ByteBuffer) {}
	
	fn get_frame_count(&self) -> i32 {
		0
	}
	
	fn get_sprite_def(&self) -> &SpriteDef {
		&self.sprite_def
	}

	fn begin_process_frame(&mut self, _index: i32) -> i32 {
		0
	}

	fn next_sprite(&mut self) {}

	fn process(&mut self, _p0: &AnmTransform, _p1: &mut AnmTransform) -> i16 {
		0
	}
}

#[derive(Clone)]
pub struct SpriteDefinition {
	pub sprite_def: Box<dyn SpriteDefinitionTrait>
}

impl Default for SpriteDefinition {
	fn default() -> Self {
		SpriteDefinition {
			sprite_def: Box::new(EmptySpriteDef { sprite_def: SpriteDef::new() })
		}
	}
}

impl SpriteDefinition {
	pub fn create_from(table: &Option<AnmTransformDataTable>, buffer: &mut ByteBuffer, optimized: bool) -> Result<Self> {
		if table.is_none() {
			let result = SpriteDefinition {
				sprite_def: Box::new(AnmSpriteDefinition::new(&AnmTransformDataTable::default(), optimized)?)
			};
			return Ok(result);
		}
		let m_type = buffer.read_i8()?;
		
		let def: Box<dyn SpriteDefinitionTrait> = match m_type {
			1 => Box::new(AnmSpriteDefinitionSingle::new(&table.as_ref().unwrap(), optimized)?),
			2 => Box::new(AnmSpriteDefinitionSingleNoAction::new(&table.as_ref().unwrap(), optimized)?),
			3 => Box::new(AnmSpriteDefinitionSingleFrame::new(&table.as_ref().unwrap(), optimized)?),
			4 => Box::new(AnmSpriteDefinitionIndexed::new(&table.as_ref().unwrap(), optimized)?),
			_ => return Err(anyhow!("Index not found"))
		};

		let result = SpriteDefinition {
			sprite_def: def
		};
		Ok(result)
	}
}

