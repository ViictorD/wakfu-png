use std::{collections::HashMap, path::PathBuf};

use glam::Vec2;
use image::{RgbaImage, Rgba};

use super::{Paper, anm_coords::FullMapCoord};

use crate::{assets::build_atlas::{create_perspective_transform_matrix, build_and_tint_altas}, anm::processing::anm_instance::{AnmInstance, SpriteCoord}, custom_lib::custom_imageops::color::BlendModes};

pub struct RenderPaperData {
	pub atlas: RgbaImage,
	pub atlas_2: Option<RgbaImage>,
	pub anm_instance: AnmInstance,
	pub id: i32,
	pub _start_coords: Vec2,
	pub _end_coords: Vec2,
}

pub fn render_papers(papers: Paper, data: HashMap<i32, Vec<RenderPaperData>>) {
	let full_coords = FullMapCoord::new();

	for (id, paper_group) in papers.maps {
		if !full_coords.coords.contains_key(&id) {
			continue ;
		}
		let mut result = HashMap::with_capacity(1);
		result.insert(0, transform_bg(&paper_group.texture));
		let width = result.get(&0).unwrap().width() as f32;
		let height = result.get(&0).unwrap().height() as f32;
		let ratio_x = width / 100.;
		let ratio_y = height / 2. / 100.;
		for paper_data in data.get(&id).unwrap() {
			let coords = paper_data.anm_instance.coords.clone();
			let colors = paper_data.anm_instance.colors.clone();
	
			let (min_x, min_y, max_x, max_y) = get_result_min_max_coord(&coords);
			let position_coord = full_coords.get_coord(&id, &paper_data.id);
			if position_coord.is_err() {
				continue ;
			}

			let mut sprite_position = iso_to_screen(
				position_coord.as_ref().unwrap().x as f32,
				position_coord.as_ref().unwrap().y as f32,
				ratio_x,
				ratio_y,
				height
			);
			sprite_position.y -= height / 4.;
			let origin = Vec2::new((min_x.abs() + max_x.abs()) / 2., (min_y.abs() + max_y.abs()) / 2.);
			let position = sprite_position - origin;
			build_and_tint_altas(
				&mut result,
				position,
				&paper_data.atlas,
				&paper_data.atlas_2,
				&coords,
				&colors,
				min_x,
				max_y,
				(BlendModes::One, BlendModes::InvSrcAlpha),
				paper_data.anm_instance.flip_animation
			);
		}
		let output_path = PathBuf::from(format!("./output/{id}/outdoor/{id}.png"));
		if !output_path.exists() {
			std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
		}
		result.get(&0).unwrap().save_with_format(output_path, image::ImageFormat::Png).unwrap();
	}
}

fn iso_to_screen(x: f32, y: f32, ratio_x: f32, ratio_y: f32, px_height: f32) -> Vec2 {
	let nb_width = 100.;
	let nb_height = 100.;
	let res_x = (ratio_x * x / 2.) + (nb_height * ratio_x / 2.) - (y * ratio_x / 2.);
	let res_y = px_height - (((nb_height - y - 1.) * ratio_y / 2.) + (nb_width * ratio_y / 2.) - (x * ratio_y / 2.));
	Vec2::new(res_x, res_y)
}

fn get_result_min_max_coord(coords: &Vec<SpriteCoord>) -> (f32, f32, f32, f32) {
	let mut min_x = f32::MAX;
	let mut min_y = f32::MAX;
	let mut max_x = f32::MIN;
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

		if *y < min_y {
			min_y = *y;
		}
		if *y2 < min_y {
			min_y = *y2;
		}
		if *y3 < min_y {
			min_y = *y3;
		}
		if *y4 < min_y {
			min_y = *y4;
		}

		if *x > max_x {
			max_x = *x;
		}
		if *x2 > max_x {
			max_x = *x2;
		}
		if *x3 > max_x {
			max_x = *x3;
		}
		if *x4 > max_x {
			max_x = *x4;
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

	(min_x, min_y, max_x, max_y)
}

fn transform_bg(image: &RgbaImage) -> RgbaImage{
	let width = image.width() as f32;
	let height = image.height() as f32;
	let scaled_width = width * 2.84;
	let scaled_height= height * 2.84;
	let height_quarter = scaled_height / 4.;

	let pts_src = [
		[0., 0.],
		[width, 0.],
		[width, height],
		[0., height]
	];

	let pts_dst = [
		[scaled_width / 2., height_quarter],
		[scaled_width, scaled_height / 2.],
		[scaled_width / 2., height_quarter * 3.],
		[0., scaled_height / 2.]
	];

	let matrix = create_perspective_transform_matrix(&pts_src, &pts_dst).unwrap();
	let projection = imageproc::geometric_transformations::Projection::from_matrix(matrix).unwrap();
	let mut result = RgbaImage::new(scaled_width.ceil() as u32, scaled_height.ceil() as u32);
	imageproc::geometric_transformations::warp_into(
		&image,
		&projection,
		imageproc::geometric_transformations::Interpolation::Bilinear,
		Rgba([0, 0, 0, 0]),
		&mut result
	);
	result
}
