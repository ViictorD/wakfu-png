use bytebuffer::ByteBuffer;
use anyhow::Result;
use crate::anm::{anm::AnmTransformDataTable, processing::anm_transform::AnmTransform};
use super::{
	sprite_definition::SpriteDefinitionTrait,
	sprite_def::{SpriteDef},
	shape_factory::anm_shap_factory::AnmShapeFactory,
	anm_frame::AnmFrame
};

#[derive(Clone)]
pub struct AnmSpriteDefinition {
	sprite_def: SpriteDef,
	current_sprite: i32,
	optimized: bool,
	frames: Vec<AnmFrame>,
	current_frame_index: i32
}

impl SpriteDefinitionTrait for AnmSpriteDefinition {
	fn new(_table: &AnmTransformDataTable, optimized: bool) -> Result<Self> {
		let sprite_def = SpriteDef::new();
		let current_sprite = -1;
		let frames = Vec::new();
		let current_frame_index = 0;

		let result = AnmSpriteDefinition {
			sprite_def,
			current_sprite,
			optimized,
			frames,
			current_frame_index
		};
		Ok(result)
	}

	fn load(&mut self, buffer: &mut ByteBuffer) {
		self.sprite_def.load(buffer);

		if !self.optimized {
			let num_shapes = buffer.read_i8().unwrap();
			if num_shapes != 0 {
				for _ in 0..num_shapes {
					AnmShapeFactory::create_shape(buffer).unwrap();
				}
			}
			let num_sprites = buffer.read_i8().unwrap();
			if num_sprites != 0 {
				for _ in 0..num_sprites {
					AnmShapeFactory::create_shape(buffer).unwrap();
				}
			}
		}
		let num_frames: i16 = buffer.read_i16().unwrap();
		self.frames = Vec::with_capacity(num_frames as usize);
		for i in 0..num_frames {
			let mut anm_frame = AnmFrame::new();
			let count = anm_frame.load(buffer, i as i32);
			self.frames.push(anm_frame);

			if count > 0 {
				for _ in 0..count {
					let frame_without_action = self.frames.get(i as usize).unwrap().copy_without_action();
					self.frames.insert(i as usize, frame_without_action);
				}
			}
			let sprite_count = self.frames.get(i as usize).unwrap().get_sprite().len();
			if sprite_count > self.sprite_def.max_sprite_count as usize {
				self.sprite_def.max_sprite_count = sprite_count as i32;
			}
		}
		if self.frames.len() == 1 {
			let actions = &self.frames.get(0).unwrap().actions;
			for i in 0..actions.len() {
				match actions.get(i).unwrap().action.get_type() {
					"GO_TO_ANIMATION" => {},
					"GO_TO_IF_PREVIOUS_ANIMATION" => {},
					"GO_TO_RANDOM_ANIMATION" => {},
					"GO_TO_STATIC_ANIMATION" => {
						self.sprite_def.is_animation_node = true;
					},
					_ => {}
				}
			}
		}
	}

	fn get_frame_count(&self) -> i32 {
		self.frames.len() as i32
	}
	
	fn get_sprite_def(&self) -> &SpriteDef {
		&self.sprite_def
	}

	fn begin_process_frame(&mut self, index: i32) -> i32 {
		self.current_sprite = -1;
		self.current_frame_index = index;
		self.frames.get(index as usize).unwrap().sprites.len() as i32
	}

	fn next_sprite(&mut self) {
		self.current_sprite += 1;
	}

	fn process(&mut self, p0: &AnmTransform, p1: &mut AnmTransform) -> i16 {
		let sprite =
			self.frames.get(self.current_frame_index as usize)
			.unwrap()
			.sprites
			.get(self.current_sprite as usize)
			.unwrap();
		sprite.process(p0, p1);
		sprite.get_id()
	}

}
