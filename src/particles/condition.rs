use anyhow::{anyhow, Result};
use bytebuffer::ByteBuffer;

use super::{attributes_reader_writer::AttributesReaderWriter, particle::Particle};

pub enum ConditionType {
	LifeCondition(LifeCondition),
	PositionCondition(PositionCondition)
}

impl ConditionType {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Result<Self> {
		let condition_type = buffer.read_i8()?;
		match condition_type {
			1 => Ok(ConditionType::LifeCondition(LifeCondition::load(buffer, level_percent))),
			2 => Ok(ConditionType::PositionCondition(PositionCondition::load(buffer, level_percent))),
			_ => return Err(anyhow!("Unknown condition type: {condition_type}"))
		}
	}
}

#[derive(Clone)]
pub struct LifeCondition {
	pub start: f32,
	pub end: f32
}

impl LifeCondition {
	fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let start = AttributesReaderWriter::read_float(buffer, level_percent);
		let end = AttributesReaderWriter::read_float(buffer, level_percent);
		LifeCondition {
			start,
			end
		}
	}
}

#[derive(Clone)]
enum PositionConditionMode {
	XGreater,
	XLess,
	YGreater,
	YLess,
	ZGreater,
	ZLess
}

impl PositionConditionMode {
	pub fn from_index(index: i8) -> Result<Self> {
		match index {
			0 => Ok(Self::XGreater),
			1 => Ok(Self::XLess),
			2 => Ok(Self::YGreater),
			3 => Ok(Self::YLess),
			4 => Ok(Self::ZGreater),
			5 => Ok(Self::ZLess),
			_ => Err(anyhow!("Unknown condition index"))
		}
	}
}

#[derive(Clone)]
pub struct PositionCondition {
	value: i32,
	condition: PositionConditionMode,
	use_system_as_reference: bool
}

impl PositionCondition {
	fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let use_system_as_reference = buffer.read_i8().unwrap() != 0;
		let mode_value = buffer.read_i8().unwrap();
		let condition = PositionConditionMode::from_index(mode_value).unwrap();
		let value = AttributesReaderWriter::read_unsigned_short(buffer, level_percent);

		PositionCondition {
			value,
			condition,
			use_system_as_reference
		}
	}

	pub fn validate(&self, parent: &*const Particle, particle: &mut Particle) -> bool {
		let p = Particle::get_parent(parent);
		match self.condition {
			PositionConditionMode::XGreater => { return particle.x >= self.value as f32 + (if self.use_system_as_reference { panic!("Not implemented") } else { p.x }); },
			PositionConditionMode::XLess => { return particle.x <= self.value as f32 + (if self.use_system_as_reference { panic!("Not implemented") } else { p.x }); },
			PositionConditionMode::YGreater => { return particle.y >= self.value as f32 + (if self.use_system_as_reference { panic!("Not implemented") } else { p.x }); },
			PositionConditionMode::YLess => { return particle.y <= self.value as f32 + (if self.use_system_as_reference { panic!("Not implemented") } else { p.x }); },
			PositionConditionMode::ZGreater => { return particle.z >= self.value as f32 + (if self.use_system_as_reference { panic!("Not implemented") } else { p.x }); },
			PositionConditionMode::ZLess => { return particle.z <= self.value as f32 + (if self.use_system_as_reference { panic!("Not implemented") } else { p.x }); },
		}
	}
}