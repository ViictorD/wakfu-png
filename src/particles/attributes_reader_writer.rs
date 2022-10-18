use bytebuffer::ByteBuffer;

use crate::utils::math_helper::MathHelper;

pub struct AttributesReaderWriter;

impl AttributesReaderWriter {
	pub fn read_unsigned_short(buffer: &mut ByteBuffer, level: &f32) -> i32 {
		let a = buffer.read_i16().unwrap() as f32;
		let b = buffer.read_i16().unwrap() as f32;
		MathHelper::lerp_i32(a, b, level)
	}

	pub fn read_float(buffer: &mut ByteBuffer, level: &f32) -> f32 {
		let a = buffer.read_f32().unwrap();
		let b = buffer.read_f32().unwrap();
		MathHelper::lerp_f32(a, b, level)
	}

	pub fn read_int(buffer: &mut ByteBuffer, level: &f32) -> i32 {
		let a = buffer.read_i32().unwrap() as f32;
		let b = buffer.read_i32().unwrap() as f32;
		MathHelper::lerp_i32(a, b, level)
	}
}