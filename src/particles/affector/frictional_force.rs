use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct FrictionalForce {
	friction: f32
}

impl FrictionalForce {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let friction = AttributesReaderWriter::read_float(buffer, level_percent);

		FrictionalForce {
			friction
		}
	}

	pub fn affect(&self, time_increment: f32, particle: &mut Particle) {
		let d = 1. - self.friction * time_increment;
		particle.velocity_x *= d;
		particle.velocity_y *= d;
		particle.velocity_z *= d;
	}
}