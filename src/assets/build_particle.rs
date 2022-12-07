use std::collections::HashMap;

use glam::Vec2;
use image::{RgbaImage};

use crate::{
	anm::processing::anm_instance::SpriteCoord, custom_lib::custom_imageops::color::BlendModes
};

use super::build_atlas::{get_result_min_max_coord, build_and_tint_altas};

pub fn build_particle(
	result: &mut HashMap<u16, RgbaImage>,
	sprite_position: Vec2,
	atlas: RgbaImage,
	particles_coords: Vec<SpriteCoord>,
	particle_colors: Vec<[f32; 4]>,
	blend_modes: (BlendModes, BlendModes)
) {

	let (min_x, max_y) = get_result_min_max_coord(&particles_coords);

	let origin = Vec2::new(min_x * -1., max_y.abs());
	let position = sprite_position - origin;
	build_and_tint_altas(
		result,
		position,
		&atlas,
		&None,
		&particles_coords,
		&particle_colors,
		min_x,
		max_y,
		blend_modes,
		false
	);
}