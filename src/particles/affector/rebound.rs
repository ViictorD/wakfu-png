use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct Rebound {
	min_x: f32,
	min_y: f32,
	min_z: f32,
	max_x: f32,
	max_y: f32,
	max_z: f32,
	restitution_x: f32,
	restitution_y: f32,
	restitution_z: f32,
	restitution_random_x: f32,
	restitution_random_y: f32,
	restitution_random_z: f32,
	has_random: bool
}

impl Rebound {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let min_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let min_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let min_z = AttributesReaderWriter::read_float(buffer, level_percent);
		let max_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let max_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let max_z = AttributesReaderWriter::read_float(buffer, level_percent);
		let restitution_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let restitution_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let restitution_z = AttributesReaderWriter::read_float(buffer, level_percent);
		let restitution_random_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let restitution_random_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let restitution_random_z = AttributesReaderWriter::read_float(buffer, level_percent);
		let has_random = restitution_x.ne(&0.) || restitution_y.ne(&0.) || restitution_z.ne(&0.);

		Rebound {
			min_x,
			min_y,
			min_z,
			max_x,
			max_y,
			max_z,
			restitution_x,
			restitution_y,
			restitution_z,
			restitution_random_x,
			restitution_random_y,
			restitution_random_z,
			has_random
		}
	}

	pub fn affect(&self, parent: &*const Particle, particle: &mut Particle) {
		let p = Particle::get_parent(parent);
		let min_x = p.get_x() + self.min_x;
		let min_y = p.get_y() + self.min_y;
		let min_z = p.get_z() + self.min_z;
		let max_x = p.get_x() + self.max_x;
		let max_y = p.get_y() + self.max_y;
		let max_z = p.get_z() + self.max_z;

		let mut collide = false;
		if particle.last_x.ne(&f32::MAX) {
			if particle.x < min_x && particle.last_x >= min_x {
				collide = true;
				particle.x = min_x;
				particle.velocity_x = -particle.velocity_x;
			}
			if particle.x > min_x && particle.last_x <= min_x {
				collide = true;
				particle.x = max_x;
				particle.velocity_x = -particle.velocity_x;
			}
			if particle.y < min_y && particle.last_y >= min_y {
				collide = true;
				particle.y = min_y;
				particle.velocity_y = -particle.velocity_y;
			}
			if particle.y > min_y && particle.last_y <= min_y {
				collide = true;
				particle.y = max_y;
				particle.velocity_y = -particle.velocity_y;
			}
			if particle.z < min_z && particle.last_z >= min_z {
				collide = true;
				particle.z = min_z;
				particle.velocity_z = -particle.velocity_z;
			}
			if particle.z > min_z && particle.last_z <= min_z {
				collide = true;
				particle.z = max_z;
				particle.velocity_z = -particle.velocity_z;
			}
		}
		if collide {
			if self.has_random {
				particle.velocity_x *= self.restitution_x + rand::random::<f32>() * self.restitution_random_x;
				particle.velocity_y *= self.restitution_y + rand::random::<f32>() * self.restitution_random_y;
				particle.velocity_z *= self.restitution_z + rand::random::<f32>() * self.restitution_random_z;
			}
			else {
				particle.velocity_x *= self.restitution_x;
				particle.velocity_y *= self.restitution_y;
				particle.velocity_z *= self.restitution_z;
			}
		}
		particle.last_x = particle.x;
		particle.last_y = particle.y;
		particle.last_z = particle.z;
		
	}
}