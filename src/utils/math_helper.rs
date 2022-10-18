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
}