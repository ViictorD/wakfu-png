use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct Deformer {
	pub growth_x: f32,
	pub growth_y: f32,
	pub angle: f32,
	pub growth_random_x: f32,
	pub growth_random_y: f32,
	pub angle_random: f32
}

impl Deformer {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let growth_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let growth_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let angle = AttributesReaderWriter::read_float(buffer, level_percent);
		let growth_random_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let growth_random_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let angle_random = AttributesReaderWriter::read_float(buffer, level_percent);

		Deformer {
			growth_x,
			growth_y,
			angle,
			growth_random_x,
			growth_random_y,
			angle_random
		}
	}

	pub fn affect(&self, time_increment: f32, particle: &mut Particle) {
		let t = (33333. * time_increment) as i32 as f32 / 1000.;
		particle.scale_x += (self.growth_x + rand::random::<f32>() * self.growth_random_x) * t;
		particle.scale_y += (self.growth_y + rand::random::<f32>() * self.growth_random_y) * t;
		particle.angle += (self.angle + rand::random::<f32>() * self.angle_random) * t;
	}
}