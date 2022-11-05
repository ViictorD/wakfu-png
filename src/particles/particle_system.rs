use glam::Vec3;

use crate::{custom_lib::custom_imageops::color::BlendModes, anm::processing::anm_instance::SpriteCoord, map::{ELEVATION_UNIT, CELL_WIDTH, CELL_HEIGHT}};

use super::{
	rotation_matrix::RotationMatrix,
	particle::Particle,
	particle_system_loader::ParticleSystemLoader,
	emitter_definition::EmitterDefinition
};

const MIN_ALPHA: f32 = 1. / 255.;

pub struct ParticleSystem {
	pub src_blend: BlendModes,
	pub dst_blend: BlendModes,
	num_max_particles: i32,
	root: Particle,
	pub definitions: Vec<EmitterDefinition>,
	pub texture_id: i64,
	geocentric: bool
}

impl ParticleSystem {
	pub fn new() -> Self {
		let particle_system = ParticleSystem {
			src_blend: BlendModes::One,
			dst_blend: BlendModes::InvSrcAlpha,
			num_max_particles: 0,
			root: Particle::new(true),
			definitions: Vec::new(),
			texture_id: 0,
			geocentric: false,
		};
		particle_system
	}

	pub fn load(&mut self, particle_system_loader: ParticleSystemLoader) {
		self.geocentric = particle_system_loader.geocentric;
		self.root.geocentric = particle_system_loader.geocentric;
		self.texture_id = particle_system_loader.texture_id;
		self.src_blend = BlendModes::from_index(particle_system_loader.src_blend).unwrap();
		self.dst_blend = BlendModes::from_index(particle_system_loader.dst_blend).unwrap();
		for emitter_def in particle_system_loader.emitters {
			self.add_emitter_definition(emitter_def);
		}
	}

	pub fn get_particles_coords_and_colors(&self) -> (Vec<SpriteCoord>, Vec<[f32; 4]>) {
		let mut result_coords = Vec::new();
		let mut result_colors = Vec::new();
		let delta = Vec3::new(0., 0., 0.);
		self.get_particles_coords_and_colors_recurs(&mut result_coords, &mut result_colors, &self.root, &delta);
		(result_coords, result_colors)
	}

	fn get_particles_coords_and_colors_recurs(&self, result_coords: &mut Vec<SpriteCoord>, result_colors: &mut Vec<[f32; 4]>, particle: &Particle, delta: &Vec3) {
		if particle.parent.is_some() && particle.alpha.gt(&MIN_ALPHA) {
			let iso_x = particle.x + delta.x;
			let iso_y = particle.y + delta.y;
			let iso_z = particle.z + delta.z;
			let pt = Self::get_screen_position(iso_x, iso_y, iso_z);
			let final_points = self.get_final_points(particle, pt.0, pt.1);
			let atlas = [particle.texture_top, particle.texture_left, particle.texture_bottom, particle.texture_right];
			let color = [particle.red, particle.green, particle.blue, particle.alpha];
			result_coords.push(SpriteCoord::new(true, atlas, final_points));
			result_colors.push(color);
		}
		for emitter in &particle.emitters {
			if let Some(children) = &emitter.children {
				let d =
					if !emitter.definition.geocentric { delta.clone() }
					else { Vec3::new(delta.x + particle.x, delta.y + particle.y, delta.z + particle.z) };
				for child in children.get_particles() {
					self.get_particles_coords_and_colors_recurs(result_coords, result_colors, child, &d);
				}
			}
		}
	}

	pub fn get_screen_position(world_x: f32, world_y: f32, world_z: f32) -> (f32, f32) {
		let screen_x = (world_x - world_y) * CELL_WIDTH * 0.5;
		let screen_y = (-(world_x + world_y) * CELL_HEIGHT * 0.5) + world_z * ELEVATION_UNIT;
		(screen_x, screen_y)
	}

	fn add_emitter_definition(&mut self, emitter_def: EmitterDefinition) {
		self.num_max_particles += emitter_def.max_particles_count;
		self.definitions.push(emitter_def);
	}

	pub fn register_all_base_emitters(&mut self) {
		self.root.add_emitters(&self.definitions);
	}

	fn get_final_points(&self, particle: &Particle, x: f32, y: f32) -> [f32; 8] {
		let width = 2. * particle.half_width * particle.scale_x;
		let height = 2. * particle.half_height * particle.scale_y;
		let angle_cos = particle.angle.cos();
		let angle_sin = particle.angle.sin();
		let hot_x = -particle.hot_x * particle.scale_x;
		let hot_y = (particle.hot_y - particle.half_height * 2.) * particle.scale_y;
		let mut top_left_x = x + (angle_cos * hot_x - angle_sin * hot_y);
		let mut top_left_y = y + (angle_sin * hot_x + angle_cos * hot_y);
		let mut bottom_left_x = top_left_x - angle_sin * height;
		let mut bottom_left_y = top_left_y + angle_cos * height;
		let tx_axis_x = angle_cos * width;
		let tx_axis_y = angle_sin * width;
		let mut bottom_right_x = bottom_left_x + tx_axis_x;
		let mut bottom_right_y = bottom_left_y + tx_axis_y;
		let mut top_right_x = top_left_x + tx_axis_x;
		let mut top_right_y = top_left_y + tx_axis_y;

		let mut rotation_matrix = RotationMatrix::change_angle(particle.angle_x, particle.angle_y, particle.angle_z);

		let c_x = particle.x + x;
		let c_y = particle.y + y;

		rotation_matrix.transform(top_left_x, top_left_y, 0., c_x, c_y, 0.);
		top_left_x = rotation_matrix.x;
		top_left_y = rotation_matrix.y;
		rotation_matrix.transform(bottom_left_x, bottom_left_y, 0., c_x, c_y, 0.);
		bottom_left_x = rotation_matrix.x;
		bottom_left_y = rotation_matrix.y;
		rotation_matrix.transform(bottom_right_x, bottom_right_y, 0., c_x, c_y, 0.);
		bottom_right_x = rotation_matrix.x;
		bottom_right_y = rotation_matrix.y;
		rotation_matrix.transform(top_right_x, top_right_y, 0., c_x, c_y, 0.);
		top_right_x = rotation_matrix.x;
		top_right_y = rotation_matrix.y;

		return [
			top_left_x,
			top_left_y,
			bottom_left_x,
			bottom_left_y,
			bottom_right_x,
			bottom_right_y,
			top_right_x,
			top_right_y,
		];
	}

	pub fn update(&mut self, time_increment: f32) {
		self.root.update(time_increment);
	}
	

}