use anyhow::{Result, anyhow};
use bytebuffer::ByteBuffer;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
enum AttractorAxis {
	X,
	Y,
	Z,
	All
}

impl AttractorAxis {
	pub fn get_from_index(index: i8) ->  Result<Self> {
		match index {
			0 => Ok(Self::X),
			1 => Ok(Self::Y),
			2 => Ok(Self::Z),
			3 => Ok(Self::All),
			_ => Err(anyhow!("Attractor index not found"))
		}
	}
}

#[derive(Clone)]
pub struct AttractionForce {
	intensity: f32,
	axis: AttractorAxis,
	offset_x: f32,
	offset_y: f32,
	offset_z: f32
}

impl AttractionForce {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let intensity = AttributesReaderWriter::read_float(buffer, level_percent);
		let offset_x = AttributesReaderWriter::read_float(buffer, level_percent);
		let offset_y = AttributesReaderWriter::read_float(buffer, level_percent);
		let offset_z = AttributesReaderWriter::read_float(buffer, level_percent);
		let axis_value = buffer.read_i8().unwrap();
		let axis = AttractorAxis::get_from_index(axis_value).unwrap();

		AttractionForce {
			intensity,
			axis,
			offset_x,
			offset_y,
			offset_z
		}
	}

	pub fn affect(&self, time_increment: f32, parent: &*const Particle, particle: &mut Particle) {
		let delta_intensity = self.intensity * time_increment;
		let mut dx = self.offset_x - particle.x;
		let mut dy = self.offset_y - particle.y;
		let mut dz = self.offset_z - particle.z;

		let p = Particle::get_parent(parent);
		if !p.geocentric {
			dx += p.get_x();
			dy += p.get_y();
			dz += p.get_z();
		}

		let l = (dx * dx + dy * dy + dz * dz).sqrt();
		dx /= l;
		dy /= l;
		dz /= l;
		match self.axis {
			AttractorAxis::X => {
				particle.y += dy * delta_intensity;
				particle.z += dz * delta_intensity;
			},
			AttractorAxis::Y => {
				particle.x += dx * delta_intensity;
				particle.z += dz * delta_intensity;
			},
			AttractorAxis::Z => {
				particle.x += dx * delta_intensity;
				particle.y += dy * delta_intensity;
			},
			AttractorAxis::All => {
				particle.x += dx * delta_intensity;
				particle.y += dy * delta_intensity;
				particle.z += dz * delta_intensity;
			}
		}
	}
}