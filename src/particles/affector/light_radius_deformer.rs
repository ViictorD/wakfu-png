use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct LightRadiusDeformer {
	growth_x: f32
}

impl LightRadiusDeformer {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let growth_x = AttributesReaderWriter::read_float(buffer, level_percent);

		LightRadiusDeformer {
			growth_x
		}
	}

	pub fn affect(&self, time_increment: f32, particle: &mut Particle) {
		particle.scale_x += self.growth_x * (33333. * time_increment) as i32 as f32 / 1000.;
	}
}