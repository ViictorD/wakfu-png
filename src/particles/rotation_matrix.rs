pub struct RotationMatrix {
	rm: [f32; 9],
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl RotationMatrix {
	pub fn change_angle(rot_a: f32, rot_b: f32, rot_c: f32) -> Self {
		let alpha = rot_a;
		let beta = rot_b;
		let gamma = rot_c;
		let sin_a = alpha.sin();
		let sin_b = beta.sin();
		let sin_g = gamma.sin();
		let cos_a = alpha.cos();
		let cos_b = beta.cos();
		let cos_g = gamma.cos();
		let rm = [
			cos_a * cos_b,
			cos_a * sin_b * sin_g - sin_a * cos_g,
			cos_a * sin_b * cos_g + sin_a * sin_g,
			sin_a * cos_b,
			sin_a * sin_b * sin_g + cos_a * cos_g,
			sin_a * sin_b * cos_g - cos_a * sin_g,
			-sin_b, cos_b * sin_g,
			cos_b * cos_g
		];
		RotationMatrix {
			rm,
			x: 0.,
			y: 0.,
			z: 0.
		}
	}

	pub fn transform(&mut self, x: f32, y: f32, z: f32, c_x: f32, c_y: f32, c_z: f32) {
		let px = x - c_x;
		let py = y - c_y;
		let pz = z - c_z;
		self.x = self.rm[0] * px + self.rm[1] * py + self.rm[2] * pz;
		self.y = self.rm[3] * px + self.rm[4] * py + self.rm[5] * pz;
		self.z = self.rm[6] * px + self.rm[7] * py + self.rm[8] * pz;
		self.x += c_x;
		self.y += c_y;
		self.z += c_z;
	}
}