pub struct MathHelper;

impl MathHelper {
	pub fn lerp_i32(a: f32, b: f32, level: &f32) -> i32 {
		(a + *level * (b - a)).round() as i32
	}

	pub fn lerp_f32(a: f32, b: f32, level: &f32) -> f32 {
		a + *level * (b - a)
	}

	pub fn nearest_greatest_pow_of_two(value: i32) -> i32 {
		if value < 2 {
			return value;
		}
		let mut v = value - 1;
		v = v | v >> 1;
		v = v | v >> 2;
		v = v | v >> 4;
		v = v | v >> 8;
		v = v | v >> 16;
		v + 1
	}

	pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
		if value <= min {
			return min;
		}
		if value >= max {
			return max;
		}
		value
	}

	pub fn log2i(value: usize) -> usize {
		let mut count = 1;
		let mut c = 0;
		while value > count {
			count *= 2;
			c += 1;
		}
		c
	}

	pub fn get_int_from_two_int(a: i32, b: i32) -> i32 {
		(a << 16) | (b & 0xFFFF)
	}

	pub fn get_u16_from_two_u8(a: u8, b: u8) -> u16 {
		(a as u16) << 8 | (b as u16)
	}

	pub fn fast_floor(value: f32) -> i32 {
		let v = value as i32;
		if value >= 0. || v as f32 == value {
			return v;
		}
		v - 1
	}
}