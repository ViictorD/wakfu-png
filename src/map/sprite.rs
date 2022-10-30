use glam::{IVec2, Vec2};

use super::color::Color;
use crate::map::iso_to_screen;

#[derive(Debug)]
pub struct AnmSprite {
	pub gfx_id: i32,
	pub direction: i8,
	pub activation_pattern: i16,
	pub state: i16
}

impl AnmSprite {
	pub fn new(gfx_id: i32, direction: i8, activation_pattern: i16, state: i16) -> Self {
		AnmSprite {
			gfx_id,
			direction,
			activation_pattern,
			state
		}
	}
}

#[derive(Debug)]
pub struct DynamicSprite {
	pub gfx_id: i32,
	pub direction: i8,
	pub base_anm: String
}

impl DynamicSprite {
	pub fn new(gfx_id: i32, direction: i8, base_anm: String) -> Self {
		DynamicSprite {
			gfx_id,
			direction,
			base_anm
		}
	}
}

#[derive(Debug)]
pub struct ParticleSprite {
	pub system_id: i32,
	pub level: i8,
	pub offset_x: i8,
	pub offset_y: i8,
	pub offset_z: i8
}

impl ParticleSprite {
	pub fn new(system_id: i32, level: i8, offset_x: i8, offset_y: i8, offset_z: i8) -> Self {
		ParticleSprite {
			system_id,
			level,
			offset_x,
			offset_y,
			offset_z
		}
	}
}

pub enum LayerOrder {
	Ground,
	InteractiveElement,
	DynamicElement,
	Particle
}

impl LayerOrder {
	pub fn get_index(&self) -> u8 {
		*self as u8
	}
}

#[derive(Debug)]
pub struct MapSprite {
	pub cell_x: i32,
	pub cell_y: i32,
	pub cell_z: i16,
	pub height: i8,
	pub altitude_order: u8,
	pub tag: u8,
	pub element_id: i32,
	pub group_key: i32,
	pub group_id: i32,
	pub layer: u8,
	pub color: Color,
	pub anm_sprite: Option<AnmSprite>,
	pub dyn_sprite: Option<DynamicSprite>,
	pub particle_sprite: Option<ParticleSprite>
}

impl MapSprite {
	#[inline]
	pub fn screen_position(&self) -> Vec2 {
		let height = self.cell_z as i32 - self.height as i32;
		iso_to_screen(IVec2::new(self.cell_x, self.cell_y), height)
	}

	#[inline]
	pub fn hashcode(&self, delta_z: u8) -> i64 {
		(self.cell_y as i64 + 8192 & 0x3FFF) << 34
			| (self.cell_x as i64 + 8192 & 0x3FFF) << 19
			| (self.altitude_order as i64 & 0x1FFF) << 6
			| delta_z as i64
	}
}
