use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct CirclePath {
	radial_speed: f32,
	grade: f32,
}

impl CirclePath {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let radial_speed = AttributesReaderWriter::read_float(buffer, level_percent);
		let grade = 180.;
		CirclePath {
			radial_speed,
			grade
		}
	}

	pub fn affect(&mut self, time_increment: f32, parent: &*const Particle, particle: &mut Particle) {
		self.grade += self.radial_speed * time_increment;
		if self.grade.ge(&360.) {
			self.grade -= 360.;
		}
		let p = Particle::get_parent(parent);
		particle.x = p.get_x() + self.grade.cos() * particle.velocity_x + 0.4;
		particle.y = p.get_y() + self.grade.sin() * particle.velocity_y - 0.7;
	
	}
}