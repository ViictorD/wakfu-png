#[derive(Clone)]
pub struct AnmTransform {
	pub rotation_skew_x0: f32,
	pub rotation_skew_y0: f32,
	pub rotation_skew_x1: f32,
	pub rotation_skew_y1: f32,
	pub translation_x: f32,
	pub translation_y: f32,
	pub rotation_is_identity: bool,
	pub translation_is_identity: bool,
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
	pub custom_color_index: i8
}

impl AnmTransform {
	pub fn new() -> Self {
		AnmTransform {
			rotation_skew_x0: 1.,
			rotation_skew_y0: 0.,
			rotation_skew_x1: 0.,
			rotation_skew_y1: 1.,
			translation_x: 0.,
			translation_y: 0.,
			custom_color_index: 0,
			rotation_is_identity: true,
			translation_is_identity: true,
			red: 1.,
			green: 1.,
			blue: 1.,
			alpha: 1.
		}
	}

	pub fn set_rotation_to_id(&mut self) {
		self.rotation_skew_x0 = 1.;
		self.rotation_skew_y0 = 0.;
		self.rotation_skew_x1 = 0.;
		self.rotation_skew_y1 = 1.;
	}

	pub fn set_rotation_skew(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
		self.rotation_skew_x0 = x0;
		self.rotation_skew_y0 = y0;
		self.rotation_skew_x1 = x1;
		self.rotation_skew_y1 = y1;
	}
}