use anyhow::{Result, anyhow};
use image::{Primitive, Rgba};
use num_traits::{NumCast};

pub enum BlendModes {
	Zero,
	One,
	SrcColor,
	InvSrcColor,
	SrcAlpha,
	InvSrcAlpha,
	DestColor,
	InvDestColor,
	DestAlpha,
	InvDestAlpha,
	SrcAlphaSaturate
}

impl BlendModes {
	pub fn from_index(index: i32) -> Result<Self> {
		match index {
			0 => Ok(BlendModes::Zero),
			1 => Ok(BlendModes::One),
			768 => Ok(BlendModes::SrcColor),
			769 => Ok(BlendModes::InvSrcColor),
			770 => Ok(BlendModes::SrcAlpha),
			771 => Ok(BlendModes::InvSrcAlpha),
			774 => Ok(BlendModes::DestColor),
			775 => Ok(BlendModes::InvDestColor),
			772 => Ok(BlendModes::DestAlpha),
			773 => Ok(BlendModes::InvDestAlpha),
			776 => Ok(BlendModes::SrcAlphaSaturate),
			_ => Err(anyhow!("Blend mode not found: {index}"))
		}
	}
}
pub trait ExtBlend {
	/// Blends a color in-place.
	fn blend(&mut self, other: &Self, blend_src: &BlendModes, blend_dest: &BlendModes);
}

impl<T: Primitive> ExtBlend for Rgba<T> {
	fn blend(&mut self, other: &Rgba<T>, blend_src: &BlendModes, blend_dest: &BlendModes) {
		// http://stackoverflow.com/questions/7438263/alpha-compositing-algorithm-blend-modes#answer-11163848

		if other.0[3].is_zero() {
			return;
		}
		if other.0[3] == T::DEFAULT_MAX_VALUE {
			*self = *other;
			return;
		}

		// First, as we don't know what type our pixel is, we have to convert to floats between 0.0 and 1.0
		let max_t = T::DEFAULT_MAX_VALUE;
		let max_t = max_t.to_f32().unwrap();
		let (bg_r, bg_g, bg_b, bg_a) = (self.0[0], self.0[1], self.0[2], self.0[3]);
		let (fg_r, fg_g, fg_b, fg_a) = (other.0[0], other.0[1], other.0[2], other.0[3]);
		let (bg_r, bg_g, bg_b, bg_a) = (
			bg_r.to_f32().unwrap() / max_t,
			bg_g.to_f32().unwrap() / max_t,
			bg_b.to_f32().unwrap() / max_t,
			bg_a.to_f32().unwrap() / max_t,
		);
		let (fg_r, fg_g, fg_b, fg_a) = (
			fg_r.to_f32().unwrap() / max_t,
			fg_g.to_f32().unwrap() / max_t,
			fg_b.to_f32().unwrap() / max_t,
			fg_a.to_f32().unwrap() / max_t,
		);

		// Work out what the final alpha level will be

		let alpha_final = bg_a + fg_a - bg_a * fg_a;
		if alpha_final == 0.0 {
			return;
		};

		let (out_r_a, out_g_a, out_b_a) = blend_from_blend_modes(
			bg_r, bg_g, bg_b, bg_a,
			fg_r, fg_g, fg_b, fg_a,
			blend_src, blend_dest
		);

		// Unmultiply the channels by our resultant alpha channel
		let (mut out_r, mut out_g, mut out_b) = (
			out_r_a / alpha_final,
			out_g_a / alpha_final,
			out_b_a / alpha_final,
		);

		// Cast back to our initial type on return
		if out_r.gt(&1.) {
			out_r = 1.;
		}
		if out_g.gt(&1.) {
			out_g = 1.;
		}
		if out_b.gt(&1.) {
			out_b = 1.;
		}
		*self = Rgba([
			NumCast::from(max_t * out_r).unwrap(),
			NumCast::from(max_t * out_g).unwrap(),
			NumCast::from(max_t * out_b).unwrap(),
			NumCast::from(max_t * alpha_final).unwrap(),
		])
	}
}

fn blend_from_blend_modes(
	bg_r: f32, bg_g: f32, bg_b: f32, bg_a: f32,
	fg_r: f32, fg_g: f32, fg_b: f32, fg_a: f32,
	blend_src: &BlendModes, blend_dest: &BlendModes
) -> (f32, f32, f32) {
	// Not sure if this is correct if we implement more modes
	let (fg_r_a, fg_g_a, fg_b_a, bg_r_a, bg_g_a, bg_b_a) = match blend_src {
		BlendModes::SrcAlpha => {
			(fg_r * fg_a, fg_g * fg_a, fg_b * fg_a, bg_r * bg_a, bg_g * bg_a, bg_b * bg_a)
		},
		BlendModes::One => {
			(fg_r, fg_g, fg_b, bg_r * bg_a, bg_g * bg_a, bg_b * bg_a)
		},
		_ => panic!("Not implemented")
	};

	match blend_dest {
		BlendModes::InvSrcAlpha => {
			(
				fg_r_a + bg_r_a * (1.0 - fg_a),
				fg_g_a + bg_g_a * (1.0 - fg_a),
				fg_b_a + bg_b_a * (1.0 - fg_a),
			)
		},
		_ => panic!("Not implemented")
	}
}