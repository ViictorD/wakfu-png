#[derive(Clone)]
pub struct CellLightDef {
	merged: [f32; 3]
}

impl Default for CellLightDef {
	fn default() -> Self {
		CellLightDef {
			merged: [0.0, 0.0, 0.0]
		}
	}
}

impl CellLightDef {
	pub fn new(ambiance: i32, _shadow: i32, _light: i32, _allow_out_door_lighting: bool) -> Self {
		let light_red = get_red_color_from_argb(ambiance) * 2.;
		let light_green = get_green_color_from_argb(ambiance) * 2.;
		let light_blue = get_blue_color_from_argb(ambiance) * 2.;
		let merged = [light_red, light_green, light_blue];
		CellLightDef {
			merged
		}
	}

	pub fn get_color(&self) -> &[f32; 3] {
		&self.merged
	}
}

fn get_red_color_from_argb(argb: i32) -> f32 {
	(argb & 0xFF) as f32 / 255.
}

fn get_green_color_from_argb(argb: i32) -> f32 {
	((argb >> 8) & 0xFF) as f32 / 255.
}

fn get_blue_color_from_argb(argb: i32) -> f32 {
	(argb >> 16 & 0xFF) as f32 / 255.
}