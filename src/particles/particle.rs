use super::{particle_model::ParticleModel, emitter_definition::EmitterDefinition, emitter::Emitter};

#[derive(Clone)]
pub struct Particle {
	pub used: bool,
	pub spawned_address: *const Particle,
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub last_x: f32,
	pub last_y: f32,
	pub last_z: f32,
	pub velocity_x: f32,
	pub velocity_y: f32,
	pub velocity_z: f32,
	pub angle_x: f32,
	pub angle_y: f32,
	pub angle_z: f32,
	pub base_angle_x: f32,
	pub base_angle_y: f32,
	pub base_angle_z: f32,
	pub key_framed_life: f32,
	pub life: f32,
	pub life_time: f32,
	pub angle: f32,
	pub scale_x: f32,
	pub scale_y: f32,
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
	pub half_width: f32,
	pub half_height: f32,
	pub hot_x: f32,
	pub hot_y: f32,
	pub texture_top: f32,
	pub texture_left: f32,
	pub texture_bottom: f32,
	pub texture_right: f32,
	pub model: Option<ParticleModel>,
	pub parent: Option<*const Particle>,
	pub _source_emitter: Option<Emitter>,
	pub geocentric: bool,
	pub emitters: Vec<Emitter>
}

impl Particle {
	pub fn new(used: bool) -> Self {
		Particle {
			used,
			spawned_address: std::ptr::null(),
			x: 0.,
			y: 0.,
			z: 0.,
			last_x: 0.,
			last_y: 0.,
			last_z: 0.,
			velocity_x: 0.,
			velocity_y: 0.,
			velocity_z: 0.,
			angle_x: 0.,
			angle_y: 0.,
			angle_z: 0.,
			base_angle_x: 0.,
			base_angle_y: 0.,
			base_angle_z: 0.,
			key_framed_life: 0.,
			life: 0.,
			life_time: f32::MAX,
			angle: 0.,
			scale_x: 0.,
			scale_y: 0.,
			red: 0.,
			green: 0.,
			blue: 0.,
			alpha: 0.,
			half_width: 0.,
			half_height: 0.,
			hot_x: 0.,
			hot_y: 0.,
			texture_top: 0.,
			texture_left: 0.,
			texture_bottom: 0.,
			texture_right: 0.,
			model: None,
			parent: None,
			_source_emitter: None,
			geocentric: false,
			emitters: Vec::new()
		}
	}

	pub fn add_emitters(&mut self, emitters_def: &Vec<EmitterDefinition>) {
		for emitter in emitters_def {
			let mut emitter = emitter.create_emitter();
			emitter.parent = Some(self as *const Particle);
			self.emitters.push(emitter);
		}
	}

	pub fn get_parent(parent: &*const Particle) -> &Particle {
		unsafe {
			&(*(*parent))
		}
	}

	pub fn get_x(&self) -> f32 {
		if self.geocentric || self.parent.is_none() || Self::get_parent(&self.parent.unwrap()).geocentric {
			return self.x;
		}
		0.
	}

	pub fn get_y(&self) -> f32 {
		if self.geocentric || self.parent.is_none() || Self::get_parent(&self.parent.unwrap()).geocentric {
			return self.y;
		}
		0.
	}

	pub fn get_z(&self) -> f32 {
		if self.geocentric || self.parent.is_none() || Self::get_parent(&self.parent.unwrap()).geocentric {
			return self.z;
		}
		0.
	}

	pub fn is_alive(&self) -> bool {
		if self.life <= self.life_time && self.life_time != f32::MAX {
			return true;
		}
		false
	}

	pub fn is_emitter(&self) -> bool {
		self.emitters.len() > 0
	}

	pub fn update(&mut self, time_increment: f32) {
		self.life += time_increment;
		self.x += self.velocity_x * time_increment;
		self.y += self.velocity_y * time_increment;
		self.z += self.velocity_z * time_increment;
		if let Some(model) = &mut self.model {
			if let ParticleModel::ParticleBitmapSequenceModel(sequence) = model {
				let texture_coords = sequence.get_texture_coodrinates(1000. * time_increment);
				self.texture_top = texture_coords[0];
				self.texture_left = texture_coords[1];
				self.texture_bottom = texture_coords[2];
				self.texture_right = texture_coords[3];
			}
		}
		for emitters in &mut self.emitters {
			emitters.update(time_increment);
		}
	}
}
