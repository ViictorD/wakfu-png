use anyhow::{anyhow, Result};
use bytebuffer::ByteBuffer;

use super::{
	emitter_definition::{EmitterDefinitionType, EmitterDefinition},
	attributes_reader_writer::AttributesReaderWriter,
	affector::affector::{Affector, AffectorType}, condition::ConditionType, particle_model::ParticleModel
};

pub struct ParticleSystemLoader {
	pub geocentric: bool,
	pub _behind_mobile: bool,
	pub _must_apply_night_color: bool,
	pub src_blend: i32,
	pub dst_blend: i32,
	pub texture_id: i64,
	pub _duration: i32,
	pub _render_radius: i8,
	pub emitters: Vec<EmitterDefinition>
}

impl ParticleSystemLoader {
	pub fn load(mut buffer: ByteBuffer, mut level: i8) -> Result<Self> {
		if level < 1 {
			level = 1;
		}
		else if level > 100 {
			level = 100;
		}

		let geocentric = buffer.read_i8().unwrap() != 0;
		let _behind_mobile = buffer.read_i8().unwrap() != 0;
		let _must_apply_night_color = buffer.read_i8().unwrap() != 0;
		let src_blend = buffer.read_i32().unwrap();
		let dst_blend = buffer.read_i32().unwrap();
		if src_blend != 1 || dst_blend != 771 {
			panic!("Blending mode not implemented");
		}
		let texture_id = buffer.read_i64().unwrap();
		let _duration = AttributesReaderWriter::read_unsigned_short(&mut buffer, &(level as f32));
		
		let _render_radius = buffer.read_i8().unwrap();

		let emitter_count = buffer.read_i8()?;
		let mut emitters = Vec::with_capacity(emitter_count as usize);
		for _ in 0..emitter_count {
			let emitter_def = Self::load_emitter(&mut buffer, &level);
			if emitter_def.is_ok() {
				emitters.push(emitter_def.unwrap());
			}
		}

		let result = ParticleSystemLoader {
			geocentric,
			_behind_mobile,
			_must_apply_night_color,
			src_blend,
			dst_blend,
			texture_id,
			_duration,
			_render_radius,
			emitters
		};

		Ok(result)
	}

	fn load_emitter(buffer: &mut ByteBuffer, level: &i8) -> Result<EmitterDefinition> {
		let min_level = buffer.read_i8()?;
		let max_level = buffer.read_i8()?;
		let data_offset = buffer.read_i32()?;

		if *level < min_level || *level > max_level {
			buffer.set_rpos(buffer.get_rpos() + data_offset as usize);
			return Err(anyhow!(""));
		}

		let level_percent = (level - min_level) as f32 / (max_level - min_level) as f32;
		
		let mut tmp_emitter = EmitterDefinitionType::load(buffer, &level_percent)?;
		let mut emitter_def = tmp_emitter.get_emitter_definition()?;

		let model_count = buffer.read_i8()?;
		for _ in 0..model_count {
			let model = ParticleModel::load(buffer, &level_percent)?;
			emitter_def.add_particle_model(model);
		}
		tmp_emitter = EmitterDefinitionType::EmitterDefinition(emitter_def);
		Self::load_affector(buffer, &level_percent, &mut tmp_emitter);
		emitter_def = tmp_emitter.get_emitter_definition()?;

		let light_count = buffer.read_i8()?;
		for _ in 0..light_count {
			let mut light_def = EmitterDefinitionType::load(buffer, &level_percent)?;
			Self::load_affector(buffer, &level_percent, &mut light_def);
			if let EmitterDefinitionType::LightDefinition(light_definition) = &light_def {
				emitter_def.set_light_definition(light_definition.clone());
			}
		}

		let sub_emitter_count = buffer.read_i8()?;
		for _ in 0..sub_emitter_count {
			let sub_emitter_def = Self::load_emitter(buffer, level)?;
			emitter_def.add_emitter_definition(sub_emitter_def);
		}
		Ok(emitter_def)
	}
	
	fn load_affector(buffer: &mut ByteBuffer, level_percent: &f32, affectorable: &mut EmitterDefinitionType) {
		let affector_count = buffer.read_i8().unwrap();
		for _ in 0..affector_count {
			let mut affector = Affector::new();
			let affector_type = AffectorType::load(buffer, level_percent).unwrap();
			affector.add_affector(affector_type);
			let condition_count = buffer.read_i8().unwrap();
			if condition_count == 0 {
				affector.add_condition(None);
			}
			else {
				let mut conditions = Vec::with_capacity(condition_count as usize);
				for _ in 0..condition_count {
					conditions.push(ConditionType::load(buffer, level_percent).unwrap());
				}
				affector.add_condition(Some(conditions));
			}
			if affector.is_key_framed_affector() {
				affectorable.add_key_framed_affector(affector);
			}
			else {
				affectorable.add_affector(affector);
			}
		}
	}
}
