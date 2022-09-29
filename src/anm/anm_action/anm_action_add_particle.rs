use bytebuffer::ByteBuffer;
use anyhow::Result;
use super::anm_action::AnmActionTrait;

#[derive(Clone)]
pub struct AnmActionAddParticle {
	_particle_id: i32,
	_offset_x: i16,
	_offset_y: i16,
	_offset_z: i16
}

impl AnmActionTrait for AnmActionAddParticle {
	fn load(parameters_count: i8, buffer: &mut ByteBuffer) -> Result<Self> {
		let _particle_id = buffer.read_i32()?;
		if parameters_count == 3 {
			let _offset_x = buffer.read_i16()?;
			let _offset_y = buffer.read_i16()?;
			
			let result = AnmActionAddParticle {
				_particle_id,
				_offset_x,
				_offset_y,
				_offset_z: 0,
			};
			return Ok(result);
		}
		else if parameters_count == 4 {
			let _offset_x = buffer.read_i16()?;
			let _offset_y = buffer.read_i16()?;
			let _offset_z = buffer.read_i16()?;

			let result = AnmActionAddParticle {
				_particle_id,
				_offset_x,
				_offset_y,
				_offset_z,
			};
			return Ok(result);
		}

		let result = AnmActionAddParticle {
			_particle_id,
			_offset_x: 0,
			_offset_y: 0,
			_offset_z: 0,
		};
		Ok(result)
	}

	fn get_type(&self) -> &'static str {
		"ADD_PARTICLE"
	}
}