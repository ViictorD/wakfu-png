use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct RotationInterpolation {
	pub angle_x: f32,
	pub angle_y: f32,
	pub angle_z: f32
}

impl RotationInterpolation {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let angle_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let angle_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let angle_z = AttributesReaderWriter::read_float(buffer, level_percent);

		RotationInterpolation {
			angle_x,
			angle_y,
			angle_z
		}
	}

	pub fn affect(&self, time_progress_ratio: f32, particle: &mut Particle) {
		particle.angle_x = particle.base_angle_x + self.angle_x * time_progress_ratio;
		particle.angle_y = particle.base_angle_y + self.angle_x * time_progress_ratio;
		particle.angle_z = particle.base_angle_z + self.angle_x * time_progress_ratio;
	}
}