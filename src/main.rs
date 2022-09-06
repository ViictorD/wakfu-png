use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use assets::gfx::Gfx;
use assets::tgam::TgamLoader;
use glam::{const_vec2, Vec2};
use image::{RgbaImage, imageops, Rgba};
use itertools::Itertools;
use map::element::ElementLibrary;
use map::Map;
use map::sprite::MapSprite;
use pico_args::Arguments;
use std::collections::HashMap;
use map::{CELL_WIDTH, CELL_HEIGHT};

mod assets;
mod map;

const FLIP_Y: Vec2 = const_vec2!([1., -1.]);

fn get_margin_and_png_size(lib: &ElementLibrary, sorted_sprite: &HashMap<i64, &MapSprite>) -> (Vec2, Vec2) {
	let mut min = Vec2::new(f32::MAX, f32::MAX);
	let mut max = Vec2::new(f32::MIN, f32::MIN);

	for hashcode in sorted_sprite.keys().sorted() {
		let sprite = sorted_sprite[hashcode];
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

fn create_png(map_id: i32, map: Map, lib: ElementLibrary, mut gfx: Gfx) {
	let mut sorted_sprite: HashMap<i64, &MapSprite> = HashMap::new();
	
	// Apply hashcode to each sprite, that can be sorted to get the correct order
	// on how to overlay sprite into eachother
	for chunk in map.chunks() {
		for sprite in chunk.sprites() {
			sorted_sprite.insert(sprite.hashcode(), sprite);
		}
	}

	let (margin, image_size) = get_margin_and_png_size(&lib, &sorted_sprite);

	let mut image = RgbaImage::new(
		image_size.x as u32,
		image_size.y as u32
	);

	for hashcode in sorted_sprite.keys().sorted() {
		let sprite = sorted_sprite[hashcode];

		if let Some(element) = lib.get(sprite.element_id) {
			match gfx.load_texture_as_rgba_image(&element, element.texture_id) {
				Ok(mut sprite_image) => {
					if element.flags.is_flip() {
						sprite_image = imageops::flip_horizontal(&sprite_image);
					}
		
					for Rgba([r, g, b, a]) in sprite_image.pixels_mut() {
						// Apply premultiply alpha
						if *a > 0 && *a < 255 {
							*r = (*r as f32 * (1. / *a as f32) * 255.) as u8;
							*g = (*g as f32 * (1. / *a as f32) * 255.) as u8;
							*b = (*b as f32 * (1. / *a as f32) * 255.) as u8;
						}
						// Apply color tint
						if *a > 0 {
							*r = (sprite.color.r() * *r as f32) as u8;
							*g = (sprite.color.g() * *g as f32) as u8;
							*b = (sprite.color.b() * *b as f32) as u8;
							*a = (sprite.color.a() * *a as f32) as u8;
						}
					}
		
					let vec2 = sprite.screen_position() * FLIP_Y - element.origin();
					imageops::overlay(&mut image, &sprite_image, (vec2.x - margin.x) as i64, (vec2.y - margin.y) as i64);
				},
				Err(err) => println!("{}", err)
			}
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
	let map_path = maps_path.join("gfx").join(format!("{}.jar", map_id));
	let lib_path = maps_path.join("data.jar");

	let map = Map::load(File::open(map_path)?)?;
	let lib = ElementLibrary::load(File::open(lib_path)?)?;
	let gfx = Gfx::load(File::open(gfx_path)?);

	create_png(map_id, map, lib, gfx);

	Ok(())
}
