#[derive(Clone)]
pub struct Light {
	saturation: [f32; 3],
	range: f32
}

impl Light {
	pub fn new() -> Self {
		Light {
			saturation: [0.; 3],
			range: 0.,
		}
	}

	pub fn set_color(&mut self, red: f32, green: f32, blue: f32) {
		self.saturation = [red, green, blue];
	}

	pub fn set_range(&mut self, range: f32) {
		self.range = range;
	}
}