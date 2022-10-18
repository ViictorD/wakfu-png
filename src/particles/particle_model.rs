use anyhow::{anyhow, Result};
use bytebuffer::ByteBuffer;

use super::{attributes_reader_writer::AttributesReaderWriter, anim_data_use::AnimDataUse, particle::Particle};

#[derive(Clone)]
pub enum ParticleModel {
	ParticleBitmapModel(ParticleBitmapModel),
	ParticleBitmapSequenceModel(ParticleBitmapSequenceModel)
}

impl ParticleModel {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Result<Self> {
		let model_type = buffer.read_i8().unwrap();
		match model_type {
			1 => Ok(Self::ParticleBitmapModel(ParticleBitmapModel::load(buffer, level_percent))),
			2 => Ok(Self::ParticleBitmapSequenceModel(ParticleBitmapSequenceModel::load(buffer, level_percent))),
			_ => Err(anyhow!("Unknown model type: {model_type}"))
		}
	}

	pub fn generate_particle(&self) -> Particle {
		let mut particle = Particle::new(true);
		particle.model = Some(self.clone());
		particle
	}

	pub fn initialize_particle(&self, particle: &mut Particle) {
		let particle_bitmap_model = match self {
			ParticleModel::ParticleBitmapModel(model) => model,
			ParticleModel::ParticleBitmapSequenceModel(sequence) => &sequence.particle_bitmap_model
		};
		let mut scale_x = particle_bitmap_model.scale_x;
		let mut scale_y = particle_bitmap_model.scale_y;
		let mut rotation = particle_bitmap_model.rotation;
		if particle_bitmap_model.scale_random_keep_ratio {
			let random_scale = rand::random::<f32>() * particle_bitmap_model.scale_random_x;
			scale_x += random_scale;
			scale_y += random_scale
		}
		else {
			if particle_bitmap_model.scale_random_x.ne(&0.) {
				scale_x += rand::random::<f32>() * particle_bitmap_model.scale_random_x;
			}
			if particle_bitmap_model.scale_random_y.ne(&0.) {
				scale_y += rand::random::<f32>() * particle_bitmap_model.scale_random_y;
			}
		}
		if particle_bitmap_model.rotation_random.ne(&0.) {
			rotation += (rand::random::<f32>() - 0.5) * particle_bitmap_model.rotation_random;
		}
		
		particle.hot_x = particle_bitmap_model.hot_x;
		particle.hot_y = particle_bitmap_model.hot_y;
		particle.alpha = particle_bitmap_model.alpha_color + rand::random::<f32>() * particle_bitmap_model.alpha_color_random;
		particle.red = particle_bitmap_model.red_color + rand::random::<f32>() * particle_bitmap_model.red_color_random;
		particle.green = particle_bitmap_model.green_color + rand::random::<f32>() * particle_bitmap_model.green_color_random;
		particle.blue = particle_bitmap_model.blue_color + rand::random::<f32>() * particle_bitmap_model.blue_color_random;
		particle.scale_x = scale_x;
		particle.scale_y = scale_y;
		particle.angle = rotation * 0.017453292;
		particle.half_width = particle_bitmap_model.half_width;
		particle.half_height = particle_bitmap_model.half_height;
		particle.texture_top = particle_bitmap_model.texture_top;
		particle.texture_left = particle_bitmap_model.texture_left;
		particle.texture_bottom = particle_bitmap_model.texture_bottom;
		particle.texture_right = particle_bitmap_model.texture_right;
		particle.angle_x = particle_bitmap_model.rotation_x * 0.017453292;
		particle.base_angle_x = particle.angle_x;
		particle.angle_y = particle_bitmap_model.rotation_y * 0.017453292;
		particle.base_angle_y = particle.angle_y;
		particle.angle_z = particle_bitmap_model.rotation_z * 0.017453292;
		particle.base_angle_z = particle.angle_z;
	}
}

#[derive(Clone)]
pub struct ParticleBitmapModel {
	pub bitmap_id: i32,
	pub hot_x: f32,
	pub hot_y: f32,
	pub scale_x: f32,
	pub scale_y: f32,
	pub scale_random_x: f32,
	pub scale_random_y: f32,
	pub scale_random_keep_ratio: bool,
	pub rotation: f32,
	pub rotation_random: f32,
	pub red_color: f32,
	pub green_color: f32,
	pub blue_color: f32,
	pub alpha_color: f32,
	pub red_color_random: f32,
	pub green_color_random: f32,
	pub blue_color_random: f32,
	pub alpha_color_random: f32,
	pub texture_top: f32,
	pub texture_left: f32,
	pub texture_bottom: f32,
	pub texture_right: f32,
	pub half_width: f32,
	pub half_height: f32,
	pub rotation_x: f32,
	pub rotation_y: f32,
	pub rotation_z: f32
}

impl ParticleBitmapModel {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let scale_random_keep_ratio = buffer.read_i8().unwrap() != 0;
		let bitmap_id = buffer.read_i32().unwrap();
		let hot_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let hot_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let scale_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let scale_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let scale_random_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let scale_random_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let red_color = AttributesReaderWriter::read_float(buffer, level_percent);
		let green_color = AttributesReaderWriter::read_float(buffer, level_percent);
		let blue_color = AttributesReaderWriter::read_float(buffer, level_percent);
		let alpha_color = AttributesReaderWriter::read_float(buffer, level_percent);
		let red_color_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let green_color_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let blue_color_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let alpha_color_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let texture_top = AttributesReaderWriter::read_float(buffer, level_percent);
		let texture_left = AttributesReaderWriter::read_float(buffer, level_percent);
		let texture_bottom = AttributesReaderWriter::read_float(buffer, level_percent);
		let texture_right = AttributesReaderWriter::read_float(buffer, level_percent);
		let half_width = AttributesReaderWriter::read_float(buffer, level_percent);
		let half_height = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation_z = AttributesReaderWriter::read_float(buffer, level_percent);

		ParticleBitmapModel {
			bitmap_id,
			hot_x,
			hot_y,
			scale_x,
			scale_y,
			scale_random_x,
			scale_random_y,
			scale_random_keep_ratio,
			rotation,
			rotation_random,
			red_color,
			green_color,
			blue_color,
			alpha_color,
			red_color_random,
			green_color_random,
			blue_color_random,
			alpha_color_random,
			texture_top,
			texture_left,
			texture_bottom,
			texture_right,
			half_width,
			half_height,
			rotation_x,
			rotation_y,
			rotation_z
		}
	}
}

#[derive(Clone)]
pub struct ParticleBitmapSequenceModel {
	particle_bitmap_model: ParticleBitmapModel,
	anim_data: Option<AnimDataUse>,
	speed: f32,
	loop_count: i32,
	current_time: f32,
}

impl ParticleBitmapSequenceModel {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let scale_random_keep_ratio = buffer.read_i8().unwrap() != 0;
		let bitmap_id = buffer.read_i32().unwrap();
		let hot_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let hot_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let scale_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let scale_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let scale_random_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let scale_random_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let red_color = AttributesReaderWriter::read_float(buffer, level_percent);
		let green_color = AttributesReaderWriter::read_float(buffer, level_percent);
		let blue_color = AttributesReaderWriter::read_float(buffer, level_percent);
		let alpha_color = AttributesReaderWriter::read_float(buffer, level_percent);
		let red_color_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let green_color_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let blue_color_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let alpha_color_random = AttributesReaderWriter::read_float(buffer, level_percent);
		let half_width = AttributesReaderWriter::read_float(buffer, level_percent);
		let half_height = AttributesReaderWriter::read_float(buffer, level_percent);
		let anim_data = AnimDataUse::load(buffer);
		let speed = AttributesReaderWriter::read_float(buffer, level_percent);
		let loop_count = AttributesReaderWriter::read_int(buffer, level_percent);
		let rotation_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let rotation_z = AttributesReaderWriter::read_float(buffer, level_percent);
		let current_time = 0.;

		let particle_bitmap_model = ParticleBitmapModel {
			bitmap_id,
			hot_x,
			hot_y,
			scale_x,
			scale_y,
			scale_random_x,
			scale_random_y,
			scale_random_keep_ratio,
			rotation,
			rotation_random,
			red_color,
			green_color,
			blue_color,
			alpha_color,
			red_color_random,
			green_color_random,
			blue_color_random,
			alpha_color_random,
			texture_top: 0.,
			texture_left: 0.,
			texture_bottom: 0.,
			texture_right: 0.,
			half_width,
			half_height,
			rotation_x,
			rotation_y,
			rotation_z
		};

		ParticleBitmapSequenceModel {
			particle_bitmap_model,
			anim_data,
			speed,
			loop_count,
			current_time,
		}
	}

	pub fn get_texture_coodrinates(&mut self, elapsed_time: f32) -> [f32; 4] {
		self.current_time += self.speed * elapsed_time;
		let anim_data = self.anim_data.as_ref().unwrap();
		let total_time = anim_data.total_time as f32;
		if self.current_time >= total_time {
			self.current_time -= total_time;
			if self.loop_count > 0 {
				self.loop_count -= 1;
			}
		}

		if self.loop_count == 0 {
			return anim_data.get_texture_coodrinates(total_time as u16);
		}
		return anim_data.get_texture_coodrinates(self.current_time as u16);
	}
}
