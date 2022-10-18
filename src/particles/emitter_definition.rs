use bytebuffer::ByteBuffer;
use anyhow::{Result, anyhow};

use super::{affector::affector::Affector, affectorable::Affectorable, attributes_reader_writer::AttributesReaderWriter, particle_model::ParticleModel, emitter::Emitter, particle::Particle, light::Light};

pub enum EmitterDefinitionType {
	EmitterDefinition(EmitterDefinition),
	LightDefinition(LightDefinition),
}

impl EmitterDefinitionType {
	pub fn load(buffer: &mut ByteBuffer, &level_percent: &f32) -> Result<Self> {
		let emmiter_type = buffer.read_i8()?;
		match emmiter_type {
			1 => Ok(EmitterDefinitionType::EmitterDefinition(EmitterDefinition::load(buffer, level_percent))),
			2 => Ok(EmitterDefinitionType::LightDefinition(LightDefinition::load(buffer, level_percent))),
			_ => return Err(anyhow!("Unknown definition type: {emmiter_type}"))
		}
	}

	pub fn get_emitter_definition(self) -> Result<EmitterDefinition> {
		match self {
			EmitterDefinitionType::EmitterDefinition(emitter_definition) => Ok(emitter_definition),
			_ => Err(anyhow!("Emitter is not a emitter definition"))
		}
	}

	pub fn add_affector(&mut self, affector: Affector) {
		match self {
			EmitterDefinitionType::EmitterDefinition(emitter_definition) => emitter_definition.affector.add_affector(affector),
			EmitterDefinitionType::LightDefinition(light_definition) => light_definition.affector.add_affector(affector),
		}
	}

	pub fn add_key_framed_affector(&mut self, affector: Affector) {
		match self {
			EmitterDefinitionType::EmitterDefinition(emitter_definition) => emitter_definition.affector.add_key_framed_affector(affector),
			EmitterDefinitionType::LightDefinition(light_definition) => light_definition.affector.add_key_framed_affector(affector),
		}
	}
}


#[derive(Clone)]
pub struct EmitterDefinition {
	pub affector: Affectorable,
	pub emitter_definitions: Vec<EmitterDefinition>, // Sub emitters
	pub models: Vec<ParticleModel>,
	pub light_definition: Option<LightDefinition>,
	pub start_spawn_time: f32,
	pub end_spawn_time: f32,
	pub max_particles_count: i32,
	pub max_particles_per_spawn: i32,
	pub spawn_frequency: f32,
	pub spawn_frequency_random: f32,
	pub particle_life_time: f32,
	pub particle_life_time_random: f32,
	pub particle_offset_x: f32,
	pub particle_offset_y: f32,
	pub particle_offset_z: f32,
	pub particle_offset_random_x: f32,
	pub particle_offset_random_y: f32,
	pub particle_offset_random_z: f32,
	pub particle_velocity_x: f32,
	pub particle_velocity_y: f32,
	pub particle_velocity_z: f32,
	pub particle_velocity_random_x: f32,
	pub particle_velocity_random_y: f32,
	pub particle_velocity_random_z: f32,
	pub geocentric: bool
}

impl EmitterDefinition {
	fn load(buffer: &mut ByteBuffer, level_percent: f32) -> Self {
		let affector = Affectorable::new();
		let models = Vec::new();
		let light_definition = None;
		let emitter_definitions = Vec::new();
		let geocentric = buffer.read_i8().unwrap() != 0;
		let max_particles_count = AttributesReaderWriter::read_unsigned_short(buffer, &level_percent);
		let max_particles_per_spawn = AttributesReaderWriter::read_unsigned_short(buffer, &level_percent);
		let spawn_frequency = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_life_time = AttributesReaderWriter::read_float(buffer, &level_percent);
		let spawn_frequency_random = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_life_time_random = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_offset_x = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_offset_y = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_offset_z = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_offset_random_x = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_offset_random_y = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_offset_random_z = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_velocity_x = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_velocity_y = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_velocity_z = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_velocity_random_x = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_velocity_random_y = AttributesReaderWriter::read_float(buffer, &level_percent);
		let particle_velocity_random_z = AttributesReaderWriter::read_float(buffer, &level_percent);
		let start_spawn_time = AttributesReaderWriter::read_float(buffer, &level_percent);
		let end_spawn_time = AttributesReaderWriter::read_float(buffer, &level_percent);

		EmitterDefinition {
			affector,
			emitter_definitions,
			models,
			light_definition,
			start_spawn_time,
			end_spawn_time,
			max_particles_count,
			max_particles_per_spawn,
			spawn_frequency,
			spawn_frequency_random,
			particle_life_time,
			particle_life_time_random,
			particle_offset_x,
			particle_offset_y,
			particle_offset_z,
			particle_offset_random_x,
			particle_offset_random_y,
			particle_offset_random_z,
			particle_velocity_x,
			particle_velocity_y,
			particle_velocity_z,
			particle_velocity_random_x,
			particle_velocity_random_y,
			particle_velocity_random_z,
			geocentric
		}
	}

	pub fn add_particle_model(&mut self, model: ParticleModel) {
		self.models.push(model);
	}

	pub fn set_light_definition(&mut self, light_definition: LightDefinition) {
		self.light_definition = Some(light_definition);
	}

	pub fn add_emitter_definition(&mut self, emitter_def: EmitterDefinition) {
		self.emitter_definitions.push(emitter_def);
	}

	pub fn create_emitter(&self) -> Emitter {
		let emitter = Emitter::from_definition(self);
		emitter
	}

	pub fn has_affector(&self) -> bool {
		self.affector.affectors.len() > 0
	}

	pub fn has_key_framed_affector(&self) -> bool {
		self.affector.has_key_framed_affector()
	}

	pub fn get_affectors(&mut self) -> &mut Vec<Affector> {
		&mut self.affector.affectors
	}

	pub fn get_key_framed_affectors(&mut self) -> &mut Vec<Affector> {
		&mut self.affector.key_framed_affectors
	}

	pub fn get_random_particle_model_index(&self) -> Option<usize> {
		if self.models.len() > 0 {
			return Some(rand::random::<u8>() as usize % self.models.len());
		}
		None
	}

	pub fn initialize_particle(&mut self, emitter: &Option<*const Particle>, particle: &mut Particle) {
		let mut offset_x = self.particle_offset_x;
		let mut offset_y = self.particle_offset_y;
		let mut offset_z = self.particle_offset_z;
		let mut velocity_x = self.particle_velocity_x;
		let mut velocity_y = self.particle_velocity_y;
		let mut velocity_z = self.particle_velocity_z;
		if self.particle_offset_random_x != 0. {
			offset_x += (rand::random::<f32>() - 0.5) * self.particle_offset_random_x;
		}
		if self.particle_offset_random_y != 0. {
			offset_y += (rand::random::<f32>() - 0.5) * self.particle_offset_random_y;
		}
		if self.particle_offset_random_z != 0. {
			offset_z += (rand::random::<f32>() - 0.5) * self.particle_offset_random_z;
		}
		if self.particle_velocity_random_x != 0. {
			velocity_x += (rand::random::<f32>() - 0.5) * self.particle_velocity_random_x;
		}
		if self.particle_velocity_random_y != 0. {
			velocity_y += (rand::random::<f32>() - 0.5) * self.particle_velocity_random_y;
		}
		if self.particle_velocity_random_z != 0. {
			velocity_z += (rand::random::<f32>() - 0.5) * self.particle_velocity_random_z;
		}
		particle.x = offset_x;
		particle.y = offset_y;
		particle.z = offset_z;
		particle.velocity_x = velocity_x;
		particle.velocity_y = velocity_y;
		particle.velocity_z = velocity_z;
		particle.life_time = self.particle_life_time + rand::random::<f32>() * self.particle_life_time_random;
		particle.life = 0.;
		particle.geocentric = self.geocentric;
		particle.parent = emitter.clone();

		if let Some(emitter2) = emitter {
			let p = Particle::get_parent(emitter2);
			if !p.geocentric {
				particle.x += p.get_x();
				particle.y += p.get_y();
				particle.z += p.get_z();
			}
		}
		if self.emitter_definitions.len() > 0 {
			particle.add_emitters(&self.emitter_definitions);
		}
	}
}

#[derive(Clone)]
pub struct LightDefinition {
	affector: Affectorable,
	red: f32,
	green: f32,
	blue: f32,
	intensity: f32,
	range: f32
}

impl LightDefinition {
	fn load(buffer: &mut ByteBuffer, level_percent: f32) -> Self {
		let affector = Affectorable::new();
		let red = AttributesReaderWriter::read_float(buffer, &level_percent);
		let green = AttributesReaderWriter::read_float(buffer, &level_percent);
		let blue = AttributesReaderWriter::read_float(buffer, &level_percent);
		let intensity = AttributesReaderWriter::read_float(buffer, &level_percent);
		let range = AttributesReaderWriter::read_float(buffer, &level_percent);

		LightDefinition {
			affector,
			red,
			green,
			blue,
			intensity,
			range
		}
	}

	pub fn create_light(&self) -> Light {
		let mut light = Light::new();
		light.set_color(self.red * self.intensity, self.green * self.intensity, self.blue * self.intensity);
		light.set_range(self.range);
		light
	}

	pub fn has_affector(&self) -> bool {
		self.affector.affectors.len() > 0
	}

	pub fn has_key_framed_affector(&self) -> bool {
		self.affector.has_key_framed_affector()
	}

	pub fn create_light_particle(&self) -> Particle {
		let mut particle = Particle::new(true);
		particle.red = self.red;
		particle.green = self.green;
		particle.blue = self.blue;
		particle.alpha = self.intensity;
		particle.half_width = self.range;
		particle.half_height = self.range;
		particle.scale_x = 1.;
		particle
	}
}
