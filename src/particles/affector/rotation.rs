use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct Rotation {
	angle_x: f32,
	angle_y: f32,
	angle_z: f32
}

impl Rotation {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let angle_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let angle_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let angle_z = AttributesReaderWriter::read_float(buffer, level_percent);
		Rotation {
			angle_x,
			angle_y,
			angle_z
		}
	}

	pub fn affect(&self, time_increment: f32, particle: &mut Particle) {
		let t = (33333. * time_increment) as i32 as f32 / 1000.;
		particle.angle_x += self.angle_x * t;
		particle.angle_y += self.angle_y * t;
		particle.angle_z += self.angle_z * t;
	}
}