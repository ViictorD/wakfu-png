use anyhow::{Result, anyhow};

use super::particle::Particle;

#[derive(Clone)]
pub struct ParticleList {
	size: usize,
	particles: Vec<Particle>,
}

// It is very important that Particles inside Vec do NOT move place position
// to keep memory address the same
impl ParticleList {
	pub fn new(capacity: i32) -> Self {
		ParticleList {
			size: 0,
			particles: Vec::with_capacity(capacity as usize),
		}
	}

	// Push a particle to a vec and return his index
	pub fn add(&mut self, particle: Particle) -> Result<usize> {
		if self.is_full() {
			return Err(anyhow!("Could not add particle, particle list is full"));
		}
		let mut index = 0;
		for particle in &self.particles {
			if !particle.used {
				break ;
			}
			index += 1;
		}
		if index == self.particles.len() {
			self.particles.push(particle);
		}
		else {
			self.particles[index] = particle;
		}
		self.size += 1;
		Ok(index)
	}

	// Set a fresh Particle at the place of index, destroying initial particle
	pub fn remove(&mut self, index: usize) -> Result<()> {
		if index > self.particles.capacity() - 1 {
			return Err(anyhow!("Could not remove particle, invalid index"));
		}
		self.particles[index] = Particle::new(false);
		self.size -= 1;
		Ok(())
	}

	pub fn get_dead_particle_index(&self) -> Option<usize> {
		self.particles.iter().position(|particle| particle.used && !particle.is_alive())
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut Particle> {
		self.particles.get_mut(index)
	}
	
	pub fn get_particles(&self) -> Vec<&Particle> {
		self.particles.iter().filter(|particle| particle.used).collect()
	}

	pub fn get_particles_mut(&mut self) -> Vec<&mut Particle> {
		self.particles.iter_mut().filter(|particle| particle.used).collect()
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn is_full(&self) -> bool {
		self.size >= self.particles.capacity()
	}
}