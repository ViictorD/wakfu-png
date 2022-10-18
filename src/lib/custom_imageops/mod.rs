use image::RgbaImage;
use self::color::BlendModes;

use super::custom_imageops::color::ExtBlend;

pub mod color;

pub fn custom_overlay(bottom: &mut RgbaImage, top: &RgbaImage, x: i64, y: i64, blend_src: &BlendModes, blend_dest: &BlendModes) {
	let bottom_dims = bottom.dimensions();
	let top_dims = top.dimensions();

	// Crop our top image if we're going out of bounds
	let (
		origin_bottom_x,
		origin_bottom_y,
		origin_top_x,
		origin_top_y,
		range_width,
		range_height
	) = overlay_bounds_ext(bottom_dims, top_dims, x, y);

	for y in 0..range_height {
		for x in 0..range_width {
			let p = top.get_pixel(origin_top_x + x, origin_top_y + y);
			let mut bottom_pixel = bottom.get_pixel(origin_bottom_x + x, origin_bottom_y + y).clone();

			bottom_pixel.blend(&p, blend_src, blend_dest);

			bottom.put_pixel(origin_bottom_x + x, origin_bottom_y + y, bottom_pixel);
		}
	}
}

fn overlay_bounds_ext(
	(bottom_width, bottom_height): (u32, u32),
	(top_width, top_height): (u32, u32),
	x: i64,
	y: i64,
) -> (u32, u32, u32, u32, u32, u32) {
	// Return a predictable value if the two images don't overlap at all.
	if x > i64::from(bottom_width)
		|| y > i64::from(bottom_height)
		|| x.saturating_add(i64::from(top_width)) <= 0
		|| y.saturating_add(i64::from(top_height)) <= 0
	{
		return (0, 0, 0, 0, 0, 0);
	}

	// Find the maximum x and y coordinates in terms of the bottom image.
	let max_x = x.saturating_add(i64::from(top_width));
	let max_y = y.saturating_add(i64::from(top_height));

	// Clip the origin and maximum coordinates to the bounds of the bottom image.
	// Casting to a u32 is safe because both 0 and `bottom_{width,height}` fit
	// into 32-bits.
	let max_inbounds_x = max_x.clamp(0, i64::from(bottom_width)) as u32;
	let max_inbounds_y = max_y.clamp(0, i64::from(bottom_height)) as u32;
	let origin_bottom_x = x.clamp(0, i64::from(bottom_width)) as u32;
	let origin_bottom_y = y.clamp(0, i64::from(bottom_height)) as u32;

	// The range is the difference between the maximum inbounds coordinates and
	// the clipped origin. Unchecked subtraction is safe here because both are
	// always positive and `max_inbounds_{x,y}` >= `origin_{x,y}` due to
	// `top_{width,height}` being >= 0.
	let x_range = max_inbounds_x - origin_bottom_x;
	let y_range = max_inbounds_y - origin_bottom_y;

	// If x (or y) is negative, then the origin of the top image is shifted by -x (or -y).
	let origin_top_x = x.saturating_mul(-1).clamp(0, i64::from(top_width)) as u32;
	let origin_top_y = y.saturating_mul(-1).clamp(0, i64::from(top_height)) as u32;

	(
		origin_bottom_x,
		origin_bottom_y,
		origin_top_x,
		origin_top_y,
		x_range,
		y_range,
	)
}