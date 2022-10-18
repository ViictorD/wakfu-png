use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct LinearForce {
	x: f32,
	y: f32,
	z: f32,
	apply_on_velocity: bool
}

impl LinearForce {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let apply_on_velocity = buffer.read_i8().unwrap() != 0;
		let x = AttributesReaderWriter::read_float(buffer, level_percent);
		let y = AttributesReaderWriter::read_float(buffer, level_percent);
		let z = AttributesReaderWriter::read_float(buffer, level_percent);

		LinearForce {
			x,
			y,
			z,
			apply_on_velocity
		}
	}

	pub fn affect(&self, time_increment: f32, particle: &mut Particle) {
		if self.apply_on_velocity {
			particle.velocity_x += self.x * time_increment;
			particle.velocity_y += self.y * time_increment;
			particle.velocity_z += self.z * time_increment;
		}
		else {
			particle.x += self.x * time_increment;
			particle.y += self.y * time_increment;
			particle.z += self.z * time_increment;
		}
	}
}