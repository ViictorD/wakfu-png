use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct BoostForce {
	x: f32,
	y: f32,
	z: f32
}

impl BoostForce {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let x = AttributesReaderWriter::read_float(buffer, level_percent);
		let y = AttributesReaderWriter::read_float(buffer, level_percent);
		let z = AttributesReaderWriter::read_float(buffer, level_percent);
		BoostForce {
			x,
			y,
			z
		}
	}

	pub fn affect(&self, particle: &mut Particle) {
		particle.velocity_x += self.x;
		particle.velocity_y += self.y;
		particle.velocity_z += self.z;
	}
}