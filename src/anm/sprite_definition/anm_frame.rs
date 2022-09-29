use anyhow::Result;
use bytebuffer::ByteBuffer;

use crate::anm::anm_action::anm_action::{AnmAction};
use super::shape_factory::{anm_shap_factory::AnmShapeFactory, anm_shape::AnmShapeTrait};

#[derive(Clone)]
pub struct AnmFrame {
	pub sprites: Vec<Box<dyn AnmShapeTrait>>,
	pub actions: Vec<AnmAction>
}

impl AnmFrame {
	pub fn new() -> Self {
		AnmFrame {
			sprites: Vec::new(),
			actions: Vec::new(),
		}
	}

	pub fn load(&mut self, buffer: &mut ByteBuffer, index: i32) -> i32 {
		let num_sprites = buffer.read_i16().unwrap();
		self.sprites = Vec::new();
		for _ in 0..num_sprites {
			self.sprites.push(AnmShapeFactory::create_shape(buffer).unwrap());
		}
		return self.load_actions(buffer, index);
	}

	fn load_actions(&mut self, buffer: &mut ByteBuffer, index: i32) -> i32 {
		let num_actions = buffer.read_i8().unwrap();

		if num_actions == 0 {
			self.actions = Vec::new();
		}
		else {
			self.actions = Vec::with_capacity(num_actions as usize);
			for _ in 0..num_actions {
				let action_id = buffer.read_i8().unwrap();
				let parameters_count = buffer.read_i8().unwrap();
				let mut anm_action = AnmAction::get(action_id as u8, parameters_count, buffer).unwrap();

				let steps = || -> Result<()> {
					anm_action.set_frame_index(index);
					self.actions.push(anm_action);
					Ok(())
				};
				steps().unwrap();
			}
		}
		return buffer.read_i16().unwrap() as i32;
	}

	pub fn copy_without_action(&self) -> Self {
		AnmFrame {
			sprites: self.sprites.clone(),
			actions: Vec::new()
		}
	}

	pub fn get_sprite(&self) -> &Vec<Box<dyn AnmShapeTrait>> {
		&self.sprites
	}
}