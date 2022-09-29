use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use assets::gfx::Gfx;
use assets::tgam::TgamLoader;
use glam::{const_vec2, Vec2};
use image::{RgbaImage, imageops};
use itertools::Itertools;
use map::color::Color;
use map::environment::EnvironmentChunk;
use map::{element::ElementLibrary};
use map::Map;
use map::sprite::{MapSprite, AnmSprite, DynamicSprite};
use pico_args::Arguments;
use std::collections::HashMap;
use map::{CELL_WIDTH, CELL_HEIGHT};
use bdata::binary_document::{InteractiveElementModelBinaryData};

use crate::bdata::binary_document::BinaryDocument;
use crate::map::binar_serial_part::BinarSerialPartsEnum;

mod assets;
mod map;
mod bdata;
mod utils;
mod anm;

const FLIP_Y: Vec2 = const_vec2!([1., -1.]);

fn convert_interactive_as_sprite(env: &EnvironmentChunk, iem: &HashMap<i32, InteractiveElementModelBinaryData>) -> Vec<MapSprite>{
	let mut interactive_sprites: Vec<MapSprite> = Vec::new();
	for chunk in env.get_chunks() {
		for interactive in chunk.get_interactive_elements() {
			for interactive_data in interactive.data.get_data() {
				if let BinarSerialPartsEnum::SpecificDataPart(specific_data_part) = interactive_data {
					if let Some(view) = interactive.views.get(0) {
						let bin_data = iem.get(view).unwrap();
						let sprite = MapSprite {
							cell_x: specific_data_part.x,
							cell_y: specific_data_part.y,
							cell_z: specific_data_part.z,
							height: 0,
							altitude_order: 100,
							tag: 0,
							element_id: -1,
							group_key: 0,
							group_id: 0,
							layer: 0,
							color: Color::rgb_linear(1.0, 1.0, 1.0), // Change this to bdata color
							anm_sprite: Some(AnmSprite::new(
								bin_data.gfx,
								specific_data_part.direction,
								specific_data_part.activation_pattern,
								specific_data_part.state
							)),
							dyn_sprite: None
						};
						interactive_sprites.push(sprite);
					}
				}
			}
		}
	}
	interactive_sprites
}

fn convert_dynamic_element_as_sprite(env: &EnvironmentChunk) -> Vec<MapSprite> {
	let mut dynamic_sprites: Vec<MapSprite> = Vec::new();
	for chunk in env.get_chunks() {
		for dynamic_element in chunk.get_dynamic_elements() {
			if dynamic_element.dynamic_type == 1 {
				let sprite = MapSprite {
					cell_x: dynamic_element.coord.x as i32 + chunk.x as i32 * 18,
					cell_y: dynamic_element.coord.y as i32 + chunk.y as i32 * 18,
					cell_z: dynamic_element.coord.z as i16,
					height: 0,
					altitude_order: 50,
					tag: 0,
					element_id: -1,
					group_key: 0,
					group_id: 0,
					layer: 0,
					color: Color::rgb_linear(1.0, 1.0, 1.0),
					anm_sprite: None,
					dyn_sprite: Some(DynamicSprite::new(
						dynamic_element.gfx_id,
						dynamic_element.direction,
						dynamic_element.base_animation.clone()
					))
				};
				dynamic_sprites.push(sprite);
			}
		}
	}
	dynamic_sprites
}

fn get_margin_and_png_size(lib: &ElementLibrary, sorted_sprite: &HashMap<i64, &MapSprite>) -> (Vec2, Vec2) {
	let mut min = Vec2::new(f32::MAX, f32::MAX);
	let mut max = Vec2::new(f32::MIN, f32::MIN);

	for hashcode in sorted_sprite.keys().sorted() {
		let sprite = sorted_sprite[hashcode];
		if sprite.element_id == -1 {
			continue ;
		}
		match lib.get(sprite.element_id) {
			Some(element) => {
				let vec2 = sprite.screen_position() * FLIP_Y - element.origin();
		
				if vec2.x < min.x {
					min.x = vec2.x;
				}
				if vec2.y < min.y {
					min.y = vec2.y;
				}
				if vec2.x > max.x {
					max.x = vec2.x;
				}
				if vec2.y > max.y {
					max.y = vec2.y;
				}
			},
			None => println!("Element not found id: {}", sprite.element_id)
		}
	}

	// Calculate the amount to add to remove negative coords
	let margin = Vec2::new(
		if min.x < 0. { min.x } else { 0. },
		if min.y < 0. { min.y } else { 0. }
	);

	(
		margin,
		Vec2::new(
			max.x - margin.x + CELL_WIDTH,
			max.y - margin.y + CELL_HEIGHT
		)
	)

}

fn create_png(
	map_id: i32,
	map: Map,
	lib: ElementLibrary,
	mut gfx: Gfx,
	env: EnvironmentChunk,
	iem: HashMap<i32, InteractiveElementModelBinaryData>
) {
	let mut sorted_sprite: HashMap<i64, &MapSprite> = HashMap::new();
	
	// Base map
	for chunk in map.chunks() {
		for sprite in chunk.sprites() {
			sorted_sprite.insert(sprite.hashcode(), sprite);
		}
	}

	// Interactive elements
	let interactive_sprites: Vec<MapSprite> = convert_interactive_as_sprite(&env, &iem);
	for sprite in interactive_sprites.iter() {
		sorted_sprite.insert(sprite.hashcode(), sprite);
	}

	// Dynamic elements
	let dynamic_sprites: Vec<MapSprite> = convert_dynamic_element_as_sprite(&env);
	for sprite in dynamic_sprites.iter() {
		sorted_sprite.insert(sprite.hashcode(), sprite);
	}

	let (margin, image_size) = get_margin_and_png_size(&lib, &sorted_sprite);

	let mut image = RgbaImage::new(
		image_size.x as u32,
		image_size.y as u32
	);


	for hashcode in sorted_sprite.keys().sorted() {
		let sprite = sorted_sprite[hashcode];

		// Handle interactive animated sprite
		if let Some(anm_sprite) = &sprite.anm_sprite {
			let res = gfx.load_interactive_texture_as_rgba_image(anm_sprite);
			if let Err(err) = res {
				println!("{}", err);
				continue;
			}
			let (sprite_image, origin) = res.unwrap();
			let vec2 = sprite.screen_position() * FLIP_Y - origin;
			imageops::overlay(
				&mut image,
				&sprite_image,
				(vec2.x - margin.x) as i64,
				(vec2.y - margin.y) as i64
			);
			continue;
		}
		// Handle dynamic animated sprite
		if let Some(dyn_sprite) = &sprite.dyn_sprite {
			let res = gfx.load_dynamic_texture_as_rgba_image(dyn_sprite);
			if let Err(err) = res {
				println!("{}", err);
				continue;
			}
			let (sprite_image, origin) = res.unwrap();
			let vec2 = sprite.screen_position() * FLIP_Y - origin;
			imageops::overlay(
				&mut image,
				&sprite_image,
				(vec2.x - margin.x) as i64,
				(vec2.y - margin.y) as i64
			);
			continue;
		}
		// Handle base map sprite
		if let Some(element) = lib.get(sprite.element_id) {
			// Skip the debug animated cell
			if element.texture_id == 19067 {
				continue;
			}
			
			let res = gfx.load_texture_as_rgba_image(&element, &sprite);
			if let Err(err) = res {
				println!("{}", err);
				continue;
			}
			let vec2 = sprite.screen_position() * FLIP_Y - element.origin();
			imageops::overlay(
				&mut image,
				&res.unwrap(),
				(vec2.x - margin.x) as i64,
				(vec2.y - margin.y) as i64
			);
		}
	}

	println!("Saving into {}.png...", map_id);
	if let Err(err) = image.save_with_format(format!("./{}.png", map_id), image::ImageFormat::Png){
		println!("Failed to save image: {}", err);
	}
}

fn main() -> Result<()> {
	let mut pargs = Arguments::from_env();

	let game_path: PathBuf = pargs.value_from_str("--path")?;
	let map_id: i32 = pargs.value_from_str("--map")?;

	let maps_path = game_path.join("contents").join("maps");
	let gfx_path = maps_path.join("gfx.jar");
	let animation_path = game_path
		.join("contents")
		.join("animations");
	let interactive_path = animation_path
		.join("interactives")
		.join("interactives.jar");
	let dynamic_path = animation_path
		.join("dynamics")
		.join("dynamics.jar");
	let map_path = maps_path.join("gfx").join(format!("{}.jar", map_id));
	let lib_path = maps_path.join("data.jar");
	let env_path = maps_path.join("env").join(format!("{}.jar", map_id));
	let iem_path = game_path.join("contents").join("bdata").join("34.jar");

	let map = Map::load(File::open(map_path)?)?;
	let lib = ElementLibrary::load(File::open(lib_path)?)?;
	let gfx = Gfx::load(
		File::open(gfx_path)?,
		File::open(interactive_path)?,
		File::open(dynamic_path)?,
	);
	let env = EnvironmentChunk::load(File::open(env_path)?)?;
	let mut iem = BinaryDocument::load(File::open(iem_path)?)?;
	let iem_data = iem.read_iem()?;

	create_png(map_id, map, lib, gfx, env, iem_data);

	Ok(())
}