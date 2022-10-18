use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct ColorFader {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
	pub speed: f32
}

impl ColorFader {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let red = AttributesReaderWriter::read_float(buffer, level_percent);
		let green = AttributesReaderWriter::read_float(buffer, level_percent);
		let blue = AttributesReaderWriter::read_float(buffer, level_percent);
		let alpha = AttributesReaderWriter::read_float(buffer, level_percent);
		let speed = AttributesReaderWriter::read_float(buffer, level_percent);
		ColorFader {
			red,
			green,
			blue,
			alpha,
			speed
		}
	}

	pub fn affect(&self, time_increment: f32, particle: &mut Particle) {
		let local_speed = self.speed * time_increment;
		particle.red -= (particle.red - self.red) * local_speed;
		particle.green -= (particle.green - self.green) * local_speed;
		particle.blue -= (particle.blue - self.blue) * local_speed;
		particle.alpha -= (particle.alpha - self.alpha) * local_speed;
	}
}