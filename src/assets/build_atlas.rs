use anyhow::{anyhow, Result};
use glam::Vec2;
use image::{RgbaImage, imageops, Rgba};
use crate::anm::processing::anm_instance::{AnmInstance, SpriteCoord};
use crate::custom_lib::custom_imageops;
use crate::custom_lib::custom_imageops::color::BlendModes;

pub fn build_atlas(
	result: &mut RgbaImage,
	sprite_position: Vec2,
	atlas: RgbaImage,
	atlas_2: Option<RgbaImage>,
	anm: AnmInstance
) {
	let coords = anm.coords;
	let colors = anm.colors;

	let (min_x, max_y) = get_result_min_max_coord(&coords);

	let origin = Vec2::new(min_x * -1., max_y.abs());

	let position = sprite_position - origin;
	build_and_tint_altas(
		result,
		position,
		&atlas,
		&atlas_2,
		&coords,
		&colors,
		min_x,
		max_y,
		(BlendModes::One, BlendModes::InvSrcAlpha),
		anm.flip_animation
	);
}

fn get_dest_pts(final_coord: &[f32; 8], sprite_orientation: &SpriteOrientation) -> [[f32; 2]; 4] {
	let [
		top_right,
		bottom_right,
		bottom_left,
		top_left
	] = get_sprite_points(final_coord, sprite_orientation);

	let ref_x: f32;
	let ref_y: f32;
	if bottom_left[0] < top_left[0] {
		ref_x = bottom_left[0];
	}
	else {
		ref_x = top_left[0];
	}
	if top_left[1] > top_right[1] {
		ref_y = top_left[1];
	}
	else {
		ref_y = top_right[1];
	}
	
	let result = [
		[top_left[0] - ref_x, ref_y - top_left[1]],
		[top_right[0] - ref_x, ref_y - top_right[1]],
		[bottom_right[0] - ref_x, ref_y - bottom_right[1]],
		[bottom_left[0] - ref_x, ref_y - bottom_left[1]]
	];
	result
}

pub fn get_transformed_size(points: &[[f32; 2]; 4]) -> (i32, i32) {
	let mut min_x = f32::MAX;
	let mut min_y = f32::MAX;
	let mut max_x = f32::MIN;
	let mut max_y = f32::MIN;

	for point in points {
		if point[0] < min_x {
			min_x = point[0];
		}
		if point[0] > max_x {
			max_x = point[0];
		}
		if point[1] < min_y {
			min_y = point[1];
		}
		if point[1] > max_y {
			max_y = point[1];
		}
	}
	let width = max_x.abs() - min_x.abs();
	let height = max_y.abs() - min_y.abs();
	(width.ceil().abs() as i32, height.ceil().abs() as i32)
}

enum SpriteOrientation {
	Normal,
	XInvert,
	YInvert,
	XAndYInvert
}

fn get_sprite_orientation(final_coord: &[f32; 8]) -> SpriteOrientation {
	let mut x_invert = false;
	let mut y_invert = false;
	if final_coord[0] < final_coord[6] {
		x_invert = true;
	}
	if final_coord[1] < final_coord[3] {
		y_invert = true;
	}
	if x_invert && y_invert {
		return SpriteOrientation::XAndYInvert;
	}
	else if x_invert {
		return SpriteOrientation::XInvert;
	}
	else if y_invert {
		return SpriteOrientation::YInvert;
	}
	SpriteOrientation::Normal
}

fn get_placement_coords(min_x: &f32, max_y: &f32, final_coord: &[f32; 8],  sprite_orientation: &SpriteOrientation) -> (i64, i64) {
	let [
		top_right,
		_bottom_right,
		bottom_left,
		top_left
	] = get_sprite_points(final_coord, sprite_orientation);

	let x: f32;
	let y: f32;
	if bottom_left[0] < top_left[0] {
		x = bottom_left[0];
	}
	else {
		x = top_left[0];
	}
	if top_left[1] > top_right[1] {
		y = top_left[1];
	}
	else {
		y = top_right[1];
	}
	((x - min_x).round() as i64, (max_y - y).round() as i64)
}

// Return the 4 points of the sprite in this order: "Top left", "Bottom right", "Bottom left", "Top left"
fn get_sprite_points(final_coord: &[f32; 8], sprite_orientation: &SpriteOrientation) -> [[f32; 2]; 4] {
	let top_right;
	let bottom_right;
	let bottom_left;
	let top_left;

	match sprite_orientation {
		SpriteOrientation::Normal => {
			top_right = [final_coord[0], final_coord[1]];
			bottom_right = [final_coord[2], final_coord[3]];
			bottom_left = [final_coord[4], final_coord[5]];
			top_left = [final_coord[6], final_coord[7]];
		},
		SpriteOrientation::XInvert => {
			top_right = [final_coord[6], final_coord[7]];
			bottom_right = [final_coord[4], final_coord[5]];
			bottom_left = [final_coord[2], final_coord[3]];
			top_left = [final_coord[0], final_coord[1]];
		}
		SpriteOrientation::YInvert => {
			top_right = [final_coord[2], final_coord[3]];
			bottom_right = [final_coord[0], final_coord[1]];
			bottom_left = [final_coord[6], final_coord[7]];
			top_left = [final_coord[4], final_coord[5]];
		},
		SpriteOrientation::XAndYInvert => {
			top_right = [final_coord[4], final_coord[5]];
			bottom_right = [final_coord[6], final_coord[7]];
			bottom_left = [final_coord[0], final_coord[1]];
			top_left = [final_coord[2], final_coord[3]];
		}
	}
	[top_right, bottom_right, bottom_left, top_left]
}

pub fn create_perspective_transform_matrix(pts_src: &[[f32; 2]; 4], pts_dst: &[[f32; 2]; 4]) -> Result<[f32; 9]> {
	let mut a: [[f64; 8]; 8] = [[0.; 8]; 8];
	let mut b: [f64; 9] = [0.; 9];

	for i in 0..4 {
		a[i][0] = pts_src[i][0] as f64;
		a[i + 4][3] = pts_src[i][0] as f64;

		a[i][1] = pts_src[i][1] as f64;
		a[i + 4][4] = pts_src[i][1] as f64;

		a[i][2] = 1.;
		a[i + 4][5] = 1.;
		
		a[i][3] = 0.;
		a[i][4] = 0.;
		a[i][5] = 0.;
		a[i + 4][0] = 0.;
		a[i + 4][1] = 0.;
		a[i + 4][2] = 0.;

		a[i][6] = -pts_src[i][0] as f64 * pts_dst[i][0] as f64;
		a[i][7] = -pts_src[i][1] as f64 * pts_dst[i][0] as f64;

		a[i + 4][6] = -pts_src[i][0] as f64 * pts_dst[i][1] as f64;
		a[i + 4][7] = -pts_src[i][1] as f64 * pts_dst[i][1] as f64;
		b[i] = pts_dst[i][0] as f64;
		b[i + 4] = pts_dst[i][1] as f64;
	}

	let m = 8;
	let nb = 1;

	const EPS: f64 = 2.2204460492503131e-016 * 100.;
	let mut k;

	for i in 0..m {
		k = i;

		for j in i + 1..m {
			if a[j][i].abs() > a[k][i].abs() {
				k = j;
			}
		}

		if a[k][i].abs() < EPS {
			return Err(anyhow!("Could not create matrix"));
		}

		if k != i {
			for j in i..m {
				let tmp = a[i][j];
				a[i][j] = a[k][j];
				a[k][j] = tmp;
			}
			for j in 0..nb {
				let tmp = b[i + j];
				b[i + j] = b[k + j];
				b[k + j] = tmp;
			}
		}

		let d = -1. / a[i][i];

		for j in i + 1..m {
			let alpha = a[j][i] * d;
			for k in i + 1..m {
				a[j][k] += alpha * a[i][k];
			}
			for k in 0..nb {
				b[j + k] += alpha * b[i + k];
			}
		}
	}

	for i in (0..m).rev() {
		for j in 0..nb {
			let mut s = b[i + j];
			for k in i + 1..m {
				s -= a[i][k] * b[k + j];
			}
			b[i + j] = s / a[i][i];
		}
	}

	b[8] = 1.;

	let mut result: [f32; 9] = [0.; 9];
	for i in 0..9 {
		result[i] = b[i] as f32;
	}

	Ok(result)
}

pub fn get_result_min_max_coord(coords: &Vec<SpriteCoord>) -> (f32, f32) {
	let mut min_x = f32::MAX;
	let mut max_y = f32::MIN;

	for coord in coords {
		let [x, y, x2, y2, x3, y3, x4, y4] = &coord.result;
		if *x < min_x {
			min_x = *x;
		}
		if *x2 < min_x {
			min_x = *x2;
		}
		if *x3 < min_x {
			min_x = *x3;
		}
		if *x4 < min_x {
			min_x = *x4;
		}

		if *y > max_y {
			max_y = *y;
		}
		if *y2 > max_y {
			max_y = *y2;
		}
		if *y3 > max_y {
			max_y = *y3;
		}
		if *y4 > max_y {
			max_y = *y4;
		}
	}

	(min_x, max_y)
}

pub fn build_and_tint_altas(
	result: &mut RgbaImage,
	position: Vec2,
	atlas: &RgbaImage,
	atlas_2: &Option<RgbaImage>,
	coords: &Vec<SpriteCoord>,
	colors: &Vec<[f32; 4]>,
	min_x: f32,
	max_y: f32,
	blend_modes: (BlendModes, BlendModes),
	is_anm_flip: bool
) {
	for i in 0..coords.len() {
		let color = &colors[i];

		let atlas_coord = &coords.get(i).unwrap().atlas;
		let mut final_coord = coords.get(i).unwrap().result.clone();

		if is_anm_flip {
			final_coord = [
				-final_coord[0], final_coord[1],
				-final_coord[2], final_coord[3],
				-final_coord[4], final_coord[5],
				-final_coord[6], final_coord[7]
			];
		}

		let [top, left, bottom, right] = atlas_coord;

		let altas_width;
		let atlas_height;
		if coords.get(i).unwrap().is_root_atlas {
			altas_width = atlas.width() as f32;
			atlas_height = atlas.height() as f32;
		}
		else {
			altas_width = atlas_2.as_ref().unwrap().width() as f32;
			atlas_height = atlas_2.as_ref().unwrap().height() as f32;
		}

		let x =
			if *left < 0. { (altas_width + left * altas_width).round() as u32 }
			else { (left * altas_width).round() as u32 };
		let y =
			if *top < 0. { (atlas_height + top * atlas_height).round() as u32 }
			else { (top * atlas_height).round() as u32 };
		let width =
			if *right < 0. { (altas_width + right * altas_width - x as f32).round() as u32 }
			else { (right * altas_width - x as f32).round() as u32 };
		let height =
			if *bottom < 0. { (atlas_height + bottom * atlas_height - y as f32).round() as u32 }
			else { (bottom * atlas_height - y as f32).round() as u32 };

		let mut crop;
		if coords.get(i).unwrap().is_root_atlas {
			crop = imageops::crop(&mut atlas.clone(), x, y, width, height).to_image();
		}
		else {
			crop = imageops::crop(&mut atlas_2.as_ref().unwrap().clone(), x, y, width, height).to_image();
		}

		let sprite_orientation = get_sprite_orientation(&final_coord);

		match sprite_orientation {
			SpriteOrientation::Normal => imageops::rotate180_in_place(&mut crop),
			SpriteOrientation::XInvert => imageops::flip_vertical_in_place(&mut crop),
			SpriteOrientation::YInvert => imageops::flip_horizontal_in_place(&mut crop),
			SpriteOrientation::XAndYInvert => {}
		}

		let pts_src = [
			[0., 0.],
			[crop.width() as f32, 0.],
			[crop.width() as f32, crop.height() as f32],
			[0., crop.height() as f32]
		];

		let pts_dst = get_dest_pts(&final_coord, &sprite_orientation);

		let (trans_width, trans_height) = get_transformed_size(&pts_dst);
		
		let matrix_res = create_perspective_transform_matrix(&pts_src, &pts_dst);
		if let Err(_) = matrix_res {
			continue;
		}
		let matrix = matrix_res.unwrap();

		let projection = imageproc::geometric_transformations::Projection::from_matrix(matrix).unwrap();
		let mut tmp_img = RgbaImage::new(trans_width as u32, trans_height as u32);
		imageproc::geometric_transformations::warp_into(
			&crop,
			&projection,
			imageproc::geometric_transformations::Interpolation::Bilinear,
			Rgba([0, 0, 0, 0]),
			&mut tmp_img
		);

		crop = tmp_img;

		for Rgba([r, g, b, a]) in crop.pixels_mut() {
			// Apply color tint
			if *a > 0 {
				*r = (color[0] * *r as f32) as u8;
				*g = (color[1] * *g as f32) as u8;
				*b = (color[2] * *b as f32) as u8;
				*a = (color[3] * *a as f32) as u8;
			}
		}
		let (mut final_x, mut final_y) = get_placement_coords(&min_x, &max_y, &final_coord, &sprite_orientation);
		final_x += position.x as i64;
		final_y += position.y as i64;
		custom_imageops::custom_overlay(result, &crop, final_x, final_y, &blend_modes.0, &blend_modes.1);
	}
}