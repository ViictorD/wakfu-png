use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct RotorForce {
	intensity: f32
}

impl RotorForce {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let intensity = AttributesReaderWriter::read_float(buffer, level_percent);

		RotorForce {
			intensity
		}
	}

	pub fn affect(&self, time_increment: f32, parent: &*const Particle, particle: &mut Particle) {
		let delta_intensity = self.intensity * time_increment;
		
		let p = Particle::get_parent(parent);
		if let Some(p_parent) = &p.parent {
			let p_p = Particle::get_parent(p_parent);
			particle.x += (particle.y - p_p.get_y()) * delta_intensity;
			particle.y -= (particle.x - p_p.get_x()) * delta_intensity;
		}
		else {
			particle.x += (particle.y - p.get_y()) * delta_intensity;
			particle.y -= (particle.x - p.get_x()) * delta_intensity;
		}
	}
}