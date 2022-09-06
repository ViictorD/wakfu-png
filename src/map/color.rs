pub trait SrgbColorSpace {
	fn linear_to_nonlinear_srgb(self) -> Self;
}

impl SrgbColorSpace for f32 {
	#[inline]
	fn linear_to_nonlinear_srgb(self) -> f32 {
		if self <= 0.0 {
			return self;
		}

		if self <= 0.0031308 {
			self * 12.92 // linear falloff in dark values
		} else {
			(1.055 * self.powf(1.0 / 2.4)) - 0.055 // gamma curve in other area
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
	/// Red channel. [0.0, 1.0]
	red: f32,
	/// Green channel. [0.0, 1.0]
	green: f32,
	/// Blue channel. [0.0, 1.0]
	blue: f32,
	/// Alpha channel. [0.0, 1.0]
	alpha: f32,
}

impl Color {
	pub const fn rgb_linear(r: f32, g: f32, b: f32) -> Color {
		Color {
			red: r,
			green: g,
			blue: b,
			alpha: 1.0,
		}
	}

	pub const fn rgba_linear(r: f32, g: f32, b: f32, a: f32) -> Color {
		Color {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		}
	}

	/// Get red in sRGB colorspace.
	pub fn r(&self) -> f32 {
		self.red.linear_to_nonlinear_srgb()
	}

	/// Get green in sRGB colorspace.
	pub fn g(&self) -> f32 {
		self.green.linear_to_nonlinear_srgb()
	}

	/// Get blue in sRGB colorspace.
	pub fn b(&self) -> f32 {
		self.blue.linear_to_nonlinear_srgb()
	}

	/// Get alpha.
	#[inline(always)]
	pub fn a(&self) -> f32 {
		self.alpha
	}
	
}

