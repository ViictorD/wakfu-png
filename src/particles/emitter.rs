use super::{
	particle::Particle,
	emitter_definition::{EmitterDefinition, LightDefinition},
	particle_list::ParticleList, light::Light
};

#[derive(Clone)]
pub struct Emitter {
	elapsed_time: f32,
	key_frame_time: f32,
	time_for_spawn: f32,
	pub children: Option<ParticleList>,
	pub parent: Option<*const Particle>,
	pub definition: EmitterDefinition,
	light_definition: Option<LightDefinition>,
	pub light_particle: Option<Box<Particle>>,
	light: Option<Light>,
	random_frequency: f32
}

impl Emitter {
	pub fn from_definition(emitter_definition: &EmitterDefinition) -> Self {
		let mut emitter = Emitter {
			elapsed_time: 0.,
			key_frame_time: 0.,
			time_for_spawn: 0.,
			children: None,
			parent: None,
			definition: emitter_definition.clone(),
			light_definition: None,
			light_particle: None,
			light: None,
			random_frequency: 0.
		};
		emitter.set_light_definition(&emitter_definition.light_definition);
		emitter
	}

	fn set_light_definition(&mut self, light_def: &Option<LightDefinition>) {
		if let Some(light_definition) = light_def {
			self.light_definition = Some(light_definition.clone());
			self.light = Some(light_definition.create_light());
			let mut light_particle = light_definition.create_light_particle();
			self.definition.initialize_particle(&self.parent, &mut light_particle);
			light_particle.life_time -= -0.1;
			light_particle.parent = self.parent.clone();
			self.light_particle = Some(Box::new(light_particle));
		}
	}

	pub fn get_parent(&self) -> Option<&Particle> {
		if let Some(particle) = &self.parent {
			unsafe {
				return Some(&(*(*particle)));
			}
		}
		None
	}

	pub fn can_spawn_particles(&self) -> bool {
		let mut parent_bool = true;
		if let Some(particle) = self.get_parent() {
			if !particle.life.gt(&0.) {
				parent_bool = false;
			}
		}
		parent_bool
			&& self.elapsed_time >= self.definition.start_spawn_time
			&& (self.definition.end_spawn_time == 0. || self.elapsed_time <= self.definition.end_spawn_time)
	}

	fn spawn_particles(&mut self, time_increment: f32) {
		if self.definition.spawn_frequency.eq(&0.) {
			return ;
		}
		self.time_for_spawn += time_increment;
		let frequency = self.definition.spawn_frequency + self.random_frequency;
		if self.children.is_none() {
			self.children = Some(ParticleList::new(self.definition.max_particles_count));
		}
		else {
			if self.time_for_spawn < frequency || self.children.as_ref().unwrap().is_full() {
				return ;
			}
			self.time_for_spawn -= frequency;
		}
		self.random_frequency = self.definition.spawn_frequency_random * rand::random::<f32>();
		for _ in 0..self.definition.max_particles_per_spawn {
			if let Some(model_index) = self.definition.get_random_particle_model_index() {
				if let Some(model) = self.definition.models.get(model_index) {
					let new_particle = model.generate_particle();
					// We push and get back particle to get the final memory address of this particle
					let particle_index = self.children.as_mut().unwrap().add(new_particle).unwrap();
					let particle = self.children.as_mut().unwrap().get_mut(particle_index).unwrap();
					particle.spawned_address = particle as *const Particle;
					model.initialize_particle(particle);
					self.definition.initialize_particle(&self.parent, particle);
					if particle.is_emitter() {
						for emitter in &mut particle.emitters {
							if emitter.can_spawn_particles() {
								emitter.spawn_particles(time_increment);
							}
						}
					}
					if self.children.as_ref().unwrap().is_full() {
						return ;
					}
				}
			}
		}
	}

	pub fn update(&mut self, time_increment: f32) {
		self.elapsed_time += time_increment;
		// Kill
		if let Some(children) = &mut self.children {
			while let Some(index) = children.get_dead_particle_index() {
				children.remove(index).unwrap();
			}
		}
		// Spawn
		if self.can_spawn_particles() {
			self.spawn_particles(time_increment);
		}

		if let Some(light_definition) = &self.light_definition {
			if light_definition.has_affector() {
				panic!("Not implemented");
			}
			if light_definition.has_key_framed_affector() {
				panic!("Not implemented");
			}
		}
		if self.children.is_none() || self.children.as_ref().unwrap().size() == 0 {
			return ;
		}
		// Update
		let num_children = self.children.as_ref().unwrap().size();
		if self.definition.has_key_framed_affector() {
			if num_children > 0 {
				self.key_frame_time += time_increment;
			}
			if let Some(parent) = &self.parent {
				while self.key_frame_time.ge(&0.03) {
					for child in self.children.as_mut().unwrap().get_particles_mut() {
						let life = child.life;
						child.life = child.key_framed_life;
						if child.key_framed_life.le(&child.life_time) {
							for affector in self.definition.get_key_framed_affectors() {
								affector.update(0.03, parent, child);
							}
						}
						child.key_framed_life += 0.03;
						child.life = life;
					}
					self.key_frame_time -= 0.03;
				}
			}
		}
		if self.definition.has_affector() {
			if let Some(parent) = &self.parent {
				for affector in self.definition.get_affectors() {
					for i in (0..self.children.as_ref().unwrap().size()).rev() {
						let particle = self.children.as_mut().unwrap().get_mut(i).unwrap();
						if affector.update(time_increment, parent, particle) {
							break ;
						}
					}
				}
			}
		}
		for particle in self.children.as_mut().unwrap().get_particles_mut() {
			assert_eq!(particle.spawned_address, particle as *const Particle, "Particle children moved in memory");
			particle.update(time_increment);
		}

	}
}